use bevy::prelude::*;
use bevy::utils::Duration;

use crate::{
    animation_error::NotFoundError,
    animation_key::AnimationKey,
    prelude::{AllAnimationResource, ImmutableAnimationFrames},
    types::{AnimationIndex, KeyLookUpResult},
};

#[derive(Component, Debug)]
pub struct AnimationComp {
    pub all_frames: AnimationKey,
    pub current_state: AnimationKey,
    pub reset_state: bool,
    pub(crate) frame_seq_duration: Timer,
    // Box because only an individual component holds it. It is never cloned.
    // Only there for the next frame so the system knows which one is the new state.
    pub(crate) next_state: Option<Box<str>>,
}

impl AnimationComp {
    pub fn new(
        all_frames: AnimationKey,
        start_state: AnimationKey,
        repos: &AllAnimationResource,
    ) -> Result<Self, NotFoundError> {
        let duration_secs = get_animation_seq(repos, &all_frames, &start_state)?
            .1
            .time_per_frame();
        let frame_seq_duration = new_reapting_time(duration_secs);
        Ok(Self {
            frame_seq_duration,
            all_frames,
            current_state: start_state,
            next_state: None,
            reset_state: false,
        })
    }

    pub fn current_state(&self) -> &str {
        &self.current_state
    }

    pub fn set_state(&mut self, key: &str) {
        self.next_state = Some(Box::from(key));
    }

    pub fn change_state(&mut self, key: &str) {
        if key != self.current_state.as_ref() {
            self.set_state(key);
        }
    }

    pub fn reset_current_state(&mut self) {
        self.reset_state = true;
    }

    pub fn start_index(&self, repos: &AllAnimationResource) -> KeyLookUpResult<AnimationIndex> {
        let animation = self.get_current_seq(repos)?;
        Ok(animation.start())
    }

    pub fn get_current_seq(
        &self,
        repos: &AllAnimationResource,
    ) -> KeyLookUpResult<ImmutableAnimationFrames> {
        get_animation_seq(repos, &self.all_frames, &self.current_state).map(|(_, value)| value)
    }
}

pub(crate) fn new_reapting_time(time: f32) -> Timer {
    Timer::new(Duration::from_secs_f32(time), TimerMode::Repeating)
}

pub(crate) fn get_animation_seq(
    repos: &AllAnimationResource,
    frames: &str,
    current_state: &str,
) -> KeyLookUpResult<(AnimationKey, ImmutableAnimationFrames)> {
    repos
        .animation_under(frames)?
        .key_and_frames_under(&current_state)
}
