use bevy::prelude::*;
#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

use crate::{
    animation_error::NotFoundError,
    animation_key::AnimationKey,
    prelude::{AllAnimationResource, ImmutableAnimationFrames},
    types::{AnimationDuration, AnimationIndex, KeyLookUpResult},
};

#[derive(Component, Debug, Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct AnimationComp {
    #[reflect(ignore)]
    pub(crate) sequence: AnimationKey,
    #[reflect(ignore)]
    pub(crate) current_state: AnimationKey,
    pub(crate) reset_state: bool,
    pub(crate) duration_for_animation: Timer,
    // Box because only an individual component holds it. It is never cloned.
    // Only there for the next frame so the system knows which one is the new state.
    #[reflect(ignore)]
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
            duration_for_animation: frame_seq_duration,
            sequence: all_frames,
            current_state: start_state,
            next_state: None,
            reset_state: false,
        })
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
        get_animation_seq(repos, &self.sequence, &self.current_state).map(|(_, value)| value)
    }

    pub fn sequence(&self) -> &str {
        self.sequence.as_ref()
    }

    pub fn current_state(&self) -> &str {
        self.current_state.as_ref()
    }

    pub fn duration_for_animation(&self) -> Timer {
        self.duration_for_animation.clone()
    }

    pub fn get_reset_state(&self) -> bool {
        self.reset_state
    }
}

pub(crate) fn new_reapting_time(time: AnimationDuration) -> Timer {
    Timer::new(time, TimerMode::Repeating)
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
