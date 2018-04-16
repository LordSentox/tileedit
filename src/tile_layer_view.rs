use ani_sprite::AniSprite;
use graphics::{Graphics, math::Matrix2d};
use super::DeviceTexture;
use std::cell::RefCell;
use std::u16;

pub type Tilemap = Vec<Vec<Option<u16>>>;

pub struct TileLayerView {
	tilemap: Tilemap,
	tiles: RefCell<AniSprite<DeviceTexture>>
}

impl TileLayerView {
	pub fn new((width, height): (usize, usize), tiles: AniSprite<DeviceTexture>) -> TileLayerView {
		TileLayerView {
			tilemap: vec![vec![None; width]; height],
			tiles: RefCell::new(tiles)
		}
	}

	pub fn set_tile(&mut self, (x, y): (usize, usize), to: Option<u16>) {
		self.tilemap[x][y] = to;
	}

	pub fn clone_map(&mut self, map: &Tilemap) {
		self.tilemap = map.clone();
	}

	pub fn draw<B: Graphics<Texture=DeviceTexture>>(&self, t: Matrix2d, b: &mut B) {
		let (width, height) = (self.tiles.borrow().frame_width(), self.tiles.borrow().frame_width());
		let mut tiles = self.tiles.borrow_mut();

		for (x, row) in self.tilemap.iter().enumerate() {
			for (y, e) in row.iter().enumerate() {
					if let Some(e) = *e {
					tiles.set_frame(e as u32);
					tiles.set_position(x as f64*width, y as f64*height);

					tiles.draw(t, b);
				}
			}
		}
	}
}
