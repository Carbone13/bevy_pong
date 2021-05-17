use bevy::prelude::*;
use crate::Scoreboard;

pub struct ScoreboardPlugin;
impl Plugin for ScoreboardPlugin
{
	fn build (&self, app: &mut AppBuilder)
	{
		app
			.add_startup_system(show_scoreboard.system())
			.add_system(update_scoreboard.system());
	}
}

fn show_scoreboard
(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
)
{
	println!("Spawning text");

	commands.spawn_bundle(TextBundle {
		text: Text {
			alignment: TextAlignment
			{
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
			sections: vec![
				TextSection {
					value: "0".to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraMono-Medium.ttf"),
						font_size: 60.0,
						color: Color::rgb(0.75, 0.75, 0.75),
					},
				},
				TextSection {
					value: " : ".to_string(),
					style: TextStyle {
						font: asset_server.load("fonts/FiraMono-Bold.ttf"),
						font_size: 60.0,
						color: Color::rgb(0.6, 0.6, 0.6),
					},
				},
				TextSection {
					value: "0".to_string(),
					style: TextStyle
					{
						font: asset_server.load("fonts/FiraMono-Medium.ttf"),
						font_size: 60.0,
						color: Color::rgb(0.75, 0.75, 0.75),
					},
				},
			],
			..Default::default()
		},
		style: Style
		{
			align_content: AlignContent::Center,
			align_items: AlignItems::Center,
			align_self: AlignSelf::Center,
			position_type: PositionType::Absolute,
			position: Rect {
				left: Val::Percent(45.0),
				..Default::default()
			},
			..Default::default()
		},
		..Default::default()
	});
}

fn update_scoreboard
(
	scoreboard: Res<Scoreboard>,
	mut query: Query<&mut Text>
)
{
	let mut text = query.single_mut().unwrap();
	text.sections[0].value = format!("{}", scoreboard.left_score.to_string());
	text.sections[2].value = format!("{}", scoreboard.right_score.to_string());
}
