use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use bevy_asset_loader::prelude::*;
use godot_bevy::prelude::{
    godot_prelude::{ExtensionLibrary, gdextension},
    *,
};
use godot_bevy::plugins::core::PhysicsDelta;

#[bevy_app]
fn build_app(app: &mut App) {
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
    input: Res<ButtonInput<KeyCode>>,  // ← Bevy's input, bridged by godot-bevy
) {
    let Ok((player, mut transform)) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    if input.pressed(KeyCode::ArrowLeft)  || input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if input.pressed(KeyCode::ArrowDown)  || input.pressed(KeyCode::KeyS) { direction.y += 1.0; }
    if input.pressed(KeyCode::ArrowUp)    || input.pressed(KeyCode::KeyW) { direction.y -= 1.0; }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    transform.translation.x += direction.x * player.speed * physics_delta.delta_seconds;
    transform.translation.y += direction.y * player.speed * physics_delta.delta_seconds;
}