pub mod prelude;

#[cfg(feature = "assets")]
pub mod save_load;

pub mod types;

pub use animation_ended::AnimationEnded;
pub use animation_precent_progress::AnimationPrecentProgress;
pub use animation_time_factor::AnimationTimeScale;
pub use listen_animation_end::ListenAnimationEnd;
pub use pos_scale_factor::{InvalidPosScaleValue, PosScaleFactor};
pub use precent::{InvalidScaleValue, PercentScaleFactor};

pub(crate) mod utils;

mod animation_altlas;
mod animation_collection;
mod animation_comp;
mod animation_ended;
mod animation_error;
mod animation_frames;
mod animation_precent_progress;
mod animation_respo_resource;
mod animation_time_factor;
mod bool_animation_plugin;
mod listen_animation_end;
mod pos_scale_factor;
mod precent;
mod sprite_animation_bundle;
mod static_text_repos;
mod systems;
mod text_like;
