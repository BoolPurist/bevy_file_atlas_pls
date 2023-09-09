use std::sync::Arc;

use bevy::utils::HashMap;

use crate::{
    animation_collection::AnimationCollection, animation_frames::AnimationFrames,
    animation_key::AnimationKey,
};

pub type AnimationIndex = usize;
pub type ImmutableAnimationFrames = Arc<AnimationFrames>;
pub type AnimationSequence = HashMap<AnimationKey, ImmutableAnimationFrames>;
pub type AnimationRepository = HashMap<AnimationKey, AnimationCollection>;
