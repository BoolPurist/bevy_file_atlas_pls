use bevy::prelude::*;

#[cfg(feature = "bevy_inspect")]
use bevy_inspector_egui::prelude::*;

#[derive(Debug, Component, Reflect)]
#[cfg_attr(
    feature = "bevy_inspect",
    derive(InspectorOptions),
    reflect(InspectorOptions)
)]
pub struct ListenAnimationEnd;
