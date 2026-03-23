use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::prelude::*;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_systems(Update, hello);
}

fn hello() {
    godot::prelude::godot_print!("Hello from Bevy!");
}