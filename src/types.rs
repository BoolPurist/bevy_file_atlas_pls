use std::{borrow::Cow, sync::Arc};

use bevy::utils::HashMap;

use crate::{
    animation_collection::AnimationCollection,
    animation_error::{AnimationError, AnimationFrameError, NotFoundError},
    animation_frames::AnimationFrames,
    text_like::TextLike,
};

/// Most of the time it is Borrowed 'static.
/// Only if user changes the keys of the component via bevy inspector
/// then it will be a heap allocated String.
pub(crate) type AnimationReference = Cow<'static, str>;
pub type AnimationFrameResult = Result<AnimationFrames, AnimationFrameError>;
pub type AnimationResult<T = ()> = Result<T, AnimationError>;
pub type KeyLookUpResult<T = ()> = Result<T, NotFoundError>;
pub type AnimationIndex = usize;
pub type ImmutableAnimationFrames = Arc<AnimationFrames>;
pub type AnimationSequence = HashMap<&'static str, ImmutableAnimationFrames>;
pub type AnimationSeqToBuild<'a> = HashMap<TextLike<'a>, ImmutableAnimationFrames>;
pub type AnimationRepository = HashMap<&'static str, AnimationCollection>;
pub type AnimationDuration = bevy::utils::Duration;

pub fn to_build_to_ani_seq<'a>(to_convert: AnimationSeqToBuild<'a>) -> AnimationSequence {
    to_convert
        .into_iter()
        .map(|(key_to_convert, value)| (key_to_convert.to_registered_name(), value))
        .collect()
}
