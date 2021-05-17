mod paddle;
mod ball;
mod level;
mod scoreboard;

use bevy::{
	prelude::*,
};
use crate::paddle::PaddlePlugin;
use crate::ball::BallPlugin;
use crate::level::LevelPlugin;
use crate::scoreboard::ScoreboardPlugin;

enum Collider
{
	Paddle,
	Solid,
	ScoreLeft,
	ScoreRight,
}

struct Scoreboard
{
	left_score: u16,
	right_score: u16,
}

fn main()
{
	App::build()
		.add_plugins(DefaultPlugins)
		.insert_resource(Scoreboard { left_score: 0, right_score:0 })
		.add_plugin(PaddlePlugin)
		.add_plugin(BallPlugin)
		.add_plugin(LevelPlugin)
		.add_plugin(ScoreboardPlugin)
		.add_startup_system(initialize.system())
		.run();
}

fn initialize
(
	mut commands: Commands,
)
{
	// Spawn the Camera & the UI renderer
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());
}
