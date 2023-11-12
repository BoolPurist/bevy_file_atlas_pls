use bevy::prelude::*;
use std::error::Error;

use crate::types::AnimationIndex;

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
