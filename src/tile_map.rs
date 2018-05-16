use ani_sprite::AniSprite;
use byteorder::{BigEndian, ReadBytesExt};
use rect::Rect;
use std::io;
use std::fs::File;
use std::path::Path;

use super::DeviceTexture;

pub struct TileLayer {
	// XXX: The rows of the TileLayer must all have the same length, the
	// Vec was chosen because of the convenience as compared to a Boxed slice
	tiles: Vec<Vec<Option<u16>>>,
	sprites: AniSprite<DeviceTexture>
}


impl TileLayer {
	/// Create a new empty tile layer. The sprites that will be used are already
	/// required to be set and generally should net be changed later.
	pub fn new((width, height): (usize, usize), sprites: AniSprite<DeviceTexture>) -> TileLayer {
		TileLayer {
			tiles: vec![vec![None; width]; height],
			sprites: sprites
		}
	}

	/// Get the size of this TileLayer
	pub fn size(&self) -> (usize, usize) {
		let height = self.tiles.len();

		if height != 0 { (self.tiles[0].len(), height) }
		else { (0, 0) }
	}
}

/// Represents a whole TileMap comrised of multiple Layers. The rules of the
/// Layers are as follows:
///
/// ## Background:
/// Rendered first, supposed to fill the whole screen at all times
///
/// ## GameLayer:
/// The same Layer the Player is in. The render time is dependent on the Player
/// position
///
/// ## Collisions:
/// Not a real layer, these are unbound objects that can be placed without being
/// bound to a grid and are invisible
///
/// ## Foreground:
/// Always rendered last, so that they overshadow everything
pub struct TileMap {
	background: TileLayer,
	game_layer: TileLayer,
	collisions: Vec<Rect<f64>>,
	foreground: TileLayer
}

impl TileMap {
	/// Create a new TileMap from the provided Layers. The layers have to be the
	/// same size for it to work properly.
	pub fn new(background: TileLayer, game_layer: TileLayer, foreground: TileLayer, collisions: Vec<Rect<f64>>) -> TileMap {
		// Check if the layers are all the same size.
		assert_eq!(background.size(), game_layer.size());
		assert_eq!(game_layer.size(), foreground.size());

		TileMap {
			background: background,
			game_layer: game_layer,
			collisions: collisions,
			foreground: foreground
		}
	}

	/// Read the TileMap information from a file
	pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<TileMap> {
		let file = File::open(path)?;

		let width = file.read_u64::<BigEndian>()?;
		let height = file.read_u64::<BigEndian>()?;

		// Read the names of the sprite files
		
	}

	/// Get the background TileLayer
	pub fn background(&self) -> &TileLayer {
		&self.background
	}

	/// Get the background TileLayer mutably
	pub fn background_mut(&mut self) -> &mut TileLayer {
		&mut self.background
	}

	/// Get the game TileLayer
	pub fn game_layer(&self) -> &TileLayer {
		&self.game_layer
	}

	/// Get the game TileLayer mutably
	pub fn game_layer_mut(&mut self) -> &mut TileLayer {
		&mut self.game_layer
	}

	/// Get the foreground TileLayer
	pub fn foreground(&self) -> &TileLayer {
		&self.foreground
	}

	/// Get the foreground TileLayer mutably
	pub fn foreground_mut(&mut self) -> &mut TileLayer {
		&mut self.foreground
	}
}
