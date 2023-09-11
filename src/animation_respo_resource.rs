use crate::{
    animation_collection::AnimationCollection,
    animation_comp::AnimationComp,
    animation_error::{AnimationError, NotFoundError},
    animation_key::AnimationKey,
    save_load::AnimationAssets,
    sprite_animation_bundle::SpriteAnimationBundle,
    types::{AnimationRepository, AnimationResult, KeyLookUpResult},
};

use bevy::{prelude::*, utils::HashMap};

#[derive(Resource, Default)]
pub struct AllAnimationResource {
    animation_seqs: AnimationRepository,
    handle_to_key: HashMap<Handle<AnimationAssets>, AnimationKey>,
}

impl AllAnimationResource {
    pub fn state_names_under(&self, key: &str) -> &AnimationCollection {
        self.animation_seqs.get(key).unwrap()
    }
    pub fn add_animations(&mut self, key: &str, collection: AnimationCollection) -> &mut Self {
        let key = AnimationKey::new(key);
        self.animation_seqs.insert(key, collection);
        self
    }
    pub fn animation_under(&self, key: &str) -> Result<&AnimationCollection, NotFoundError> {
        self.animation_seqs
            .get(key)
            .ok_or_else(|| NotFoundError::AnimationSequence(key.into()))
    }

    pub fn add_from_asset(
        &mut self,
        animations: Handle<AnimationAssets>,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
        asset_animation: &Assets<AnimationAssets>,
        key: Option<&str>,
    ) -> AnimationResult<&mut Self> {
        let Some(animations_loaded) = asset_animation.get(&animations) else {
            warn!("Animation asset is not loaded yet.");
            return Ok(self);
        };
        let key: AnimationKey = match (animations_loaded.name(), key) {
            (_, Some(from_func_call)) => from_func_call,
            (Some(from_asset), _) => from_asset,
            _ => {
                return Err(AnimationError::NoSeqeunceKeyProvided);
            }
        }
        .into();

        self.inner_add_from_asset(key.clone(), animations_loaded, image, asset_atlases)?;
        self.handle_to_key.insert(animations, key);
        Ok(self)
    }

    pub fn replace_from_assets(
        &mut self,
        animations: &Handle<AnimationAssets>,
        assets_animations: &Assets<AnimationAssets>,
    ) -> AnimationResult<&mut Self> {
        let Some(animations_loaded) = assets_animations.get(animations) else {
            return Ok(self);
        };
        let name = self.handle_to_key.get(animations).unwrap();
        let to_change = self
            .animation_seqs
            .get_mut(name)
            .ok_or_else(|| NotFoundError::AnimationSequence(name.clone()))?;
        let new_seq = animations_loaded.to_ani_seq()?;
        to_change.set_frames(new_seq);
        Ok(self)
    }

    pub fn create_sprite_comp(&self, key: &str) -> KeyLookUpResult<SpriteAnimationBundle> {
        let (key, animations) = self
            .animation_seqs
            .get_key_value(key)
            .ok_or_else(|| NotFoundError::AnimationSequence(key.into()))?;
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
        if self
            .animation_seqs
            .insert(key.clone(), collection)
            .is_some()
        {
            return Err(AnimationError::DuplicateSequenceProvided(key));
        } else {
            info!("New animations are added under new key ({})", key)
        };
        Ok(self)
    }
}
