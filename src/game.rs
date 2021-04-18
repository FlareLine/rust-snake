use bevy::prelude::*;

use crate::{food::Food, snake::{Materials, SnakeSegment, SnakeSegments, spawn_snake}};

pub struct GameOverEvent;

pub fn game_over(
	mut commands: Commands,
	mut reader: EventReader<GameOverEvent>,
	materials: Res<Materials>,
	segments_res: ResMut<SnakeSegments>,
	food: Query<Entity, With<Food>>,
	segments: Query<Entity, With<SnakeSegment>>,
) {
	if reader.iter().next().is_some() {
		for entity in food.iter().chain(segments.iter()) {
			commands.entity(entity).despawn();
		}
		spawn_snake(commands, materials, segments_res);
	}
}
