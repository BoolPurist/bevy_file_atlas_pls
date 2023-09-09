use std::sync::Arc;

use bevy::utils::HashMap;

use crate::{
    animation_collection::AnimationCollection,
    animation_error::{AnimationError, AnimationFrameError, NotFoundError},
    animation_frames::AnimationFrames,
    animation_key::AnimationKey,
};

pub type AnimationFrameResult = Result<AnimationFrames, AnimationFrameError>;
pub type AnimationResult<T> = Result<T, AnimationError>;
pub type KeyLookUpResult<T = ()> = Result<T, NotFoundError>;
pub type AnimationIndex = usize;
pub type ImmutableAnimationFrames = Arc<AnimationFrames>;
pub type AnimationSequence = HashMap<AnimationKey, ImmutableAnimationFrames>;
pub type AnimationRepository = HashMap<AnimationKey, AnimationCollection>;
