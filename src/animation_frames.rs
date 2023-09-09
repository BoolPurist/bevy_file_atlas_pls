use crate::{
    animation_error::AnimationFrameError,
    types::{AnimationFrameResult, AnimationIndex},
    utils,
};

#[derive(Debug, Clone)]
pub struct AnimationFrames {
    start: AnimationIndex,
    end: AnimationIndex,
    time: f32,
}
impl AnimationFrames {
    pub fn new(
        row: AnimationIndex,
        column: Option<AnimationIndex>,
        end_row: Option<AnimationIndex>,
        end_column: Option<AnimationIndex>,
        time: f32,
        columns: AnimationIndex,
    ) -> AnimationFrameResult {
        let (column, end_row, end_column) = (
            column.unwrap_or(0),
            end_row.unwrap_or(row),
            end_column.unwrap_or(columns),
        );

        if row > end_row {
            return Err(AnimationFrameError::InvalidRows {
                start: row,
                end: end_row,
            });
        }

        let (start, end) = (
            utils::index_from_row_column(row, column, columns),
            utils::index_from_row_column(end_row, end_column, columns).saturating_sub(1),
        );

        if start > end {
            return Err(AnimationFrameError::InvalidIndexes { start, end });
        }

        Ok(Self { start, end, time })
    }

    pub fn from_row(row: AnimationIndex, time: f32, columns: usize) -> AnimationFrameResult {
        Self::new(row, None, None, None, time, columns)
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn frame_gap(&self) -> usize {
        self.end - self.start
    }

    pub fn time_per_frame(&self) -> f32 {
        self.time() / self.frame_gap() as f32
    }

    pub fn end(&self) -> usize {
        self.end
    }
}
