use piston_window::{Flip, Texture, TextureSettings};
use gfx_device_gl::Factory;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use super::DeviceTexture;

pub struct TextureManager {
	root: PathBuf,
	textures: RefCell<HashMap<PathBuf, Weak<DeviceTexture>>>
}

impl TextureManager {
	/// Create a new TextureManager. No resources are loaded by default.
	// TODO: Resources *cannot* be loaded by default. Maybe rethink using weak
	// pointers for this.
	pub fn new<P: AsRef<Path>>(root: P) -> TextureManager {
		TextureManager {
			root: root.as_ref().to_path_buf(),
			textures: RefCell::new(HashMap::new())
		}
	}

	/// Get the texture if it is already loaded, or load it in case it doesn't
	/// exist yet.
	pub fn get<P: AsRef<Path>>(&self, path: P, factory: &mut Factory) -> Rc<DeviceTexture> {
		// Try finding the relative path in the textures already loaded.
		if let Some(weak) = self.textures.borrow().get(path.as_ref()) {
			if let Some(rc) = weak.upgrade() {
				return rc;
			}
		}

		let tex = Rc::new(Texture::from_path(
					factory,
					self.root.join(path.as_ref()),
					Flip::None,
					&TextureSettings::new()
		).expect("Tried to load invalid texture"));

		self.textures.borrow_mut().insert(path.as_ref().to_path_buf(), Rc::downgrade(&tex));
		tex
	}

	/// Clean up textures that are no longer used. The textures themselves are
	/// immediately deallocated when they are no longer needed, but the weak
	/// pointer is still registered.
	pub fn clean(&mut self) {
		self.textures.borrow_mut().retain(|_, v| { v.upgrade().is_some() });
	}
}
