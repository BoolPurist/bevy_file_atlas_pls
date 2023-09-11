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
    frame_seq_duration: Timer,
    all_frames: AnimationKey,
    current_state: AnimationKey,
    // Box because only an individual component holds it. It is never cloned.
    // Only there for the next frame so the system knows which one is the new state.
    next_state: Option<Box<str>>,
    reset_state: bool,
}

impl AnimationComp {
    pub fn new(
        all_frames: AnimationKey,
        start_state: AnimationKey,
        repos: &AllAnimationResource,
    ) -> Result<Self, NotFoundError> {
        let duration_secs = Self::get_animation_seq(repos, &all_frames, &start_state)?
            .1
            .time_per_frame();
        let frame_seq_duration = Self::new_time(duration_secs);
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

    pub fn change_state(&mut self, key: &str) {
        if key != self.current_state.as_ref() {
            self.set_state(key);
        }
    }

    pub fn reset_current_state(&mut self) {
        self.reset_state = true;
    }

    pub fn set_state(&mut self, key: &str) {
        self.next_state = Some(Box::from(key));
    }

    pub(crate) fn start_index(
        &self,
        repos: &AllAnimationResource,
    ) -> KeyLookUpResult<AnimationIndex> {
        let animation = self.get_current_seq(repos)?;
        Ok(animation.start())
    }

    pub(crate) fn udpate(
        &mut self,
        altlas_sprite: &mut TextureAtlasSprite,
        time: &Time,
        repos: &AllAnimationResource,
    ) -> KeyLookUpResult {
        if self.frame_seq_duration.tick(time.delta()).just_finished() {
            let next = altlas_sprite.index + 1;
            let current_animation = self.get_current_seq(repos)?;
            self.frame_seq_duration = Self::new_time(current_animation.time());
            altlas_sprite.index = if next > current_animation.end() {
                current_animation.start()
            } else {
                next
            }
        };
        Ok(())
    }

    pub(crate) fn do_pending_change(
        &mut self,
        to_adjust: &mut TextureAtlasSprite,
        respo: &AllAnimationResource,
    ) -> KeyLookUpResult {
        if let Some(new) = self.next_state.take() {
            let (new_time, start) = {
                let (key, new_animation) = Self::get_animation_seq(respo, &self.all_frames, &new)?;
                self.current_state = key;
                (new_animation.time_per_frame(), new_animation.start())
            };
            self.frame_seq_duration = Self::new_time(new_time);
            to_adjust.index = start;
        }
        Ok(())
    }

    pub(crate) fn do_pending_reset(
        &mut self,
        to_adjust: &mut TextureAtlasSprite,
        respo: &AllAnimationResource,
    ) -> KeyLookUpResult {
        if self.reset_state {
            self.reset_state = false;
            let (_, current_animation) =
                Self::get_animation_seq(respo, &self.all_frames, &self.current_state)?;
            to_adjust.index = current_animation.start();
            self.frame_seq_duration.reset();
        }
        Ok(())
    }

    fn get_animation_seq(
        repos: &AllAnimationResource,
        frames: &str,
        current_state: &str,
    ) -> KeyLookUpResult<(AnimationKey, ImmutableAnimationFrames)> {
        repos
            .animation_under(frames)?
            .key_and_frames_under(&current_state)
    }

    fn new_time(time: f32) -> Timer {
        Timer::new(Duration::from_secs_f32(time), TimerMode::Repeating)
    }

    fn get_current_seq(
        &self,
        repos: &AllAnimationResource,
    ) -> KeyLookUpResult<ImmutableAnimationFrames> {
        Self::get_animation_seq(repos, &self.all_frames, &self.current_state)
            .map(|(_, value)| value)
    }
}
