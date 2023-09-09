use bevy::prelude::*;
use bevy::utils::Duration;

use crate::{
    animation_key::AnimationKey,
    prelude::{AllAnimationResource, ImmutableAnimationFrames},
    types::AnimationIndex,
};

#[derive(Component, Debug)]
pub struct AnimationComp {
    frame_seq_duration: Timer,
    all_frames: AnimationKey,
    current_state: AnimationKey,
    next_state: Option<AnimationKey>,
}

impl AnimationComp {
    pub fn new(
        all_frames: AnimationKey,
        start_state: AnimationKey,
        repos: &AllAnimationResource,
    ) -> Self {
        let duration_secs =
            Self::get_animation_seq(repos, &all_frames, &start_state).time_per_frame();
        let frame_seq_duration = Self::new_time(duration_secs);
        Self {
            frame_seq_duration,
            all_frames,
            current_state: start_state,
            next_state: None,
        }
    }

    fn get_animation_seq(
        repos: &AllAnimationResource,
        frames: &AnimationKey,
        current_state: &AnimationKey,
    ) -> ImmutableAnimationFrames {
        repos
            .animation_under(frames)
            .frames()
            .get(current_state)
            .unwrap()
            .clone()
    }

    fn get_current_seq(&self, repos: &AllAnimationResource) -> ImmutableAnimationFrames {
        Self::get_animation_seq(repos, &self.all_frames, &self.current_state)
    }

    pub fn start_index(&self, repos: &AllAnimationResource) -> AnimationIndex {
        let animation = self.get_current_seq(repos);
        animation.start()
    }

    pub fn udpate(
        &mut self,
        altlas_sprite: &mut TextureAtlasSprite,
        time: &Time,
        repos: &AllAnimationResource,
    ) {
        if self.frame_seq_duration.tick(time.delta()).just_finished() {
            let next = altlas_sprite.index + 1;
            let current_animation = self.get_current_seq(repos);
            self.frame_seq_duration = Self::new_time(current_animation.time());
            altlas_sprite.index = if next > current_animation.end() {
                current_animation.start()
            } else {
                next
            }
        };
    }

    pub fn change_state(&mut self, key: &str) {
        let new_key = AnimationKey::new(key);
        self.next_state = Some(new_key);
    }

    pub fn do_pending_change(
        &mut self,
        to_adjust: &mut TextureAtlasSprite,
        respo: &AllAnimationResource,
    ) {
        if let Some(new) = self.next_state.take() {
            self.current_state = new;
            let (new_time, start) = {
                let new_animation = self.get_current_seq(respo);
                (new_animation.time_per_frame(), new_animation.start())
            };
            self.frame_seq_duration = Self::new_time(new_time);
            to_adjust.index = start;
        }
    }

    fn new_time(time: f32) -> Timer {
        Timer::new(Duration::from_secs_f32(time), TimerMode::Repeating)
    }
}
