use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::Rng;

use crate::{Collider, Scoreboard};

struct Ball
{
	velocity: Vec3,
}

pub struct BallPlugin;
impl Plugin for BallPlugin
{
	fn build (&self, app: &mut AppBuilder)
	{
		app.add_startup_system(spawn_ball.system())
			.add_system(move_ball.system())
			.add_system(ball_collide.system());
	}
}

fn spawn_ball
(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
)
{
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
			transform: Transform::from_xyz(0.0, 0.0, 1.0),
			sprite: Sprite::new(Vec2::new(30.0, 30.0)),
			..Default::default()
		})
		.insert(Ball
		{
			velocity: 400.0 * Vec3::new(0.5, -0.5, 0.0).normalize(),
		});
}

fn move_ball
(
	time: Res<Time>,
 	mut ball_query: Query<(&Ball, &mut Transform)>
)
{
	let delta_seconds = f32::min(0.2, time.delta_seconds());

	if let Ok((ball, mut transform)) = ball_query.single_mut()
	{
		transform.translation += ball.velocity * delta_seconds;
	}
}

fn ball_collide
(
	mut scoreboard: ResMut<Scoreboard>,
	mut ball:      Query<(&mut Transform, &mut Ball, &Sprite)>,
	mut colliders: Query<(&Transform, Entity, &Collider, &Sprite, Without<Ball>)>,
)
{
	if let Ok((mut ball_transform, mut ball, sprite)) = ball.single_mut()
	{
		let mut rng = rand::thread_rng();
		let ball_size = sprite.size;
		let velocity = &mut ball.velocity;

		// check collision with walls
		for (transform, _collider_entity, collider, sprite, _is_ball) in colliders.iter_mut()
		{
			let collision = collide
				(
				ball_transform.translation,
				ball_size,
				transform.translation,
				sprite.size,
				);

			if let Some(collision) = collision
			{
				if let Collider::ScoreLeft = *collider
				{
					scoreboard.right_score += 1;

					ball.velocity = 400.0 * Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
					ball_transform.translation = Vec3::new(0.0, 0.0, 1.0);

					break;
				}
				if let Collider::ScoreRight = *collider
				{
					scoreboard.left_score += 1;

					ball.velocity = 400.0 * Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0).normalize();
					ball_transform.translation = Vec3::new(0.0, 0.0, 1.0);

					break;
				}
				if let Collider::Paddle = *collider
				{
					println!("Hit a paddle, going faster !");
					velocity.x += velocity.x.signum() * 20.0;
					velocity.y += velocity.y.signum() * 20.0;

					println!("{}", velocity.x.to_string());
				}


				// reflect the ball when it collides
				let mut reflect_x = false;
				let mut reflect_y = false;

				// only reflect if the ball's velocity is going in the opposite direction of the
				// collision
				match collision
				{
					Collision::Left => reflect_x = velocity.x > 0.0,
					Collision::Right => reflect_x = velocity.x < 0.0,
					Collision::Top => reflect_y = velocity.y < 0.0,
					Collision::Bottom => reflect_y = velocity.y > 0.0,
				}

				// reflect velocity on the x-axis if we hit something on the x-axis
				if reflect_x
				{
					velocity.x = -velocity.x;
				}

				// reflect velocity on the y-axis if we hit something on the y-axis
				if reflect_y
				{
					velocity.y = -velocity.y;
				}

				// break if this collide is on a solid, otherwise continue check whether a solid is
				// also in collision
				if let Collider::Solid = *collider
				{
					break;
				}
			}
		}
	}
}
