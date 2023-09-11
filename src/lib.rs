pub mod animation_altlas;
pub mod animation_collection;
pub mod animation_comp;
pub mod animation_error;
pub mod animation_frames;
pub mod animation_respo_resource;
pub mod animation_time_factor;
pub mod bool_animation_plugin;
pub mod pos_scale_factor;
pub mod prelude;

#[cfg(feature = "assets")]
pub mod save_load;

pub mod sprite_animation_bundle;
pub mod systems;
pub mod types;

pub use animation_time_factor::AnimationTimeScale;
pub use pos_scale_factor::PosScaleFactor;
pub(crate) mod animation_key;
pub(crate) mod utils;
