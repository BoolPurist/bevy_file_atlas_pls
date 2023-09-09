use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_asset_loader::prelude::*;

use assets::{prelude::*, save_load::AnimationAssets};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(
                        bevy::utils::Duration::from_millis(200),
                    ),
                    ..Default::default()
                })
                .build(),
            BoolAnimationPlugin,
        ))
        .add_state::<GameLoadingState>()
        .add_loading_state(
            LoadingState::new(GameLoadingState::Loading).continue_to_state(GameLoadingState::Done),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameLoadingState::Loading)
        .add_systems(
            OnEnter(GameLoadingState::Done),
            (setup_animated_sprites, setup),
        )
        .add_systems(
            Update,
            change_state_on_input.run_if(in_state(GameLoadingState::Done)),
        )
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLoadingState {
    #[default]
    Loading,
    Done,
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "BODY_skeleton.png")]
    pub skeleton_sprite: Handle<Image>,
    #[asset(path = "player.animations.ron")]
    pub skeleton_animations: Handle<AnimationAssets>,
}

pub const PLAYER_TAG: &str = "player";
fn setup_animated_sprites(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ani_respo: ResMut<AllAnimationResource>,
    animations: Res<Assets<AnimationAssets>>,
    mut atlase_coll: ResMut<Assets<TextureAtlas>>,
) {
    let skeleton_image = assets.skeleton_sprite.clone();
    let ani = animations.get(&assets.skeleton_animations).unwrap();
    ani_respo
        .add_from_asset(ani, skeleton_image, &mut atlase_coll)
        .unwrap();
    commands.spawn((ani_respo.create_sprite_comp(PLAYER_TAG).unwrap(), Player));
}

#[allow(dead_code)]
fn via_code(
    skeleton_image: Handle<Image>,
    mut ani_respo: ResMut<AllAnimationResource>,
    mut atlase_coll: ResMut<Assets<TextureAtlas>>,
) {
    let meta =
        AnimationAltlasMeta::new(4, 6, Vec2::splat(64.)).build(skeleton_image, &mut atlase_coll);
    let skeleton_animations = AnimationCollectionBuilder::new(meta)
        .add_row_ani(AniStates::Left.into(), 1, 1.0)
        .add_row_ani(AniStates::Right.into(), 3, 1.0)
        .add_row_ani(AniStates::Bottom.into(), 2, 1.0)
        .add_row_ani(AniStates::Top.into(), 0, 1.0)
        .build(AniStates::Left.into());
    ani_respo.add_animations(PLAYER_TAG, skeleton_animations);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
pub struct Player;

fn change_state_on_input(mut query: Query<&mut AnimationComp>, input: Res<Input<KeyCode>>) {
    let mut animation = query.single_mut();
    if input.just_pressed(KeyCode::A) {
        animation.change_state(AniStates::Left.into());
    }
    if input.just_pressed(KeyCode::D) {
        animation.change_state(AniStates::Right.into());
    }
    if input.just_pressed(KeyCode::W) {
        animation.change_state(AniStates::Top.into());
    }
    if input.just_pressed(KeyCode::S) {
        animation.change_state(AniStates::Bottom.into());
    }
}

pub enum AniStates {
    Left,
    Right,
    Bottom,
    Top,
}

impl From<AniStates> for &'static str {
    fn from(value: AniStates) -> Self {
        match value {
            AniStates::Left => "Left",
            AniStates::Right => "Right",
            AniStates::Bottom => "Bottom",
            AniStates::Top => "Top",
        }
    }
}
