use bevy::prelude::*;

use crate::{
    animation_comp::AnimationComp, prelude::AllAnimationResource, save_load::AnimationAssets, utils,
};

pub fn animate(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        utils::log_if_error(
            animmator.udpate(&mut texture_sprite, &time, &repos),
            "Updating animation frame over time failed.",
        );
    }
}

pub fn apply_pending_states(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        utils::log_if_error(
            animmator.do_pending_change(&mut texture_sprite, &repos),
            "Applying state change for animation failed.",
        );
    }
}
pub fn do_pending_resets(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        utils::log_if_error(
            animmator.do_pending_reset(&mut texture_sprite, &repos),
            "Applying state reset for animation failed.",
        );
    }
}

#[allow(clippy::single_match)]
pub fn reload_animation_assets(
    assets_antimation: Res<Assets<AnimationAssets>>,
    mut asset_event: EventReader<AssetEvent<AnimationAssets>>,
    mut repository: ResMut<AllAnimationResource>,
) {
    for event in asset_event.iter() {
        match event {
            AssetEvent::Modified { handle } => {
                if let Some(animations) = assets_antimation.get(handle) {
                    if let Err(error) = repository.replace_from_assets(animations) {
                        error!(
                            "Changes from animation assets not applied, due to error.\nDetails: {}",
                            error
                        )
                    }
                }
            }
            _ => (),
        }
    }
}
