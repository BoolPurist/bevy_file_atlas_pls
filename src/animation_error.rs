use thiserror::Error;

use crate::{animation_key::AnimationKey, prelude::AnimationIndex};
#[derive(Debug, Error)]
pub enum AnimationError {
    #[error("Invalid frames for an animation:\n{0}")]
    InvalidFrames(#[from] AnimationFrameError),
    #[error("{0}")]
    NotFound(#[from] NotFoundError),
    #[error("There was no key for an animaiton sequence provided.")]
    NoSeqeunceKeyProvided,
    #[error("Key {0} for an animation sequence was provided.")]
    DuplicateSequenceProvided(AnimationKey),
}

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("There is no animation frequence for key the ({0})")]
    AnimationSequence(AnimationKey),
    #[error("There are no animation frames for key the ({0})")]
    SingleAnimation(AnimationKey),
}

#[derive(Debug, Error)]
pub enum AnimationFrameError {
    #[error("Starting row ({start}) must not be greate than the ending row ({end}).")]
    InvalidRows {
        start: AnimationIndex,
        end: AnimationIndex,
    },
    #[error("Start index ({start}) must not be greater than the end index ({end}).")]
    InvalidIndexes {
        start: AnimationIndex,
        end: AnimationIndex,
    },
}
