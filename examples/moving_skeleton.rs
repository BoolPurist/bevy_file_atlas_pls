use bevy::{asset::ChangeWatcher, prelude::*, utils::petgraph::matrix_graph::Zero};
use bevy_asset_loader::prelude::*;

use bevy_file_atlas_pls::{prelude::*, save_load::AnimationAssets};

pub const PLAYER_TAG: &str = "playerxxx";
pub const PLAYER_SPEED: f32 = 200.;

fn main() {
    setup_app();
}

fn change_state_on_input(
    mut query: Query<(&mut AnimationComp, &mut Transform), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let (mut animation, mut location) = query.single_mut();
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::A) {
        animation.change_state(AniStates::Left.into());
        direction.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        animation.change_state(AniStates::Right.into());
        direction.x += 1.;
    }
    if input.pressed(KeyCode::W) {
        animation.change_state(AniStates::Top.into());
        direction.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        animation.change_state(AniStates::Bottom.into());
        direction.y -= 1.;
    }
    let movement = direction * time.delta_seconds() * PLAYER_SPEED;

    location.translation += Vec3::new(movement.x, movement.y, 0.);
    if direction.length().is_zero() {
        animation.reset_current_state();
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

#[derive(Component)]
pub struct Player;

fn setup_animated_sprites(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ani_respo: ResMut<AllAnimationResource>,
    animations: Res<Assets<AnimationAssets>>,
    mut atlase_coll: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let skeleton_image = assets.skeleton_sprite.clone();
    ani_respo
        .add_from_asset(
            assets.skeleton_animations.clone(),
            skeleton_image,
            &mut atlase_coll,
            &animations,
            Some(PLAYER_TAG),
        )
        .unwrap();
    commands.spawn((ani_respo.create_sprite_comp(PLAYER_TAG).unwrap(), Player));
}

fn setup_app() {
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
        .add_systems(OnEnter(GameLoadingState::Done), setup_animated_sprites)
        .add_systems(
            Update,
            (print_player_animation_status(1.), change_state_on_input)
                .run_if(in_state(GameLoadingState::Done)),
        )
        .run();
}

fn print_player_animation_status(
    interval_secs: f32,
) -> impl FnMut(Res<Time>, Query<&AnimationComp, With<Player>>) {
    use bevy::utils::Duration;
    let mut timer = Timer::new(Duration::from_secs_f32(interval_secs), TimerMode::Repeating);
    move |time, query| {
        if timer.tick(time.delta()).just_finished() {
            if let Ok(state) = query.get_single() {
                println!("Current state of player: {}", state.current_state())
            }
        }
    }
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(path = "BODY_skeleton.png")]
    pub skeleton_sprite: Handle<Image>,
    #[asset(path = "player.animations.ron")]
    pub skeleton_animations: Handle<AnimationAssets>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLoadingState {
    #[default]
    Loading,
    Done,
}
