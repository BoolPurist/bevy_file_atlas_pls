use crate::{prelude::AnimationSequence, types::AnimationIndex};

pub fn index_from_row_column(
    row: AnimationIndex,
    column: AnimationIndex,
    columns: AnimationIndex,
) -> AnimationIndex {
    (row * columns) + column
}

pub fn get_all_state_names_from_seq(seq: &AnimationSequence) -> Vec<&str> {
    seq.keys().map(|key| key.as_ref()).collect()
}
