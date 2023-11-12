use bevy::{prelude::Component, reflect::Reflect};
#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

use crate::PercentScaleFactor;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Reflect, Component)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct AnimationPrecentProgress {
    pub progress: PercentScaleFactor,
    pub manual: bool,
}

impl Default for AnimationPrecentProgress {
    fn default() -> Self {
        Self {
            progress: PercentScaleFactor::zero(),
            manual: false,
        }
    }
}

impl AnimationPrecentProgress {
    pub fn value(&self) -> PercentScaleFactor {
        self.progress
    }
}
