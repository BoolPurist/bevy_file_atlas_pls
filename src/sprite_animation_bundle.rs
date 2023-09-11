use bevy::prelude::*;

use crate::{animation_comp::AnimationComp, animation_time_factor::AnimationTimeScale};

#[derive(Bundle)]
pub struct SpriteAnimationBundle {
    pub sprite_sheet: SpriteSheetBundle,
    pub frames: AnimationComp,
    pub time_scale: AnimationTimeScale,
}
