use sprite::Sprite;
use graphics::types::SourceRectangle;
use graphics::ImageSize;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::cmp;

const X: usize = 0;
const Y: usize = 1;
const WIDTH: usize = 2;
const HEIGHT: usize = 3;

/// Contains the necessary information to semi-automatically animate the
/// AniSprite.
#[allow(missing_docs)]
#[derive(Clone)]
pub struct Animation {
	pub paused: bool,
	pub looping: bool,
	/// The frame this animation starts with
	pub from: u32,
	/// The last frame of the animation, or the frame it wraps around again in
	/// case of looping
	pub to: u32,
	pub fps: f32,
	cur_frame: u32,
	t: f64
}

impl Animation {
	/// Create a new animation that starts running as soon as it is assigned to
	/// an AniSprite.
	pub fn new_immediate(from: u32, to: u32, fps: f32, looping: bool) -> Animation {
		Animation {
			paused: false,
			looping: looping,
			from: from,
			to: to,
			fps: fps,
			cur_frame: from,
			t: 0.
		}
	}

	/// Create a new animation that is not supposed to automatically start
	/// running, but is used to be preemtively assigned to an AniSprite.
	pub fn new_dormant(from: u32, to: u32, fps: f32, looping: bool) -> Animation {
		Animation {
			paused: true,
			looping: looping,
			from: from,
			to: to,
			fps: fps,
			cur_frame: from,
			t: 0.
		}
	}

	/// Reset the Animation to its starting point.
	pub fn reset(&mut self) {
		self.t = 0.;
		self.cur_frame = self.from;
	}

	/// Returns true if the animation is finished. The last frame must have been
	/// reached, but must not have been shown in its full duration for the
	/// function to return true.
	pub fn finished(&self) -> bool {
		if self.looping {
			false
		}
		else {
			self.cur_frame == self.to
		}
	}

	/// Like finished, but the last frame must have been shown for the complete
	/// duration it was assigned to.
	pub fn finished_with_last(&self) -> bool {
		if self.looping {
			false
		}
		else {
			(self.t as f32 * self.fps) as i32 >= (self.to as i32 - self.from as i32).abs() + 1
		}
	}

	/// Update the animation by the given time. If the frame changes, it is
	/// returned. If the frame stays the same, None is returned.
	pub fn update(&mut self, dt: f64) -> Option<u32> {
		if self.paused || self.finished_with_last() {
			return None;
		}

		self.t += dt;

		// The number of the frame that is reached. This must be adjusted by
		// taking the from and to frame into account.
		let frame_num = (self.t as f32 * self.fps) as u32;

		if self.looping {
			// The frame number, adjusted to the relative number of the loop
			// instance that is currently running.
			let ranged_frame = frame_num % ((self.to as i32 - self.from as i32).abs() as u32 + 1);

			// Add or substract the ranged_fram, depending on the animation
			// direction.
			let new_frame = if self.to >= self.from {
				self.from + ranged_frame
			} else {
				self.from - ranged_frame
			};

			// Update the frame
			if new_frame != self.cur_frame {
				self.cur_frame = new_frame;
				Some(new_frame)
			}
			else {
				None
			}
		}
		else {
			// Add or substract the frame_num depending on the direction to get
			// the absolute frame.
			let mut new_frame = if self.to >= self.from {
				self.from + frame_num
			} else {
				self.from - frame_num
			};

			// Since we are not looping, make sure we stay in animation bounds.
			if new_frame > cmp::max(self.from, self.to) {
				new_frame = cmp::max(self.from, self.to);
			}
			else if new_frame < cmp::min(self.from, self.to) {
				new_frame = cmp::min(self.from, self.to);
			}

			// Update the frame
			if new_frame != self.cur_frame {
				self.cur_frame = new_frame;
				Some(new_frame)
			}
			else {
				None
			}
		}
	}

}

/// Represents a Sprite that is animated using frame strips in the texture.
pub struct AniSprite<I: ImageSize> {
	sprite: Sprite<I>,
	frame: SourceRectangle,
	animations: Vec<Animation>,
	looping: bool,
	cur_anim: usize
}

impl<I: ImageSize> AniSprite<I> {
	/// Create a new animated sprite. Like Sprite::from_texture(), but the basic
	/// frame parameters have to be provided. These currently cannot be changed
	/// later.
	pub fn new(tex: Rc<I>, frame_w: f64, frame_h: f64) -> AniSprite<I> {
		// Assure that the texture has a size that can be split using the frame
		// width and height that were provided.
		if tex.get_width() % frame_w as u32 != 0 ||
			tex.get_height() % frame_h as u32 != 0 {
			panic!("Inappropriate frame size. Size of texture must be divisible by frame size.");
		}

		AniSprite {
			sprite: Sprite::from_texture_rect(tex, [0., 0., frame_w, frame_h]),
			frame: [0., 0., frame_w, frame_h],
			animations: Vec::new(),
			looping: false,
			cur_anim: 0
		}
	}

	/// The number of frame rows calculated through height of the texture and
	/// the frame height.
	pub fn num_rows(&self) -> u32 {
		self.sprite.get_texture().get_height() / self.frame[HEIGHT] as u32
	}

	/// The number of frame colums calculated through width of the texture and
	/// the frame width.
	pub fn num_cols(&self) -> u32 {
		self.sprite.get_texture().get_width() / self.frame[WIDTH] as u32
	}

	/// The total amaunt of frames this AniSprite has.
	pub fn num_frames(&self) -> u32 {
		self.num_rows() * self.num_cols()
	}

	/// Get the frame height
	pub fn frame_height(&self) -> f64 {
		self.frame[HEIGHT]
	}

	/// Get the frame width
	pub fn frame_width(&self) -> f64 {
		self.frame[WIDTH]
	}

	/// Manually set the current frame. The frames are numbered left to right
	/// and top to bottom like
	/// 0 1 2
	/// 3 4 5
	/// for an imaginary set consisting of six frames.
	pub fn set_frame(&mut self, num: u32) {
		assert!(num < self.num_frames());

		let num_cols = self.num_cols();
		self.frame[X] = (num % num_cols) as f64 * self.frame[WIDTH];
		self.frame[Y] = (num / num_cols) as f64 * self.frame[HEIGHT];

		self.sprite.set_src_rect(self.frame);
	}

	/// Update the current animation of the sprite. Switches animations when an
	/// animation is finished or does nothing in case there is no animation left
	/// to run.
	pub fn update(&mut self, dt: f64) {
		if self.animations.len() == 0 {
			return;
		}

		assert!(self.cur_anim < self.animations.len());

		if let Some(f) = self.animations[self.cur_anim].update(dt) {
			self.set_frame(f);
			return;
		}

		let mut animation_changed = false;

		// Check if we should play the next animation.
		if self.animations[self.cur_anim].finished_with_last() {
			self.cur_anim += 1;
			animation_changed = true;
		}

		if self.cur_anim == self.animations.len() {
			if self.looping {
				self.cur_anim = 0;
			}
			else {
				self.cur_anim = self.animations.len() - 1;
				animation_changed = true;
			}
		}

		if animation_changed {
			self.animations[self.cur_anim].reset();
			let start_frame = self.animations[self.cur_anim].from;
			self.set_frame(start_frame);
		}
	}

	/// Give this AniSprite a new set of animations. If an animation is
	/// currently playing, it is immediately stopped.
	pub fn animate_fresh(&mut self, animations: Vec<Animation>, looping: bool) {
		self.animations = animations;
		self.looping = looping;
		self.cur_anim = 0;
	}

	/// Get the animation currently being executed on this AniSprite (if any).
	pub fn current_animation(&self) -> Option<&Animation> {
		if self.animations.len() == 0 {
			None
		}
		else {
			Some(&self.animations[self.cur_anim])
		}
	}

	/// Resume the current animation if it was paused.
	/// Returns true if there is an animation to run and it has been started
	/// successfully, false otherwise.
	pub fn resume(&mut self) -> bool {
		if self.animations.len() == 0 {
			return false;
		}

		self.animations[self.cur_anim].paused = false;
		true
	}

	/// Pause the animation that is currently running.
	/// Returns true if there is an animation available. The animation does not
	/// have to be running for it to return true.
	pub fn pause(&mut self) -> bool {
		if self.animations.len() == 0 {
			return false;
		}

		self.animations[self.cur_anim].paused = true;
		true
	}
}

impl<I: ImageSize> Deref for AniSprite<I> {
	type Target = Sprite<I>;

	fn deref(&self) -> &Sprite<I> {
		&self.sprite
	}
}

impl<I: ImageSize> DerefMut for AniSprite<I> {
	fn deref_mut(&mut self) -> &mut Sprite<I> {
		&mut self.sprite
	}
}
