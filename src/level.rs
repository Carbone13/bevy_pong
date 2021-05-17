use bevy::prelude::*;
use crate::Collider;

pub struct LevelPlugin;
impl Plugin for LevelPlugin
{
	fn build (&self, app: &mut AppBuilder)
	{
		app.add_startup_system(spawn_goals.system())
			.add_startup_system(spawn_walls.system());
	}
}

fn spawn_goals
(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
)
{
	let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
	let wall_thickness = 5.0;
	let bounds = &Vec2::new(900.0, 600.0);

	// left
	commands
		.spawn_bundle(SpriteBundle {
			material: wall_material.clone(),
			transform: Transform::from_xyz(-bounds.x.clone() / 2.0, 0.0, 0.0),
			sprite: Sprite::new(Vec2::new(wall_thickness.clone(), bounds.y.clone() + wall_thickness.clone())),
			..Default::default()
		})
		.insert(Collider::Solid);
	// right
	commands
		.spawn_bundle(SpriteBundle {
			material: wall_material.clone(),
			transform: Transform::from_xyz(bounds.x.clone() / 2.0, 0.0, 0.0),
			sprite: Sprite::new(Vec2::new(wall_thickness.clone(), bounds.y.clone() + wall_thickness.clone())),
			..Default::default()
		})
		.insert(Collider::Solid);
}

fn spawn_walls
(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
)
{
	let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
	let wall_thickness = 10.0;
	let bounds = &Vec2::new(900.0, 600.0);

	// bottom
	commands
		.spawn_bundle(SpriteBundle {
			material: wall_material.clone(),
			transform: Transform::from_xyz(0.0, -bounds.y.clone() / 2.0, 0.0),
			sprite: Sprite::new(Vec2::new(bounds.x.clone() + wall_thickness.clone(), wall_thickness.clone())),
			..Default::default()
		})
		.insert(Collider::Solid);
	// top
	commands
		.spawn_bundle(SpriteBundle {
			material: wall_material,
			transform: Transform::from_xyz(0.0, bounds.y.clone() / 2.0, 0.0),
			sprite: Sprite::new(Vec2::new(bounds.x.clone() + wall_thickness.clone(), wall_thickness)),
			..Default::default()
		})
		.insert(Collider::Solid);
}
