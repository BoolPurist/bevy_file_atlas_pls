use std::sync::Arc;

use bevy::{prelude::Handle, sprite::TextureAtlas};

use crate::{
    animation_altlas::AnimationAltlas,
    animation_frames::AnimationFrames,
    animation_key::AnimationKey,
    types::{AnimationIndex, AnimationSequence},
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
    pub fn set_frames(&mut self, new_seq: AnimationSequence) {
        self.frames = new_seq;
    }

    pub fn start_state(&self) -> AnimationKey {
        self.start_state.clone()
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
    pub fn add_row_ani(mut self, key: &str, row: AnimationIndex, time: f32) -> Self {
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
        time: f32,
        meta: &AnimationAltlas,
    ) -> Self {
        let ani_key = AnimationKey::new(key);
        let animation_frames = AnimationFrames::from_row(row, time, meta.data().columns());
        self.0.insert(ani_key, Arc::new(animation_frames));
        self
    }
    pub fn build(self) -> AnimationSequence {
        self.0
    }
}
