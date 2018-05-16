use std::ops::{Add, AddAssign};
use super::Vec2;
use nalgebra::{Real, Scalar};

/// Represents a Rectangle with the value type T.
#[derive(Copy, Clone, Debug)]
pub struct Rect<T: Scalar> {
	/// The x coordinate, or leftmost coordinate of the Rect.
	pub x: T,
	/// The y coordinate, or rightmost coordinate of the Rect.
	pub y: T,
	/// The width of the Rect.
	pub w: T,
	/// The height of the Rect.
	pub h: T
}

impl<T: Scalar> Rect<T> {
	/// Create a Rectangle from a slice. Indices are [x, y, w, h].
	pub fn from_slice(slice: [T; 4]) -> Rect<T> where T: Copy {
		Rect {
			x: slice[0],
			y: slice[1],
			w: slice[2],
			h: slice[3]
		}
	}

	/// Move by the Vec provided.
	pub fn translate(&mut self, by: Vec2<T>) where T: AddAssign {
		self.x += by.x;
		self.y += by.y;
	}

	/// Test if two rectangles intersect.
	pub fn intersect<'a>(this: &'a Rect<T>, other: &'a Rect<T>) -> bool
		where T: Add<Output = T> + PartialOrd + Copy {
		!(this.x > other.x + other.w || this.x + this.w < other.x ||
		this.y > other.y + other.h || this.y + this.h < other.y)
	}

	/// Check if the point is inside this Rect and return true if so.
	pub fn contains(&self, point: Vec2<T>) -> bool where T: PartialOrd + Add<Output = T> {
		point.x >= self.x && point.x <= self.x + self.w &&
		point.y >= self.y && point.y <= self.y + self.h
	}

	/// Get the shortest way that must be applied to this Rect to clear out of
	/// another Rect of the same type so that they would not intersect any more.
	pub fn shortest_way_out(&self, of: &Rect<T>) -> Vec2<T> where T: Real {
		// Check upwards
		let mut move_y = of.y - self.y - self.h;
		// Check downwards
		let move_down = of.y + of.h - self.y;
		if move_down < -move_y {
			move_y = move_down;
		}

		// Check left
		let mut move_x = of.x - self.x - self.w;
		// Check right
		let move_right = of.x + of.w - self.x;
		if move_right < -move_x {
			move_x = move_right;
		}

		if move_x.abs() < move_y.abs() {
			Vec2::new(move_x, T::zero())
		}
		else {
			Vec2::new(T::zero(), move_y)
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_intersect() {
		let a = Rect::from_slice([0, 0, 4, 4]);
		let b = Rect::from_slice([-1, -1, 1, 1]);

		assert!(Rect::intersect(&a, &b));
	}
}
