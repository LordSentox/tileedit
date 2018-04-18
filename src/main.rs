extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate piston_window;
extern crate sdl2_window;
extern crate sprite;

mod ani_sprite;
mod texture_manager;
mod tile_layer_view;

pub type DeviceTexture = Texture<gfx_device_gl::Resources>;

use piston_window::*;
use sdl2_window::Sdl2Window;

use texture_manager::TextureManager;

fn main() {
	let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("TileEdit", (900, 900))
		.opengl(OpenGL::V3_2)
		.build()
		.expect("Could not create piston window");

	window.set_lazy(true);

	let textures = TextureManager::new(&find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("Could not find asset folder"));

	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			clear([1.0, 1.0, 1.0, 1.0], g);
		});
	}
}
