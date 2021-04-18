use bevy::prelude::*;

pub const MAP_WIDTH: u32 = 10;
pub const MAP_HEIGHT: u32 = 10;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

pub struct Size {
	pub width: f32,
	pub height: f32,
}

impl Size {
	pub fn square(size: f32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

pub fn size_scaling(windows: Res<Windows>, mut query: Query<(&Size, &mut Sprite)>) {
	let window = windows.get_primary().unwrap();
	for (sprite_size, mut sprite) in query.iter_mut() {
		sprite.size = Vec2::new(
			sprite_size.width / MAP_WIDTH as f32 * window.width() as f32,
			sprite_size.height / MAP_HEIGHT as f32 * window.height() as f32,
		);
	}
}

pub fn position_translation(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
	fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
		let tile_size = bound_window / bound_game;
		pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
	}
	let window = windows.get_primary().unwrap();
	for (pos, mut transform) in query.iter_mut() {
		transform.translation = Vec3::new(
			convert(pos.x as f32, window.width() as f32, MAP_WIDTH as f32),
			convert(pos.y as f32, window.height() as f32, MAP_HEIGHT as f32),
			0.0,
		)
	}
}
