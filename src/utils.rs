use bevy::prelude::*;
use std::error::Error;

use crate::{
    animation_error::NegativeAnimationTime,
    types::{AnimationDuration, AnimationIndex},
};

pub fn index_from_row_column(
    row: AnimationIndex,
    column: AnimationIndex,
    columns: AnimationIndex,
) -> AnimationIndex {
    (row * columns) + column
}

pub fn log_if_error<E>(result: Result<(), E>, message: &str)
where
    E: Error,
{
    if let Err(error) = result {
        error!("{}", message);
        error!("Reason: {}", error);
    }
}

pub fn f32_to_animation_duration(time: f32) -> Result<AnimationDuration, NegativeAnimationTime> {
    if time < 0. {
        Err(NegativeAnimationTime(time))
    } else {
        Ok(bevy::utils::Duration::from_secs_f32(time))
    }
}
