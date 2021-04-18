use bevy::prelude::*;

use crate::{food::GrowthEvent, game::GameOverEvent, map::{MAP_HEIGHT, MAP_WIDTH, Position, Size}};

pub struct SnakeHead {
	direction: Direction,
}

pub struct SnakeSegment;

#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);

pub struct Materials {
	pub head_material: Handle<ColorMaterial>,
	pub segment_material: Handle<ColorMaterial>,
	pub food_material: Handle<ColorMaterial>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
	Left,
	Up,
	Right,
	Down,
}

impl Direction {
	pub fn opposite(self) -> Self {
		match self {
			Self::Left => Self::Right,
			Self::Right => Self::Left,
			Self::Up => Self::Down,
			Self::Down => Self::Up,
		}
	}
}

pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>, mut segments: ResMut<SnakeSegments>) {
	segments.0 = vec![
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.head_material.clone(),
				sprite: Sprite::new(Vec2::new(10.0, 10.0)),
				..Default::default()
			})
			.insert(SnakeHead {
				direction: Direction::Up,
			})
			.insert(SnakeSegment)
			.insert(Position {
				x: 3,
				y: 3})
			.insert(Size::square(0.8))
			.id(),
		spawn_segment(commands, &materials.segment_material, Position {
			x: 3,
			y: 2,
		}),
	];

}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
	Input,
	Movement,
	Eating,
	Growth,
}

#[derive(Default)]
pub struct LastTailPosition(Option<Position>);

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
	if let Some(mut head) = heads.iter_mut().next() {
		let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
			Direction::Left
		} else if keyboard_input.pressed(KeyCode::Right) {
			Direction::Right
		} else	if keyboard_input.pressed(KeyCode::Down) {
			Direction::Down
		} else if keyboard_input.pressed(KeyCode::Up) {
			Direction::Up
		} else {
			head.direction
		};
		if dir != head.direction.opposite() {
			head.direction = dir;
		}
	}
}

pub fn snake_movement(
	segments: ResMut<SnakeSegments>,
	mut heads: Query<(Entity, &SnakeHead)>,
	mut positions: Query<&mut Position>,
	mut last_tail_position: ResMut<LastTailPosition>,
	mut game_over_writer: EventWriter<GameOverEvent>) {
	if let Some((head_entity, head)) = heads.iter_mut().next() {
		let segment_positions =
			segments
				.0
				.iter()
				.map(|entity| *positions.get_mut(*entity).unwrap())
				.collect::<Vec<Position>>();

		let mut head_position = positions.get_mut(head_entity).unwrap();

		last_tail_position.0 = Some(*segment_positions.last().unwrap());

		match &head.direction {
			Direction::Left => {
				head_position.x -= 1;
			}
			Direction::Right => {
				head_position.x += 1;
			}
			Direction::Down => {
				head_position.y -= 1;
			}
			Direction::Up => {
				head_position.y += 1;
			}
		}

		if head_position.x < 0 || head_position.x as u32 >= MAP_WIDTH ||
			head_position.y < 0 || head_position.y as u32 >= MAP_HEIGHT {
				game_over_writer.send(GameOverEvent);
		}

		if segment_positions.contains(&head_position) {
			game_over_writer.send(GameOverEvent);
		}

		segment_positions
			.iter()
			.zip(segments.0.iter().skip(1))
			.for_each(|(pos, segment)| {
				*positions.get_mut(*segment).unwrap() = *pos;
			});
	}
}

pub fn spawn_segment(mut commands: Commands, material: &Handle<ColorMaterial>, position: Position) -> Entity {
	commands.spawn_bundle(SpriteBundle {
		material: material.clone(),
		..Default::default()
	})
		.insert(SnakeSegment)
		.insert(position)
		.insert(Size::square(0.65))
		.id()
}

pub fn snake_growth(
	commands: Commands,
	last_tail_position: Res<LastTailPosition>,
	mut segments: ResMut<SnakeSegments>,
	mut growth_reader: EventReader<GrowthEvent>,
	materials: Res<Materials>
) {
	if growth_reader.iter().next().is_some() {
		segments.0.push(spawn_segment(commands, &materials.segment_material, last_tail_position.0.unwrap()));
	}
}
