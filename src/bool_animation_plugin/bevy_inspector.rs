use bevy::prelude::*;
use std::any::{Any, TypeId};

use bevy_inspector_egui::{
    egui, inspector_egui_impls::InspectorEguiImpl, reflect_inspector::InspectorUi,
};

use crate::{PercentScaleFactor, PosScaleFactor};

pub fn setup_bevy_inspect(app: &mut App) {
    let type_registry = app.world.resource::<bevy::ecs::prelude::AppTypeRegistry>();
    let mut type_registry = type_registry.write();
    macro_rules! register_type {
        ($type:ty, $module:ident) => {
            type_registry
                .get_mut(TypeId::of::<$type>())
                .unwrap_or_else(|| panic!("{} not registered", std::any::type_name::<$type>()))
                .insert(InspectorEguiImpl::new(
                    $module::mut_ui,
                    no_op::readonly_ui,
                    no_op::many_ui,
                ));
        };
    }
    register_type!(PosScaleFactor, pos_scale_factor);
    register_type!(PercentScaleFactor, precent_scale_factor);
}

mod precent_scale_factor {
    use super::*;
    pub fn mut_ui(
        value: &mut dyn Any,
        ui: &mut egui::Ui,
        _options: &dyn Any,
        _: egui::Id,
        _: InspectorUi<'_, '_>,
    ) -> bool {
        let value = value.downcast_mut::<PercentScaleFactor>().unwrap();
        let mut number = value.to_f32();
        let mut has_changed = false;
        ui.horizontal(|ui| {
            ui.label("Scale: ");

            has_changed = ui
                .add(egui::DragValue::new(&mut number).speed(0.1))
                .changed();
            if has_changed {
                *value = PercentScaleFactor::clamp(number);
            }
        });
        has_changed
    }
}
mod pos_scale_factor {
    use super::*;
    pub fn mut_ui(
        value: &mut dyn Any,
        ui: &mut egui::Ui,
        _options: &dyn Any,
        _: egui::Id,
        _: InspectorUi<'_, '_>,
    ) -> bool {
        let value = value.downcast_mut::<PosScaleFactor>().unwrap();
        let mut number = value.to_f32();
        let mut has_changed = false;
        ui.horizontal(|ui| {
            ui.label("Scale: ");

            has_changed = ui
                .add(egui::DragValue::new(&mut number).speed(0.1))
                .changed();
            if has_changed {
                *value = PosScaleFactor::clamp(number);
            }
        });
        has_changed
    }
}

mod no_op {
    use super::*;

    pub fn readonly_ui(
        _value: &dyn Any,
        _ui: &mut egui::Ui,
        _options: &dyn Any,
        _: egui::Id,
        _: InspectorUi<'_, '_>,
    ) {
    }
    pub fn many_ui(
        _ui: &mut egui::Ui,
        _: &dyn Any,
        _id: egui::Id,
        _env: InspectorUi<'_, '_>,
        _values: &mut [&mut dyn Reflect],
        _projector: &dyn Fn(&mut dyn Reflect) -> &mut dyn Reflect,
    ) -> bool {
        false
    }
}
