mod paddle;

use bevy::{
	prelude::*,
	render::pass::ClearColor,
	sprite::collide_aabb::{collide, Collision},
};
use crate::paddle::PaddlePlugin;



struct Ball {
	velocity: Vec3,
}


fn main()
{
	App::build()
		.add_plugins(DefaultPlugins)
		//.add_plugin(PaddlePlugin)
		.add_startup_system(initialize.system())
		.run();
}

fn initialize
(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>,
)
{
	// Spawn our Camera
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
