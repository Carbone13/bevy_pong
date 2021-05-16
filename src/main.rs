mod paddle;

use bevy::{
	prelude::*,
};
use crate::paddle::PaddlePlugin;

fn main()
{
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(PaddlePlugin)
		.add_startup_system(initialize.system())
		.run();
}

fn initialize
(
	mut commands: Commands,
)
{
	// Spawn our Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
