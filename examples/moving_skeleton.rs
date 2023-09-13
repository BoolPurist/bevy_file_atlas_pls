use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::window::PrimaryWindow;
use bevy::{asset::ChangeWatcher, utils::petgraph::matrix_graph::Zero};
use bevy_asset_loader::prelude::*;
use bevy_file_atlas_pls::listen_animation_end::ListenAnimationEnd;
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui::Ui;
use bevy_inspector_egui::{egui, DefaultInspectorConfigPlugin};

use bevy_file_atlas_pls::{prelude::*, save_load::AnimationAssets};
use bevy_inspector_egui::bevy_egui::EguiPlugin;

pub const PLAYER_TAG: &str = "player";
pub const PLAYER_SPEED: f32 = 200.;

fn main() {
    setup_app();
}

fn change_state_on_input(
    mut query: Query<(&mut AnimationComp, &mut Transform), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    if time.is_paused() {
        return;
    }
    let (mut animation, mut location) = query.single_mut();
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::A) {
        direction.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        direction.x += 1.;
    }
    if input.pressed(KeyCode::W) {
        direction.y += 1.;
    }
    if input.pressed(KeyCode::S) {
        direction.y -= 1.;
    }
    let movement = direction * time.delta_seconds() * PLAYER_SPEED;
    if direction.x > 0. {
        animation.change_state(AniStates::Right.to_str());
    } else if direction.x < 0. {
        animation.change_state(AniStates::Left.to_str());
    } else if direction.y > 0. {
        animation.change_state(AniStates::Top.to_str());
    } else {
        animation.change_state(AniStates::Bottom.to_str());
    }

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

impl AniStates {
    fn to_str(self) -> &'static str {
        match self {
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
    commands.spawn((
        ani_respo.create_sprite_comp(PLAYER_TAG).unwrap(),
        Player,
        Name::new(PLAYER_TAG),
        ListenAnimationEnd,
    ));
    commands.spawn((
        TextBundle::from_section(
            "Last",
            TextStyle {
                font_size: 25.0,
                color: Color::BLACK,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(25.0),
            left: Val::Px(25.0),
            ..default()
        }),
        LastAnimationUi,
    ));

    println!("{}", *ani_respo);
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
            EguiPlugin,
            DefaultInspectorConfigPlugin,
        ))
        .add_state::<GameLoadingState>()
        .init_resource::<TimeScaleIncrement>()
        .add_loading_state(
            LoadingState::new(GameLoadingState::Loading).continue_to_state(GameLoadingState::Done),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameLoadingState::Loading)
        .add_systems(OnEnter(GameLoadingState::Done), setup_animated_sprites)
        .add_systems(
            Update,
            (
                show_last_animation,
                print_player_animation_status(1.),
                change_state_on_input,
                scale_animation_factor(0.25),
                pause_game(0.5),
                ui_dump_show,
            )
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

fn scale_animation_factor(
    cooldown: f32,
) -> impl FnMut(
    Query<&mut AnimationTimeScale, With<Player>>,
    Res<Input<KeyCode>>,
    Res<Time>,
    Res<TimeScaleIncrement>,
) {
    let mut timer = Timer::new(
        bevy::utils::Duration::from_secs_f32(cooldown),
        TimerMode::Once,
    );
    move |mut query, input, time, scale_increment| {
        timer.tick(time.delta());
        let pressed_shift = input.pressed(KeyCode::ShiftLeft) || input.pressed(KeyCode::ShiftRight);
        let pressed_space = input.pressed(KeyCode::Space);
        if pressed_space {
            if !timer.finished() {
                return;
            }
            timer.reset();
            let mut player = query.single_mut();
            let current = &mut player.0;

            if pressed_shift {
                *current -= scale_increment.0;
            } else {
                *current += scale_increment.0;
            }
            info!("New current time factor ({}) for player animation", current);
        }
    }
}

fn pause_game(cooldown: f32) -> impl FnMut(Res<Input<KeyCode>>, ResMut<Time>) {
    let mut timer = Timer::new(
        bevy::utils::Duration::from_secs_f32(cooldown),
        TimerMode::Once,
    );
    move |input, mut time| {
        timer.tick(time.delta());
        if input.just_pressed(KeyCode::P) {
            if !timer.finished() {
                return;
            }
            if time.is_paused() {
                time.unpause();
                info!("Unpaused");
            } else {
                info!("Paused");
                time.pause();
            }
        }
    }
}

#[derive(Component)]
struct LastAnimationUi;
fn show_last_animation(
    mut on_animation_ended: EventReader<AnimationEnded>,
    mut query: Query<&mut Text, With<LastAnimationUi>>,
) {
    if let Some(last_animatio) = on_animation_ended.into_iter().last() {
        info!("Last: {:?}", &last_animatio);
        let mut text = query.single_mut();
        text.sections[0].value = format!("Last: {}", last_animatio.state);
    }
}

fn ui_dump_show(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
        ui.add_space(10.);
        ui.separator();
        ui.add_space(10.);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("Relevant values");
            ui.add_space(10.);
            ui.separator();
            ui.add_space(10.);
            show_time(world, ui);
        });
    });

    fn show_time(world: &mut World, ui: &mut Ui) {
        let mut time = world.resource_mut::<Time>();
        let paused = &mut time.is_paused();
        ui.checkbox(paused, "Paused");
        set_paused(&mut time, *paused)
    }
}

#[derive(Debug, Resource)]
pub struct TimeScaleIncrement(pub PosScaleFactor);

impl Default for TimeScaleIncrement {
    fn default() -> Self {
        Self(PosScaleFactor::at_least_zero(0.1))
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

fn set_paused(time: &mut Time, paused: bool) {
    if paused {
        time.pause();
    } else {
        time.unpause();
    }
}
