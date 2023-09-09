use bevy::prelude::*;

use crate::{
    animation_comp::AnimationComp, prelude::AllAnimationResource, save_load::AnimationAssets,
};

pub fn animate(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        animmator.udpate(&mut texture_sprite, &time, &repos);
    }
}

pub fn apply_pending_states(
    mut query: Query<(&mut AnimationComp, &mut TextureAtlasSprite)>,
    repos: Res<AllAnimationResource>,
) {
    for (mut animmator, mut texture_sprite) in query.iter_mut() {
        animmator.do_pending_change(&mut texture_sprite, &repos);
    }
}

pub fn reload_animation_assets(
    assets_antimation: Res<Assets<AnimationAssets>>,
    mut asset_event: EventReader<AssetEvent<AnimationAssets>>,
    mut repository: ResMut<AllAnimationResource>,
) {
    for event in asset_event.iter() {
        match event {
            AssetEvent::Modified { handle } => {
                let animations = assets_antimation.get(handle).unwrap();
                repository.replace_from_assets(animations);
            }
            _ => (),
        }
    }
}
