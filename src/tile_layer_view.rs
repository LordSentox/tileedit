use ani_sprite::AniSprite;
use graphics::{Graphics, math::Matrix2d};
use piston_window::{Button, ButtonArgs, ButtonState, MouseButton};
use super::DeviceTexture;
use std::cell::RefCell;
use std::u16;
use super::Vec2;

pub type Tilemap = Vec<Vec<Option<u16>>>;

pub struct TileLayerView {
	pos: Vec2<f64>,
	tilemap: Tilemap,
	tiles: RefCell<AniSprite<DeviceTexture>>,
	grabbed: bool
}

impl TileLayerView {
	/// Create a new TileLayerView with the specified size. The tiles are in
	/// form of an AniSprite.
	pub fn new((width, height): (usize, usize), tiles: AniSprite<DeviceTexture>) -> TileLayerView {
		TileLayerView {
			pos: Vec2::new(0., 0.),
			tilemap: vec![vec![None; width]; height],
			tiles: RefCell::new(tiles),
			grabbed: false
		}
	}

	/// Set the tile at position (x, y) to the value of to. Update without
	/// buffering.
	pub fn set_tile(&mut self, (x, y): (usize, usize), to: Option<u16>) {
		self.tilemap[x][y] = to;
	}

	/// Clone the tilemap information to this map. The individual values have to
	/// be copied over.
	pub fn clone_map(&mut self, map: &Tilemap) {
		self.tilemap = map.clone();
	}

	/// When the mouse is moved by a relative amount this has to be called to
	/// keep the map at the correct place and whatnot.
	pub fn mouse_relative(&mut self, dx: f64, dy: f64) {
		if self.grabbed {
			self.pos += Vec2::new(dx, dy);
		}
	}

	/// Called when a button is pressed. All shortcuts that are applied to this
	/// TileLayerView can be handled in this function.
	pub fn button(&mut self, b: &ButtonArgs) {
		// Toggle wether or not the map is currently being dragged
		if let Button::Mouse(MouseButton::Middle) = b.button {
			self.grabbed = b.state == ButtonState::Press;
		}
	}

	/// Draw the TileLayerView to a Texture (unbuffered; every tile is drawn on
	/// its own, so it's quite slow.
	pub fn draw<B: Graphics<Texture=DeviceTexture>>(&self, t: Matrix2d, b: &mut B) {
		let (width, height) = (self.tiles.borrow().frame_width(), self.tiles.borrow().frame_width());
		let mut tiles = self.tiles.borrow_mut();

		for (x, row) in self.tilemap.iter().enumerate() {
			for (y, e) in row.iter().enumerate() {
					if let Some(e) = *e {
					tiles.set_frame(e as u32);
					tiles.set_position(self.pos.x + x as f64*width, self.pos.y + y as f64*height);

					tiles.draw(t, b);
				}
			}
		}
	}
}
