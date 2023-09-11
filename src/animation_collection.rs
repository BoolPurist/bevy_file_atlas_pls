use std::sync::Arc;

use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::{
    animation_altlas::AnimationAltlas,
    animation_error::NotFoundError,
    animation_frames::AnimationFrames,
    animation_key::AnimationKey,
    types::{
        AnimationDuration, AnimationIndex, AnimationSequence, ImmutableAnimationFrames,
        KeyLookUpResult,
    },
};

#[derive(Debug)]
pub struct AnimationCollection {
    meta: AnimationAltlas,
    start_state: AnimationKey,
    pub frames: AnimationSequence,
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

    pub fn start_state(&self) -> AnimationKey {
        self.start_state.clone()
    }

    pub(crate) fn key_and_frames_under(
        &self,
        key: &str,
    ) -> KeyLookUpResult<(AnimationKey, ImmutableAnimationFrames)> {
        self.frames()
            .get_key_value(key)
            .map(|(key, value)| (key.clone(), value.clone()))
            .ok_or_else(|| NotFoundError::SingleAnimation(key.into()))
    }
}
#[derive(Debug)]
pub struct AnimationCollectionBuilder {
    meta: AnimationAltlas,
    frames: AnimationSequenceBuilder,
}

impl AnimationCollectionBuilder {
    pub fn new(meta: AnimationAltlas) -> Self {
        Self {
            meta,
            frames: Default::default(),
        }
    }
    pub fn add_animation(mut self, key: &str, frames: AnimationFrames) -> Self {
        self.frames = self.frames.add_animation(key, frames);
        self
    }
    pub fn add_row_ani(mut self, key: &str, row: AnimationIndex, time: AnimationDuration) -> Self {
        self.frames = self.frames.add_row_ani(key, row, time, &self.meta);
        self
    }
    pub fn build(self, start_state: &str) -> AnimationCollection {
        AnimationCollection {
            meta: self.meta,
            start_state: AnimationKey::new(start_state),
            frames: self.frames.build(),
        }
    }
}

#[derive(Default, Debug)]
pub struct AnimationSequenceBuilder(AnimationSequence);

impl AnimationSequenceBuilder {
    pub fn add_animation(mut self, key: &str, frames: AnimationFrames) -> Self {
        let ani_key = AnimationKey::new(key);
        self.0.insert(ani_key, Arc::new(frames));
        self
    }
    pub fn add_row_ani(
        mut self,
        key: &str,
        row: AnimationIndex,
        time: AnimationDuration,
        meta: &AnimationAltlas,
    ) -> Self {
        let ani_key = AnimationKey::new(key);
        let animation_frames = AnimationFrames::from_row(row, time, meta.data().columns()).unwrap();
        self.0.insert(ani_key, Arc::new(animation_frames));
        self
    }
    pub fn build(self) -> AnimationSequence {
        self.0
    }
}
