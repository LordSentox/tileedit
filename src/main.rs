#![allow(dead_code)]

extern crate find_folder;
extern crate gfx_device_gl;
extern crate graphics;
extern crate nalgebra;
extern crate piston_window;
extern crate sdl2_window;
extern crate sprite;

mod ani_sprite;
mod texture_manager;
mod tile_layer_view;

pub type DeviceTexture = Texture<gfx_device_gl::Resources>;
pub use nalgebra::Vector2 as Vec2;

use piston_window::*;
use sdl2_window::Sdl2Window;

use ani_sprite::AniSprite;
use texture_manager::TextureManager;
use tile_layer_view::TileLayerView;

fn main() {
	let mut window: PistonWindow<Sdl2Window> = WindowSettings::new("TileEdit", (900, 900))
		.opengl(OpenGL::V3_2)
		.build()
		.expect("Could not create piston window");

	window.set_lazy(true);

	let textures = TextureManager::new(&find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("Could not find asset folder"));

	let mut test_layer_view = TileLayerView::new((5, 5), AniSprite::new(textures.get("tiles.png", &mut window.factory), 64., 64.));

	let tilemap = vec![vec![
		Some(0), Some(1), Some(0), Some(1), Some(0)
	]; 5];

	test_layer_view.clone_map(&tilemap);

	while let Some(e) = window.next() {
		e.mouse_relative(|dx, dy| {
			test_layer_view.mouse_relative(dx, dy);
		});

		e.button(|b| {
			test_layer_view.button(&b);
		});
		
		window.draw_2d(&e, |c, g| {
			clear([1.0, 1.0, 1.0, 1.0], g);

			test_layer_view.draw(c.transform, g);
		});
	}
}
