use bevy::prelude::*;
use rand::random;

use crate::map::{MAP_HEIGHT, MAP_WIDTH, Position, Size};
use crate::snake::*;

pub struct Food;
pub struct GrowthEvent;

pub fn food_spawner(mut commands: Commands,	materials: Res<Materials>) {
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.food_material.clone(),
			..Default::default()
		})
		.insert(Food)
		.insert(Position {
			x: (random::<f32>() * MAP_WIDTH as f32) as i32,
			y: (random::<f32>() * MAP_HEIGHT as f32) as i32,
		})
		.insert(Size::square(0.8));
}

pub fn snake_eat(
	mut commands: Commands,
	mut growth_writer: EventWriter<GrowthEvent>,
	food_positions: Query<(Entity, &Position), With<Food>>,
	head_positions: Query<&Position, With<SnakeHead>>,
) {
	for head_position in head_positions.iter() {
		for (entity, food_position) in food_positions.iter() {
			if food_position == head_position {
				commands.entity(entity).despawn();
				growth_writer.send(GrowthEvent);
			}
		}
	}
}
