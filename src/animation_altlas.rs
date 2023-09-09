use bevy::prelude::*;
use serde::Deserialize;

use crate::types::AnimationIndex;
#[derive(Debug, Clone)]
pub struct AnimationAltlas {
    meta: AnimationAltlasMeta,
    atlas: Handle<TextureAtlas>,
}

impl AnimationAltlas {
    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.atlas.clone()
    }

    pub fn data(&self) -> &AnimationAltlasMeta {
        &self.meta
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnimationAltlasMeta {
    rows: AnimationIndex,
    columns: AnimationIndex,
    cell_size: Vec2,
    padding: Option<Vec2>,
    offset: Option<Vec2>,
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
