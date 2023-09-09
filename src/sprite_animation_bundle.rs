use bevy::prelude::*;

use crate::animation_comp::AnimationComp;

#[derive(Bundle)]
pub struct SpriteAnimationBundle {
    pub sprite_sheet: SpriteSheetBundle,
    pub frames: AnimationComp,
}
