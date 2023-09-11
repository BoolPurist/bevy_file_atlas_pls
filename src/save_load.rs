use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

use crate::{
    animation_collection::{AnimationCollection, AnimationSequenceBuilder},
    animation_frames::AnimationFrames,
    prelude::{AnimationAltlasMeta, AnimationCollectionBuilder, AnimationIndex, AnimationSequence},
    types::AnimationResult,
    utils, PosScaleFactor,
};

#[derive(Deserialize)]
pub struct FramesSerde {
    name: String,
    start_row: usize,
    start_column: Option<usize>,
    end_row: Option<usize>,
    end_column: Option<usize>,
    time_secs: Option<f32>,
}

impl FramesSerde {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn to_animation_frames(
        &self,
        sequence_meta: &AnimationAssets,
        default_ani_duration: PosScaleFactor,
    ) -> AnimationResult<AnimationFrames> {
        let time_secs = {
            let raw = self.time_secs.unwrap_or(
                sequence_meta
                    .time()
                    .unwrap_or(default_ani_duration.to_f32()),
            );
            utils::f32_to_animation_duration(raw)
        }?;

        let columns = sequence_meta.columns();
        Ok(AnimationFrames::new(
            self.start_row,
            self.start_column,
            self.end_row,
            self.end_column,
            time_secs,
            columns,
        )?)
    }
}

#[derive(Deserialize, TypeUuid, TypePath)]
#[uuid = "11da45ef-11d1-4e6e-94b5-686fd8b783d0"]
pub struct AnimationAssets {
    init_name: Option<String>,
    start_state: String,
    frames: Vec<FramesSerde>,
    time_secs: Option<f32>,
    #[serde(flatten)]
    general: AnimationAltlasMeta,
}

impl AnimationAssets {
    pub fn to_animaton_collection(
        &self,
        image: Handle<Image>,
        assets_atlas: &mut Assets<TextureAtlas>,
        default_ani_duration: PosScaleFactor,
    ) -> AnimationResult<AnimationCollection> {
        let meta = self.general.clone().build(image, assets_atlas);
        let mut collection = AnimationCollectionBuilder::new(meta);
        for (name, frames) in self.frames.iter().map(|to_split| {
            (
                to_split.name(),
                to_split.to_animation_frames(self, default_ani_duration),
            )
        }) {
            collection = collection.add_animation(name, frames?);
        }

        Ok(collection.build(&self.start_state))
    }

    pub fn to_ani_seq(
        &self,
        default_ani_duration: PosScaleFactor,
    ) -> AnimationResult<AnimationSequence> {
        let mut seq = AnimationSequenceBuilder::default();
        for (name, frames) in self.frames.iter().map(|to_split| {
            (
                to_split.name(),
                to_split.to_animation_frames(self, default_ani_duration),
            )
        }) {
            seq = seq.add_animation(name, frames?);
        }
        Ok(seq.build())
    }

    pub fn start_state(&self) -> &str {
        self.start_state.as_ref()
    }

    pub fn name(&self) -> Option<&str> {
        self.init_name.as_deref()
    }

    pub fn columns(&self) -> AnimationIndex {
        self.general.columns()
    }

    pub fn time(&self) -> Option<f32> {
        self.time_secs
    }
}
