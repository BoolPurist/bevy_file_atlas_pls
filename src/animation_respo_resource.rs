use crate::{
    animation_collection::AnimationCollection, animation_comp::AnimationComp,
    animation_key::AnimationKey, save_load::AnimationAssets,
    sprite_animation_bundle::SpriteAnimationBundle, types::AnimationRepository,
};

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct AllAnimationResource(AnimationRepository);

impl AllAnimationResource {
    pub fn state_names_under(&self, key: &str) -> &AnimationCollection {
        self.0.get(key).unwrap()
    }
    pub fn add_animations(&mut self, key: &str, collection: AnimationCollection) -> &mut Self {
        let key = AnimationKey::new(key);
        self.0.insert(key, collection);
        self
    }
    pub fn animation_under(&self, key: &AnimationKey) -> &AnimationCollection {
        self.0.get(key).unwrap()
    }

    pub fn add_from_asset(
        &mut self,
        animations: &AnimationAssets,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
    ) -> &mut Self {
        let key = animations.name().into();
        self.inner_add_from_asset(key, animations, image, asset_atlases);
        self
    }

    pub fn replace_from_assets(&mut self, animations: &AnimationAssets) -> &mut Self {
        let to_change = self.0.get_mut(animations.name()).unwrap();
        let new_seq = animations.to_ani_seq();
        to_change.set_frames(new_seq);
        self
    }

    pub fn create_sprite_comp(&self, key: &str) -> SpriteAnimationBundle {
        let (key, animations) = self
            .0
            .get_key_value(key)
            .unwrap_or_else(|| panic!("No animations registered under key: {}", key));
        let frames = AnimationComp::new(key.clone(), animations.start_state(), self);
        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: animations.atlas(),
            sprite: TextureAtlasSprite::new(frames.start_index(self)),
            ..default()
        };
        SpriteAnimationBundle {
            sprite_sheet,
            frames,
        }
    }

    fn inner_add_from_asset(
        &mut self,
        key: AnimationKey,
        animations: &AnimationAssets,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
    ) -> &mut Self {
        let collection = animations.to_animaton_collection(image, asset_atlases);
        self.0.insert(key, collection);
        self
    }
}
