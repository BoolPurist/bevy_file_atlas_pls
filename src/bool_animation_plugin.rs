use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    animation_respo_resource::AllAnimationResource,
    save_load::AnimationAssets,
    systems::{animate, apply_pending_states, do_pending_resets, reload_animation_assets},
};
#[derive(Default)]
pub struct BoolAnimationPlugin;

impl Plugin for BoolAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<AnimationAssets>::new(&["animations.ron"]))
            .init_resource::<AllAnimationResource>()
            .add_systems(
                Update,
                (
                    apply_pending_states,
                    animate,
                    reload_animation_assets,
                    do_pending_resets,
                ),
            );
    }
}
