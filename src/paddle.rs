use bevy::prelude::*;
use crate::Collider;

struct Paddle
{
	speed: f32,
	position: PaddlePosition,
}

enum PaddlePosition
{
	Left, Right,
}

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin
{
	fn build (&self, app: &mut AppBuilder)
	{
		app.add_startup_system(spawn_paddles.system())
			.add_system(move_paddles.system());
	}
}

fn spawn_paddles
(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
)
{
	// Spawn the Left Paddle
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
			transform: Transform::from_xyz(-450.0, 0.0, 0.0),
			sprite: Sprite::new(Vec2::new(30.0, 150.0)),
			..Default::default()
		})
		.insert(Paddle { speed: 500.0, position: PaddlePosition::Left })
		.insert(Collider::Paddle);

	// Spawn the Right Paddle
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
			transform: Transform::from_xyz(450.0, 0.0, 0.0),
			sprite: Sprite::new(Vec2::new(30.0, 150.0)),
			..Default::default()
		})
		.insert(Paddle { speed: 500.0, position: PaddlePosition::Right })
		.insert(Collider::Paddle);
}

fn move_paddles
(
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&Paddle, &mut Transform)>,
)
{
	for (paddle, mut transform) in query.iter_mut()
	{
		let mut direction = 0.0;

		match paddle.position
		{
			PaddlePosition::Left =>
				{
					if keyboard_input.pressed(KeyCode::Z)
					{
						direction += 1.0;
					}
					if keyboard_input.pressed(KeyCode::S)
					{
						direction -= 1.0;
					}
				}
			PaddlePosition::Right =>
				{
					if keyboard_input.pressed(KeyCode::Up)
					{
						direction += 1.0;
					}
					if keyboard_input.pressed(KeyCode::Down)
					{
						direction -= 1.0;
					}
				}
		}

		let translation = &mut transform.translation;

		translation.y += time.delta_seconds() * direction * paddle.speed;
		translation.y = translation.y.min(260.0).max(-260.0);
	}
}
