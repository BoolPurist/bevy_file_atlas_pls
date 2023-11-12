use bevy::prelude::*;
use std::error::Error;

use crate::{
    animation_ended::AnimationProgress,
    animation_respo_resource::AllAnimationResource,
    prelude::AnimationComp,
    types::{AnimationIndex, KeyLookUpResult},
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

pub fn indent_succive(to_ident: &str, pad_amount: usize) -> String {
    to_ident
        .lines()
        .map(|to_pad| format!("{}{}", " ".repeat(pad_amount), to_pad))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn get_progress_of_animation(
    comp: &AnimationComp,
    repos: &AllAnimationResource,
    sprite_altas: &TextureAtlasSprite,
) -> KeyLookUpResult<AnimationProgress> {
    let current_animation = comp.get_current_seq(repos)?;
    let current_index = sprite_altas.index as f32;
    let divisor = current_animation.frame_gap() as f32;
    let dividend = current_index - current_animation.start() as f32;
    let frame_progress = dividend / divisor;

    let time_progress = comp.duration_for_animation.percent();
    let progress_per_frame = (1. / divisor) * time_progress;
    Ok(AnimationProgress::new(frame_progress + progress_per_frame))
}
