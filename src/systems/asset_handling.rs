use bevy::prelude::*;

use crate::{animation_respo_resource::AllAnimationResource, save_load::AnimationAssets};

#[allow(clippy::single_match)]
pub fn reload_animation_assets(
    assets_antimation: Res<Assets<AnimationAssets>>,
    mut asset_event: EventReader<AssetEvent<AnimationAssets>>,
    mut repository: ResMut<AllAnimationResource>,
) {
    for event in asset_event.read() {
        match event {
            AssetEvent::Modified { id } => {
                if let Err(error) = repository.replace_from_assets(id, &assets_antimation) {
                    error!(
                        "Changes from animation assets not applied, due to error.\nDetails: {}",
                        error
                    )
                }
            }
            _ => (),
        }
    }
}

pub fn regisiter_systems(app: &mut App) {
    app.add_systems(Update, reload_animation_assets);
}
