mod paddle;
mod ball;
mod level;

use bevy::{
	prelude::*,
};
use crate::paddle::PaddlePlugin;
use crate::ball::BallPlugin;
use crate::level::LevelPlugin;

enum Collider
{
	Paddle,
	Solid,
	ScoreLeft,
	ScoreRight,
}

fn main()
{
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(PaddlePlugin)
		.add_plugin(BallPlugin)
		.add_plugin(LevelPlugin)
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

	// Spawn the 2 goals
}
