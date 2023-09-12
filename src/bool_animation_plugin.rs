use bevy::{log::LogPlugin, prelude::*, sprite::SpritePlugin, time::TimePlugin};

use crate::{
    animation_respo_resource::AllAnimationResource,
    prelude::AnimationComp,
    systems::{animate, apply_pending_states, do_pending_resets},
    AnimationTimeScale, PosScaleFactor,
};

#[cfg(feature = "assets")]
use crate::{save_load::AnimationAssets, systems::asset_handling};
#[cfg(feature = "assets")]
use bevy_common_assets::ron::RonAssetPlugin;

#[derive(Default)]
pub struct BoolAnimationPlugin;

#[cfg(feature = "bevy_inspect")]
mod bevy_inspector;

impl Plugin for BoolAnimationPlugin {
    fn build(&self, app: &mut App) {
        assert_added_plugin::<ImagePlugin>(app);
        assert_added_plugin::<SpritePlugin>(app);
        assert_added_plugin::<TimePlugin>(app);
        assert_added_plugin::<LogPlugin>(app);
        assert_added_plugin::<AssetPlugin>(app);

        app.init_resource::<AllAnimationResource>()
            .register_type::<AnimationTimeScale>()
            .register_type::<AnimationComp>()
            .register_type::<PosScaleFactor>()
            .add_systems(Update, (apply_pending_states, animate, do_pending_resets));

        #[cfg(feature = "bevy_inspect")]
        bevy_inspector::setup_bevy_inspect(app);
        #[cfg(feature = "assets")]
        {
            app.add_plugins(RonAssetPlugin::<AnimationAssets>::new(&["animations.ron"]));
            asset_handling::regisiter_systems(app);
        };
    }
}

fn assert_added_plugin<T: Plugin>(app: &mut App) {
    assert!(
        app.is_plugin_added::<T>(),
        "Plugin ({0}) from crate ({1}) requires the plugin ({2}).\nPlease add plugin ({2}) before adding plugin ({0}).",
        stringify!(BoolAnimationPlugin),
        env!("CARGO_PKG_NAME"),
        std::any::type_name::<T>(),
    );
}
