use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use crate::{
    animation_collection::{AnimationCollection, AnimationSequenceBuilder},
    animation_frames::AnimationFrames,
    prelude::{AnimationAltlasMeta, AnimationCollectionBuilder, AnimationIndex, AnimationSequence},
    types::{AnimationFrameResult, AnimationResult},
};
#[derive(Deserialize)]
pub struct FramesSerde {
    name: String,
    start_row: usize,
    start_column: Option<usize>,
    end_row: Option<usize>,
    end_column: Option<usize>,
    time_secs: f32,
}

impl FramesSerde {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn to_animation_frames(&self, columns: AnimationIndex) -> AnimationFrameResult {
        AnimationFrames::new(
            self.start_row,
            self.start_column,
            self.end_row,
            self.end_column,
            self.time_secs,
            columns,
        )
    }
}

#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid = "11da45ef-11d1-4e6e-94b5-686fd8b783d0"]
pub struct AnimationAssets {
    name: String,
    start_state: String,
    frames: Vec<FramesSerde>,
    #[serde(flatten)]
    general: AnimationAltlasMeta,
}

impl AnimationAssets {
    pub fn to_animaton_collection(
        &self,
        image: Handle<Image>,
        mut assets_atlas: &mut Assets<TextureAtlas>,
    ) -> AnimationResult<AnimationCollection> {
        let meta = self.general.clone().build(image, &mut assets_atlas);
        let mut collection = AnimationCollectionBuilder::new(meta);
        for (name, frames) in self.frames.iter().map(|to_split| {
            (
                to_split.name(),
                to_split.to_animation_frames(self.general.columns()),
            )
        }) {
            collection = collection.add_animation(name, frames?);
        }

        Ok(collection.build(&self.start_state))
    }

    pub fn to_ani_seq(&self) -> AnimationResult<AnimationSequence> {
        let mut seq = AnimationSequenceBuilder::default();
        for (name, frames) in self.frames.iter().map(|to_split| {
            (
                to_split.name(),
                to_split.to_animation_frames(self.general.columns()),
            )
        }) {
            seq = seq.add_animation(name, frames?);
        }
        Ok(seq.build())
    }

    pub fn start_state(&self) -> &str {
        self.start_state.as_ref()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
