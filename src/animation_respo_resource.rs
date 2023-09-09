use crate::{
    animation_collection::AnimationCollection,
    animation_comp::AnimationComp,
    animation_error::NotFoundError,
    animation_key::AnimationKey,
    save_load::AnimationAssets,
    sprite_animation_bundle::SpriteAnimationBundle,
    types::{AnimationRepository, AnimationResult, KeyLookUpResult},
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
    pub fn animation_under(
        &self,
        key: &AnimationKey,
    ) -> Result<&AnimationCollection, NotFoundError> {
        self.0
            .get(key)
            .ok_or_else(|| NotFoundError::AnimationSequence(key.clone()))
    }

    pub fn add_from_asset(
        &mut self,
        animations: &AnimationAssets,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
    ) -> AnimationResult<&mut Self> {
        let key = animations.name().into();
        self.inner_add_from_asset(key, animations, image, asset_atlases)?;
        Ok(self)
    }

    pub fn replace_from_assets(
        &mut self,
        animations: &AnimationAssets,
    ) -> AnimationResult<&mut Self> {
        let name = animations.name();
        let to_change = self
            .0
            .get_mut(name)
            .ok_or_else(|| NotFoundError::AnimationSequence(name.into()))?;
        let new_seq = animations.to_ani_seq()?;
        to_change.set_frames(new_seq);
        Ok(self)
    }

    pub fn create_sprite_comp(&self, key: &str) -> KeyLookUpResult<SpriteAnimationBundle> {
        let (key, animations) = self
            .0
            .get_key_value(key)
            .unwrap_or_else(|| panic!("No animations registered under key: {}", key));
        let frames = AnimationComp::new(key.clone(), animations.start_state(), self)?;
        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: animations.atlas(),
            sprite: TextureAtlasSprite::new(frames.start_index(self)?),
            ..default()
        };
        Ok(SpriteAnimationBundle {
            sprite_sheet,
            frames,
        })
    }

    fn inner_add_from_asset(
        &mut self,
        key: AnimationKey,
        animations: &AnimationAssets,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
    ) -> AnimationResult<&mut Self> {
        let collection = animations.to_animaton_collection(image, asset_atlases)?;
        if let Some(_) = self.0.insert(key.clone(), collection) {
            info!(
                "New animations are added under existing key ({}) and overrides old animations",
                key
            )
        } else {
            info!("New animations are added under new key ({})", key)
        };
        Ok(self)
    }
}
