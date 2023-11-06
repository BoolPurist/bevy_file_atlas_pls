use std::borrow::Cow;

use crate::{
    animation_collection::AnimationCollection,
    animation_comp::AnimationComp,
    animation_error::NotFoundError,
    sprite_animation_bundle::SpriteAnimationBundle,
    text_like::TextLike,
    types::{AnimationRepository, KeyLookUpResult},
    utils, PosScaleFactor,
};

use bevy::prelude::*;

#[cfg(feature = "assets")]
use crate::{animation_error::AnimationError, save_load::AnimationAssets, types::AnimationResult};
#[cfg(feature = "assets")]
use bevy::utils::HashMap;

#[derive(Resource, Default, Debug)]
pub struct AllAnimationResource {
    animation_seqs: AnimationRepository,
    #[cfg(feature = "assets")]
    handle_to_key: HashMap<AssetId<AnimationAssets>, &'static str>,
    global_animation_duration: PosScaleFactor,
}

impl AllAnimationResource {
    pub fn set_global_animation_duration(&mut self, global_animation_duration: PosScaleFactor) {
        self.global_animation_duration = global_animation_duration;
    }

    pub fn state_names_under(&self, key: &str) -> impl Iterator<Item = &str> + '_ {
        self.animation_sequence(key)
            .frames()
            .iter()
            .map(|frame| frame.0.as_ref())
    }
    pub fn animation_sequence(&self, key: &str) -> &AnimationCollection {
        self.animation_seqs.get(key).unwrap()
    }

    pub fn add_animations<'a>(
        &mut self,
        key: impl Into<TextLike<'a>>,
        collection: AnimationCollection,
    ) -> &mut Self {
        self.animation_seqs
            .insert(key.into().to_registered_name(), collection);
        self
    }

    pub fn animation_under(&self, key: &str) -> Result<&AnimationCollection, NotFoundError> {
        self.animation_seqs
            .get(key)
            .ok_or_else(|| NotFoundError::AnimationSequence(key.into()))
    }

    pub fn create_sprite_comp(&self, key: &str) -> KeyLookUpResult<SpriteAnimationBundle> {
        type StrRef = Cow<'static, str>;

        let (frames_key, animations) = self
            .animation_seqs
            .get_key_value(key)
            .ok_or_else(|| NotFoundError::AnimationSequence(key.into()))?;
        let (frame_key, start_state): (StrRef, StrRef) =
            (Cow::Borrowed(*frames_key), animations.start_state().into());
        let frames = AnimationComp::new(frame_key, start_state, self)?;
        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: animations.atlas(),
            sprite: TextureAtlasSprite::new(frames.start_index(self)?),
            ..default()
        };
        Ok(SpriteAnimationBundle {
            sprite_sheet,
            time_scale: Default::default(),
            frames,
        })
    }

    #[cfg(feature = "assets")]
    pub fn add_from_asset<'a>(
        &mut self,
        animations: Handle<AnimationAssets>,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
        asset_animation: &Assets<AnimationAssets>,
        key: Option<impl Into<TextLike<'a>>>,
    ) -> AnimationResult<&mut Self> {
        let Some(animations_loaded) = asset_animation.get(&animations) else {
            return Err(AnimationError::AnimationNotLoadedYet);
        };
        let animations_loaded = animations_loaded.clone();

        let static_key = {
            let key: TextLike = match (animations_loaded.name(), key) {
                (_, Some(from_func_call)) => from_func_call.into(),
                (Some(from_asset), _) => TextLike::Owned(from_asset.to_string()),
                _ => {
                    return Err(AnimationError::NoSeqeunceKeyProvided);
                }
            }
            .into();
            key.to_registered_name()
        };

        self.inner_add_from_asset(static_key, animations_loaded.clone(), image, asset_atlases)?;
        self.handle_to_key.insert(animations.id(), static_key);
        Ok(self)
    }

    #[cfg(feature = "assets")]
    pub fn replace_from_assets(
        &mut self,
        animations_id: &AssetId<AnimationAssets>,
        assets_animations: &Assets<AnimationAssets>,
    ) -> AnimationResult<&mut Self> {
        match assets_animations.get(*animations_id) {
            Some(animations) => {
                let name = *self.handle_to_key.get(animations_id).unwrap();

                let to_change = self
                    .animation_seqs
                    .get_mut(name)
                    .ok_or_else(|| NotFoundError::AnimationSequence(name.to_string()))?;
                let new_seq = animations.to_ani_seq(self.global_animation_duration)?;
                to_change.set_frames(new_seq);
                Ok(self)
            }
            None => return Ok(self),
        }
    }

    #[cfg(feature = "assets")]
    fn inner_add_from_asset(
        &mut self,
        key: &'static str,
        animations: AnimationAssets,
        image: Handle<Image>,
        asset_atlases: &mut Assets<TextureAtlas>,
    ) -> AnimationResult {
        let collection = animations.to_animaton_collection(
            image,
            asset_atlases,
            self.global_animation_duration,
        )?;
        if self.animation_seqs.insert(key, collection).is_some() {
            return Err(AnimationError::DuplicateKeySequenceProvided(
                key.to_string(),
            ));
        } else {
            info!("New animations are added under new key ({})", key)
        };
        Ok(())
    }
}

impl std::fmt::Display for AllAnimationResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Default animation duration secs: {}",
            self.global_animation_duration
        )?;
        for (seq_key, seq) in self.animation_seqs.iter() {
            writeln!(f, "Sequence key: {}", seq_key)?;

            writeln!(f, "{}", utils::indent_succive(&seq.to_string(), 2))?;
        }
        #[cfg(feature = "assets")]
        {
            writeln!(f, "The following sequences are backed behind a reference\n")?;
            for key in self.handle_to_key.values() {
                write!(f, "- {}", key)?;
            }
        }

        Ok(())
    }
}
