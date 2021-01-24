use std::f32::{NAN, INFINITY};

use crate::consts::EPSILON;

/// A continuous range of scalar values.
/// Can also represent all values and no values.
/// Note that if any of the values are NaN, then the range represents an empty range.
#[derive(Copy, Clone, Debug)]
pub struct Range {
	/// The lower bound.
	min : f32,
	/// The upper bound.
	max : f32,
}

impl Range {
	/// Creates an empty range.
	pub fn empty() -> Range {
		Range { min: NAN, max: NAN }
	}

	/// Creates a range containing a single value.
	pub fn single(value : f32) -> Range {
		Range { min: value, max: value }
	}

	/// Creates a range containing two values and all the values in between.
	pub fn range(bound1 : f32, bound2 : f32) -> Range {
		if bound1 < bound2 {
			Range { min: bound1, max: bound2 }
		} else {
			Range { min: bound2, max: bound1 }
		}
	}

	/// Creates a range over all values.
	pub fn everything() -> Range {
		Range { min: -INFINITY, max: INFINITY }
	}

	/// Whether this is empty.
	pub fn is_empty(&self) -> bool {
		self.min.is_nan() || self.max.is_nan()
	}

	/// The lower bound of the range. Will always be NaN if this range contains no values.
	pub fn min(&self) -> f32 {
		if self.is_empty() { NAN } else { self.min }
	}

	/// The upper bound of the range. Will always be NaN if this range contains no values.
	#[allow(dead_code)]
	pub fn max(&self) -> f32 {
		if self.is_empty() { NAN } else { self.max }
	}

	/// The size of this range.
	#[allow(dead_code)]
	pub fn size(&self) -> f32 {
		if self.is_empty() { 0.0 } else { self.max - self.min }
	}

	/// Finds the range common between two ranges.
	pub fn intersect(&self, other : &Range) -> Range {
		if self.is_empty() || other.is_empty() { return Range::empty(); }
		// Get the biggest min.
		let min = if self.min > other.min { self.min } else { other.min };
		// And the smallest max.
		let max = if self.max < other.max { self.max } else { other.max };
		if max < min { Range::empty() } else { Range{ min, max } }
	}

	/// Finds the range that contains both ranges.
	#[allow(dead_code)]
	pub fn contain(&self, other : &Range) -> Range {
		if self.is_empty() { return *other; }
		if other.is_empty() { return *self; }
		Range{
			// Get the smallest min.
			min: if self.min < other.min { self.min } else { other.min },
			// And the largest max.
			max: if self.max > other.max { self.max } else { other.max },
		}
	}

	/// Creates a range that's got end points at the zeros of a quadratic.
	/// Can also have no end points if the quadratic has no zeros.
	pub fn quadratic_zeros(a : f32, b : f32, c : f32) -> Range {
		if a.abs() < EPSILON {
			// Degenerates to a linear equation.
			if b.abs() < EPSILON {
				// Degenerates to a constant "equation".
				if c < EPSILON { Range::everything() } else { Range::empty() }
			} else {
				Range::single(-c / b)
			}
		} else {
			let mut det = b * b - 4.0 * a * c;
			if det < -EPSILON { // TODO: Could use a relative epsilon to keep things stable even in tiny cases.
				Range::empty()
			} else if det < EPSILON {
				Range::single(-0.5 * b / a)
			} else {
				det = det.sqrt();
				let denom = 2.0 * a;
				Range::range((-b + det) / denom, (-b - det) / denom)
			}
		}
	}

	/// If the other is moving at other_movement, see when the two ranges will overlap.
	pub fn linear_overlap(&self, other : &Range, other_movement : f32) -> Range {
		if other_movement.abs() < EPSILON {
			if self.intersect(other).is_empty() {
				Range::empty()
			} else {
				Range::everything()
			}
		} else {
			Range::range(
				(self.min() - other.min()) / other_movement,
				(self.max() - other.min()) / other_movement,
			).contain(&Range::range(
				(self.min() - other.max()) / other_movement,
				(self.max() - other.max()) / other_movement,
			))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_constructor() {
		{
			let empty = Range::empty();
			assert!(empty.is_empty());
			assert!(empty.min().is_nan());
			assert!(empty.max().is_nan());
			assert_eq!(empty.size(), 0.0);
		}
		{
			let empty = Range::single(NAN);
			assert!(empty.is_empty());
			assert!(empty.min().is_nan());
			assert!(empty.max().is_nan());
			assert_eq!(empty.size(), 0.0);
		}
		{
			let one = Range::single(1.0);
			assert!(!one.is_empty());
			assert_eq!(one.min(), 1.0);
			assert_eq!(one.max(), 1.0);
			assert_eq!(one.size(), 0.0);
		}
		{
			let range = Range::range(-1.0, 2.0);
			assert!(!range.is_empty());
			assert_eq!(range.min(), -1.0);
			assert_eq!(range.max(),  2.0);
			assert_eq!(range.size(), 3.0);
		}
		{
			let everything = Range::everything();
			assert!(!everything.is_empty());
			assert_eq!(everything.min(), -INFINITY);
			assert_eq!(everything.max(),  INFINITY);
			assert_eq!(everything.size(), INFINITY);
		}
	}

	#[test]
	fn check_intersect() {
		{ // Verify that one empty should always lead to empty
			let empty = Range::empty();
			let single = Range::single(1.0);
			assert!(empty.intersect(&single).is_empty());
			assert!(single.intersect(&empty).is_empty());
		}

		{ // Verify that can get the lower and upper bounds from either.
			let lower = Range::range(-2.0, 1.0);
			let upper = Range::range(2.0, -1.0);
			{
				let lower_upper = lower.intersect(&upper);
				assert_eq!(lower_upper.min(), -1.0);
				assert_eq!(lower_upper.max(),  1.0);
			}
			{
				let upper_lower = upper.intersect(&lower);
				assert_eq!(upper_lower.min(), -1.0);
				assert_eq!(upper_lower.max(),  1.0);
			}
		}

		{ // Verify that disjoint values won't result in a range.
			let lower = Range::range(-2.0, -1.0);
			let upper = Range::range(2.0, 1.0);
			assert!(lower.intersect(&upper).is_empty());
			assert!(upper.intersect(&lower).is_empty());
		}
	}

	#[test]
	fn check_contain() {
		{ // Verify that one everything should always lead to empty
			let empty = Range::empty();
			let single = Range::single(1.0);
			{
				let first_empty = empty.contain(&single);
				assert_eq!(first_empty.min(), 1.0);
				assert_eq!(first_empty.max(), 1.0);
			}
			{
				let second_empty = single.contain(&empty);
				assert_eq!(second_empty.min(), 1.0);
				assert_eq!(second_empty.max(), 1.0);
			}
			{
				let both_empty = empty.contain(&empty);
				assert!(both_empty.is_empty());
			}
		}

		{ // Verify that can get the lower and upper bounds from either.
			let lower = Range::range(-2.0, 1.0);
			let upper = Range::range(2.0, -1.0);
			{
				let lower_upper = lower.contain(&upper);
				assert_eq!(lower_upper.min(), -2.0);
				assert_eq!(lower_upper.max(),  2.0);
			}
			{
				let upper_lower = upper.contain(&lower);
				assert_eq!(upper_lower.min(), -2.0);
				assert_eq!(upper_lower.max(),  2.0);
			}
		}
	}

	#[test]
	fn check_quadratic() {
		{ // (2x - 1) * (x - 3) = 2x^2 - 7x + 3
			let zeros = Range::quadratic_zeros(2.0, -7.0, 3.0);
			assert!((zeros.min() - 0.5).abs() < EPSILON);
			assert!((zeros.max() - 3.0).abs() < EPSILON);
		}
		{ // (x + 2) * (x + 2) = x^2 + 4x + 4
			let zeros = Range::quadratic_zeros(1.0, 4.0, 4.0);
			assert!((zeros.min() - -2.0).abs() < EPSILON);
			assert!((zeros.max() - -2.0).abs() < EPSILON);
		}
		{ // (x + i) * (x - i) = x^2 + 1
			let zeros = Range::quadratic_zeros(1.0, 0.0, 1.0);
			assert!(zeros.is_empty());
		}
	}

	#[test]
	fn check_quadratic_degenrate() {
		{ // 0 = 0
			let zeros = Range::quadratic_zeros(0.0, 0.0, 0.0);
			assert!(zeros.min() <= -INFINITY);
			assert!(zeros.max() >=  INFINITY);
		}
		{ // 0 = 1
			let zeros = Range::quadratic_zeros(0.0, 0.0, 1.0);
			assert!(zeros.is_empty());
		}
		{ // 0 = x + 2
			let zeros = Range::quadratic_zeros(0.0, 1.0, 2.0);
			assert!((zeros.min() - -2.0).abs() < EPSILON);
			assert!((zeros.max() - -2.0).abs() < EPSILON);
		}
	}
}
