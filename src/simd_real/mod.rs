// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Derivative work of `core::simd` licensed under `MIT OR Apache-2.0`.

use super::{Real, SimdBits, SimdMask};
use core::simd::{LaneCount, Mask, Select, Simd, SupportedLaneCount, Swizzle, Swizzle2};
use core::{
	fmt::{Debug, LowerExp, UpperExp},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
	ops::{Index, IndexMut},
};

mod f32;
mod f64;

pub use core::simd::Which;

/// Constructs vector by selecting values from the lanes of one or two source vectors.
///
/// When swizzling one vector, the indices of the result vector are indicated by a `const` array of
/// `usize`. When swizzling two vectors, the indices are indicated by a `const` array of [`Which`].
///
/// [`Which`]: `core::simd::Which`
#[macro_export]
macro_rules! swizzle {
	{
		$vector:expr, $index:expr $(,)?
	} => {
		{
			use core::simd::Swizzle;
			struct Impl;
			impl<const LANES: usize> Swizzle<LANES, {$index.len()}> for Impl {
				const INDEX: [usize; {$index.len()}] = $index;
			}
			$vector.swizzle::<Impl>()
		}
	};
	{
		$first:expr, $second:expr, $index:expr $(,)?
	} => {
		{
			use core::simd::{Which, Swizzle2};
			struct Impl;
			impl<const LANES: usize> Swizzle2<LANES, {$index.len()}> for Impl {
				const INDEX: [Which; {$index.len()}] = $index;
			}
			$first.swizzle2::<Impl>($second)
		}
	}
}

/// Real number vector of [`Simd<f32, LANES>`] or [`Simd<f64, LANES>`] with associated [`SimdBits`]
/// and [`SimdMask`] vector.
///
/// [`Simd<f32, LANES>`]: `core::simd::Simd`
/// [`Simd<f64, LANES>`]: `core::simd::Simd`
pub trait SimdReal<R: Real, const LANES: usize>
where
	Self: Clone + Copy + Default,
	Self: PartialEq + PartialOrd,
	Self: Debug + LowerExp + UpperExp,
	Self: From<[R; LANES]> + AsRef<[R; LANES]> + AsMut<[R; LANES]>,
	Self: Index<usize, Output = R> + IndexMut<usize, Output = R>,
	Self: Select<Self::Mask>,
	Self: Add<Output = Self> + AddAssign + Add<R, Output = Self> + AddAssign<R>,
	Self: Sub<Output = Self> + SubAssign + Sub<R, Output = Self> + SubAssign<R>,
	Self: Mul<Output = Self> + MulAssign + Mul<R, Output = Self> + MulAssign<R>,
	Self: Div<Output = Self> + DivAssign + Div<R, Output = Self> + DivAssign<R>,
	Self: Rem<Output = Self> + RemAssign + Rem<R, Output = Self> + RemAssign<R>,
	Self: Neg<Output = Self>,
{
	/// Associated bits representation vector.
	type Bits: SimdBits<R::Bits, LANES, Mask = Self::Mask>;
	/// Associated mask vector.
	type Mask: SimdMask<LANES>;

	/// Number of lanes in this vector.
	const LANES: usize = LANES;

	/// Get the number of lanes in this vector.
	fn lanes(&self) -> usize {
		LANES
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	fn splat(value: R) -> Self;

	/// Returns an array reference containing the entire SIMD vector.
	fn as_array(&self) -> &[R; LANES];
	/// Returns a mutable array reference containing the entire SIMD vector.
	fn as_mut_array(&mut self) -> &mut [R; LANES];
	/// Converts an array to a SIMD vector.
	fn from_array(array: [R; LANES]) -> Self;
	/// Converts a SIMD vector to an array.
	fn to_array(self) -> [R; LANES];

	/// Converts a slice to a SIMD vector containing `slice[..LANES]`
	///
	/// # Panics
	///
	/// Panics if the slice's `len` is less than the vector's `Simd::LANES`.
	fn from_slice(slice: &[R]) -> Self;

	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// If an index is out-of-bounds, the lane is instead selected from the `or` vector.
	fn gather_or(slice: &[R], idxs: Simd<usize, LANES>, or: Self) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount;
	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// If an index is out-of-bounds, the lane is set to the default value for the type.
	fn gather_or_default(slice: &[R], idxs: Simd<usize, LANES>) -> Self
	where
		R: Default,
		LaneCount<LANES>: SupportedLaneCount;
	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// The mask `enable`s all `true` lanes and disables all `false` lanes.
	/// If an index is disabled or is out-of-bounds, the lane is selected from the `or` vector.
	fn gather_select(
		slice: &[R],
		enable: Mask<isize, LANES>,
		idxs: Simd<usize, LANES>,
		or: Self,
	) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount;
	/// Writes the values in a SIMD vector to potentially discontiguous indices in `slice`.
	///
	/// If two lanes in the scattered vector would write to the same index only the last lane is
	/// guaranteed to actually be written.
	fn scatter(self, slice: &mut [R], idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount;
	/// Writes the values in a SIMD vector to multiple potentially discontiguous indices in `slice`.
	///
	/// The mask `enable`s all `true` lanes and disables all `false` lanes. If an enabled index is
	/// out-of-bounds, the lane is not written. If two enabled lanes in the scattered vector would
	/// write to the same index, only the last lane is guaranteed to actually be written.
	fn scatter_select(self, slice: &mut [R], enable: Mask<isize, LANES>, idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount;

	/// Raw transmutation from an unsigned integer vector type with the same size and number of
	/// lanes.
	fn from_bits(bits: Self::Bits) -> Self;
	/// Raw transmutation to an unsigned integer vector type with the same size and number of lanes.
	fn to_bits(self) -> Self::Bits;

	/// Inserts `value` at lane selected by `index`.
	fn insert(mut self, value: R, index: usize) -> Self {
		self[index] = value;
		self
	}

	/// Horizontal wrapping add. Returns the sum of the lanes of the vector, with wrapping addition.
	fn horizontal_sum(self) -> R;
	/// Horizontal wrapping multiply. Returns the product of the lanes of the vector, with wrapping
	/// multiplication.
	fn horizontal_product(self) -> R;
	/// Horizontal minimum.  Returns the minimum lane in the vector.
	///
	/// Returns values based on equality, so a vector containing both `0.0` and `-0.0` may return
	/// either. This function will not return NaN unless all lanes are NaN.
	fn horizontal_min(self) -> R;
	/// Horizontal maximum.  Returns the maximum lane in the vector.
	///
	/// Returns values based on equality, so a vector containing both `0.0` and `-0.0` may return
	/// either. This function will not return NaN unless all lanes are NaN.
	fn horizontal_max(self) -> R;

	/// Reverse the order of the lanes in the vector.
	fn reverse(self) -> Self;
	/// Rotates the vector such that the first `OFFSET` elements of the slice move to the end while
	/// the last `Self::LANES - OFFSET` elements move to the front. The element previously in lane
	/// `OFFSET` will become the first element in the slice.
	fn rotate_lanes_left<const OFFSET: usize>(self) -> Self;
	/// Rotates the vector such that the first `Self::LANES - OFFSET` elements of the vector move to
	/// the end while the last `OFFSET` elements move to the front. The element previously at index
	/// `Self::LANES - OFFSET` will become the first element in the slice.
	fn rotate_lanes_right<const OFFSET: usize>(self) -> Self;
	/// Interleaves two vectors.
	///
	/// Produces two vectors with lanes taken alternately from `self` and `other`.
	///
	/// The first result contains the first `Self::LANES / 2` lanes from `self` and `other`,
	/// alternating, starting with the first lane of `self`.
	///
	/// The second result contains the last `Self::LANES / 2` lanes from `self` and `other`,
	/// alternating, starting with the lane `Self::LANES / 2` from the start of `self`.
	fn interleave(self, other: Self) -> (Self, Self);
	/// Deinterleaves two vectors.
	///
	/// The first result takes every other lane of `self` and then `other`, starting with the first
	/// lane.
	///
	/// The second result takes every other lane of `self` and then `other`, starting with the
	/// second lane.
	fn deinterleave(self, other: Self) -> (Self, Self);

	/// Creates a new vector by selecting values from the lanes of `self`.
	fn swizzle<T: Swizzle<LANES, LANES>>(self) -> Self;
	/// Creates a new vector by selecting values from the lanes of `self` and `other`.
	fn swizzle2<T: Swizzle2<LANES, LANES>>(self, other: Self) -> Self;

	/// Tests for approximate equality wrt `epsilon` or `ulp`, "or" in the sense of `||`.
	fn approx_eq(self, other: Self, epsilon: R, ulp: R::Bits) -> bool {
		self.lanes_approx_eq(other, Self::splat(epsilon), Self::Bits::splat(ulp))
			.all()
	}
	/// Tests for approximate inequality wrt `epsilon` or `ulp`, "and" in the sense of `&&`.
	fn approx_ne(self, other: Self, epsilon: R, ulp: R::Bits) -> bool {
		self.lanes_approx_ne(other, Self::splat(epsilon), Self::Bits::splat(ulp))
			.any()
	}

	/// Tests lanes for approximate equality wrt `epsilon` or `ulp`, "or" in the sense of `||`.
	fn lanes_approx_eq(self, other: Self, epsilon: Self, ulp: Self::Bits) -> Self::Mask {
		let (self_bits, other_bits) = (self.to_bits(), other.to_bits());
		(self - other).abs().lanes_le(epsilon)
			| !(self.is_nan() | other.is_nan())
				& !(self.is_sign_negative() ^ other.is_sign_negative())
				& self_bits.abs_sub(other_bits).lanes_le(ulp)
	}
	/// Tests lanes for approximate inequality wrt `epsilon` and `ulp`, "and" in the sense of `&&`.
	fn lanes_approx_ne(self, other: Self, epsilon: Self, ulp: Self::Bits) -> Self::Mask {
		self.lanes_approx_eq(other, epsilon, ulp)
	}

	/// Test if each lane is equal to the corresponding lane in `other`.
	fn lanes_eq(self, other: Self) -> Self::Mask;
	/// Test if each lane is not equal to the corresponding lane in `other`.
	fn lanes_ne(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than the corresponding lane in `other`.
	fn lanes_lt(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than the corresponding lane in `other`.
	fn lanes_gt(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than or equal to the corresponding lane in `other`.
	fn lanes_le(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than or equal to the corresponding lane in `other`.
	fn lanes_ge(self, other: Self) -> Self::Mask;

	/// Returns true for each lane if it has a positive sign, including `+0.0`, NaNs with positive
	/// sign bit and positive infinity.
	fn is_sign_positive(self) -> Self::Mask;
	/// Returns true for each lane if it has a negative sign, including `-0.0`, NaNs with negative
	/// sign bit and negative infinity.
	fn is_sign_negative(self) -> Self::Mask;
	/// Returns true for each lane if its value is NaN.
	fn is_nan(self) -> Self::Mask;
	/// Returns true for each lane if its value is positive infinity or negative infinity.
	fn is_infinite(self) -> Self::Mask;
	/// Returns true for each lane if its value is neither infinite nor NaN.
	fn is_finite(self) -> Self::Mask;
	/// Returns true for each lane if its value is subnormal.
	fn is_subnormal(self) -> Self::Mask;
	/// Returns true for each lane if its value is neither neither zero, infinite, subnormal, or
	/// NaN.
	fn is_normal(self) -> Self::Mask;

	/// Produces a vector where every lane has the absolute value of the equivalently-indexed lane
	/// in `self`.
	fn abs(self) -> Self;
	/// Replaces each lane with a number that represents its sign.
	///
	///   * returns `1.0` if the number is positive, `+0.0`, or [`Real::INFINITY`].
	///   * returns `-1.0` if the number is negative, `-0.0`, or [`Real::NEG_INFINITY`].
	///   * returns [`Real::NAN`] if the number is NaN.
	fn signum(self) -> Self;
	/// Returns each lane with the magnitude of `self` and the sign of `sign`.
	///
	/// If any lane is a [`Real::NAN`], then a [`Real::NAN`] with the sign of `sign` is returned.
	fn copysign(self, sign: Self) -> Self;
	/// Returns the minimum of each lane.
	///
	/// If one of the values is [`Real::NAN`], then the other value is returned.
	fn min(self, other: Self) -> Self;
	/// Returns the maximum of each lane.
	///
	/// If one of the values is [`Real::NAN`], then the other value is returned.
	fn max(self, other: Self) -> Self;
	/// Restrict each lane to a certain interval unless it is NaN.
	///
	/// For each lane in `self`, returns the corresponding lane in `max` if the lane is
	/// greater than `max`, and the corresponding lane in `min` if the lane is less
	/// than `min`.  Otherwise returns the lane in `self`.
	fn clamp(self, min: Self, max: Self) -> Self;

	/// Takes the reciprocal (inverse) of each lane, ${1 \over x}$.
	fn recip(self) -> Self;

	/// Converts each lane from radians to degrees.
	fn to_degrees(self) -> Self;
	/// Converts each lane from degrees to radians.
	fn to_radians(self) -> Self;

	/// Converts an array to a SIMD vector mask.
	fn mask_from_array(array: [bool; LANES]) -> Self::Mask {
		Self::Mask::from_array(array)
	}
}
