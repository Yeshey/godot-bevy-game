use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_asset_loader::prelude::*;
use godot::classes::Input;
use godot_bevy::prelude::{
    godot_prelude::{ExtensionLibrary, gdextension},
    *,
};
use godot_bevy::plugins::core::PhysicsDelta;

#[bevy_app]
#[no_mangle]
fn android_main(app: &mut App) {
    app.add_plugins(GodotDefaultPlugins)
        .add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Playing)
                .load_collection::<PlayerAssets>(),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_player)
        .add_systems(
            PhysicsUpdate,
            move_player.run_if(in_state(GameState::Playing)),
        );
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "scenes/player.tscn")]
    scene: Handle<GodotResource>,
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn spawn_player(mut commands: Commands, assets: Res<PlayerAssets>) {
    commands
        .spawn_empty()
        .insert(Player { speed: 300.0 })
        .insert(GodotScene::from_handle(assets.scene.clone()))
        .insert(Transform::from_xyz(400.0, 300.0, 0.0));
}

fn move_player(
    mut query: Query<(&Player, &mut Transform)>,
    physics_delta: Res<PhysicsDelta>,
    mut godot: GodotAccess,
) {
    let Ok((player, mut transform)) = query.single_mut() else {
        return;
    };

    let input = godot.singleton::<Input>();
    let mut velocity = godot::builtin::Vector2::ZERO;

    if input.is_action_pressed("ui_right") { velocity.x += 1.0; }
    if input.is_action_pressed("ui_left")  { velocity.x -= 1.0; }
    if input.is_action_pressed("ui_down")  { velocity.y += 1.0; }
    if input.is_action_pressed("ui_up")    { velocity.y -= 1.0; }

    if velocity.length() > 0.0 {
        velocity = velocity.normalized() * player.speed;
    }

    transform.translation.x += velocity.x * physics_delta.delta_seconds;
    transform.translation.y += velocity.y * physics_delta.delta_seconds;
}