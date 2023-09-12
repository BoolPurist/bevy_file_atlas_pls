use bevy::prelude::*;
#[cfg(feature = "assets")]
use serde::Deserialize;

use crate::types::AnimationIndex;
#[derive(Debug, Clone)]
pub struct AnimationAltlas {
    meta: AnimationAltlasMeta,
    atlas: Handle<TextureAtlas>,
}

impl std::fmt::Display for AnimationAltlas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}

impl AnimationAltlas {
    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.atlas.clone()
    }

    pub fn data(&self) -> &AnimationAltlasMeta {
        &self.meta
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "assets", derive(Deserialize))]
pub struct AnimationAltlasMeta {
    rows: AnimationIndex,
    columns: AnimationIndex,
    cell_size: Vec2,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
}
impl std::fmt::Display for AnimationAltlasMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rows: {}", self.rows)?;
        writeln!(f, "Columns: {}", self.columns)?;
        writeln!(f, "Cell size: {}", self.cell_size)?;
        writeln!(f, "Padding: {}", self.padding.unwrap_or_default())?;
        writeln!(f, "Offset: {}", self.offset.unwrap_or_default())?;
        Ok(())
    }
}

impl AnimationAltlasMeta {
    pub fn new(rows: AnimationIndex, columns: AnimationIndex, cell_size: Vec2) -> Self {
        Self {
            rows,
            columns,
            cell_size,
            padding: None,
            offset: None,
        }
    }
    pub fn new_padding(mut self, padding: Vec2) -> Self {
        self.padding = Some(padding);
        self
    }
    pub fn new_offset(mut self, offset: Vec2) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn build(
        self,
        image: Handle<Image>,
        assets_atlas: &mut Assets<TextureAtlas>,
    ) -> AnimationAltlas {
        let atlas = TextureAtlas::from_grid(
            image,
            self.cell_size,
            self.columns,
            self.rows,
            self.padding,
            self.offset,
        );
        let atlas = assets_atlas.add(atlas);
        AnimationAltlas { atlas, meta: self }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn cell_size(&self) -> Vec2 {
        self.cell_size
    }

    pub fn padding(&self) -> Option<Vec2> {
        self.padding
    }

    pub fn offset(&self) -> Option<Vec2> {
        self.offset
    }
}
