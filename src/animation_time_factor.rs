use bevy::{prelude::Component, reflect::Reflect};

use crate::PosScaleFactor;
use bevy::utils::Duration;

#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

#[derive(Default, Component, Reflect)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct AnimationTimeScale(pub PosScaleFactor);

impl AnimationTimeScale {
    pub fn scale_duration(&self, duration: Duration) -> Duration {
        let scale: f32 = self.0.into();
        Duration::from_secs_f32(scale * duration.as_secs_f32())
    }
}
