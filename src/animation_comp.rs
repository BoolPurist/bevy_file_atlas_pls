use std::borrow::Cow;

use bevy::prelude::*;
#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

use crate::{
    animation_error::NotFoundError,
    prelude::{AllAnimationResource, ImmutableAnimationFrames},
    text_like::TextLike,
    types::{AnimationDuration, AnimationIndex, AnimationReference, KeyLookUpResult},
};

#[derive(Component, Debug, Reflect)]
#[reflect(from_reflect = false)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct AnimationComp {
    pub(crate) sequence: AnimationReference,
    pub(crate) current_state: AnimationReference,
    pub(crate) reset_state: bool,
    pub(crate) duration_for_animation: Timer,
    #[reflect(ignore)]
    pub(crate) next_state: Option<AnimationReference>,
}

impl AnimationComp {
    pub fn new(
        all_frames: impl Into<Cow<'static, str>>,
        start_state: impl Into<Cow<'static, str>>,
        repos: &AllAnimationResource,
    ) -> Result<Self, NotFoundError> {
        let (all_frames, start_state) = (all_frames.into(), start_state.into());
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

    pub fn set_state<'a>(&mut self, key: impl Into<TextLike<'a>>) {
        let key = key.into();
        self.next_state = Some(key.into());
    }

    pub fn change_state<'a>(&mut self, key: impl Into<TextLike<'a>>) {
        let key = key.into();
        if key.as_str() != self.current_state.as_ref() {
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
) -> KeyLookUpResult<(&'static str, ImmutableAnimationFrames)> {
    repos
        .animation_under(frames)?
        .key_and_frames_under(&current_state)
}
