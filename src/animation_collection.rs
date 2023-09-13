use std::sync::Arc;

use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::{
    animation_altlas::AnimationAltlas,
    animation_error::NotFoundError,
    animation_frames::AnimationFrames,
    text_like::TextLike,
    types::{
        self, AnimationDuration, AnimationIndex, AnimationSeqToBuild, AnimationSequence,
        ImmutableAnimationFrames, KeyLookUpResult,
    },
    utils,
};

#[derive(Debug)]
pub struct AnimationCollection {
    meta: AnimationAltlas,
    start_state: &'static str,
    pub frames: AnimationSequence,
}

impl std::fmt::Display for AnimationCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Start state: {}", self.start_state)?;
        writeln!(f, "{}", self.meta)?;
        for (key, seq) in self.frames.iter() {
            writeln!(f, "Frame key: {}", key)?;
            writeln!(
                f,
                "Frames: \n{}",
                utils::indent_succive(&seq.to_string(), 2)
            )?;
        }
        Ok(())
    }
}

impl AnimationCollection {
    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.meta.atlas()
    }

    pub fn frames(&self) -> &AnimationSequence {
        &self.frames
    }

    pub fn get_frames_under(&self, key: &str) -> Result<ImmutableAnimationFrames, NotFoundError> {
        self.frames
            .get(key)
            .cloned()
            .ok_or_else(|| NotFoundError::SingleAnimation(key.into()))
    }

    pub fn set_frames(&mut self, new_seq: AnimationSequence) {
        self.frames = new_seq;
    }

    pub fn start_state(&self) -> &'static str {
        self.start_state
    }

    pub(crate) fn key_and_frames_under(
        &self,
        key: &str,
    ) -> KeyLookUpResult<(&'static str, ImmutableAnimationFrames)> {
        self.frames()
            .get_key_value(key)
            .map(|(&key, value)| (key, value.clone()))
            .ok_or_else(|| NotFoundError::SingleAnimation(key.into()))
    }
}
#[derive(Debug)]
pub struct AnimationCollectionBuilder<'a> {
    meta: AnimationAltlas,
    frames: AnimationSequenceBuilder<'a>,
}

impl<'a> AnimationCollectionBuilder<'a> {
    pub fn new(meta: AnimationAltlas) -> Self {
        Self {
            meta,
            frames: Default::default(),
        }
    }
    pub fn add_animation(mut self, key: impl Into<TextLike<'a>>, frames: AnimationFrames) -> Self {
        self.frames = self.frames.add_animation(key, frames);
        self
    }
    pub fn add_row_ani(
        mut self,
        key: impl Into<TextLike<'a>>,
        row: AnimationIndex,
        time: AnimationDuration,
    ) -> Self {
        self.frames = self.frames.add_row_ani(key, row, time, &self.meta);
        self
    }
    pub fn build(self, start_state: impl Into<TextLike<'a>>) -> AnimationCollection {
        AnimationCollection {
            meta: self.meta,
            start_state: start_state.into().to_registered_name(),
            frames: self.frames.build(),
        }
    }
}

#[derive(Default, Debug)]
pub struct AnimationSequenceBuilder<'a>(AnimationSeqToBuild<'a>);

impl<'a> AnimationSequenceBuilder<'a> {
    pub fn add_animation(mut self, key: impl Into<TextLike<'a>>, frames: AnimationFrames) -> Self {
        self.0.insert(key.into(), Arc::new(frames));
        self
    }
    pub fn add_row_ani(
        mut self,
        key: impl Into<TextLike<'a>>,
        row: AnimationIndex,
        time: AnimationDuration,
        meta: &AnimationAltlas,
    ) -> Self {
        let animation_frames = AnimationFrames::from_row(row, time, meta.data().columns()).unwrap();
        self.0.insert(key.into(), Arc::new(animation_frames));
        self
    }
    pub fn build(self) -> AnimationSequence {
        types::to_build_to_ani_seq(self.0)
    }
}
