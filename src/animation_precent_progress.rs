use bevy::{prelude::Component, reflect::Reflect};
#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

use crate::PosScaleFactor;

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Reflect, Component)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct AnimationPrecentProgress(pub(crate) PosScaleFactor);

impl AnimationPrecentProgress {
    pub fn value(&self) -> PosScaleFactor {
        self.0
    }
}
