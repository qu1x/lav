// Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Derivative work of `core::simd` licensed under `MIT OR Apache-2.0`.

use super::{ApproxEq, Real, Select, SimdBits, SimdMask};
use core::{
	fmt::Debug,
	iter::{Product, Sum},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
	ops::{Index, IndexMut},
	simd::{LaneCount, Mask, Simd, SupportedLaneCount, Swizzle},
};

mod f32;
mod f64;

/// Constructs vector by copying lanes from selected lanes of one or two input vectors.
///
/// When swizzling one vector, lanes are selected like [`Swizzle::swizzle`].
///
/// When swizzling two vectors, lanes are selected like [`Swizzle::concat_swizzle`].
#[allow(unused_macros)]
pub macro swizzle {
	(
		$vector:expr, $index:expr $(,)?
	) => {
		{
			use core::simd::Swizzle;
			struct Impl;
			impl Swizzle<{$index.len()}> for Impl {
				const INDEX: [usize; {$index.len()}] = $index;
			}
				$vector.swizzle::<Impl>()
		}
	},
	(
		$first:expr, $second:expr, $index:expr $(,)?
	) => {
		{
			use core::simd::Swizzle;
			struct Impl;
			impl Swizzle<{$index.len()}> for Impl {
				const INDEX: [usize; {$index.len()}] = $index;
			}
				$first.concat_swizzle::<Impl>($second)
		}
	}
}

/// Real number vector of [`Simd<f32, N>`] or [`Simd<f64, N>`] with associated [`SimdBits`]
/// and [`SimdMask`] vector.
///
/// [`Simd<f32, N>`]: `core::simd::Simd`
/// [`Simd<f64, N>`]: `core::simd::Simd`
#[allow(clippy::len_without_is_empty)]
pub trait SimdReal<R: Real, const N: usize>
where
	LaneCount<N>: SupportedLaneCount,
	Self: Send + Sync + Clone + Copy + Default,
	Self: ApproxEq<R, Self> + PartialEq + PartialOrd,
	Self: Debug,
	Self: From<Simd<R, N>> + Into<Simd<R, N>>,
	Self: From<[R; N]> + Into<[R; N]>,
	Self: AsRef<[R; N]> + AsMut<[R; N]>,
	Self: Product<Self> + Sum<Self>,
	for<'a> Self: Product<&'a Self> + Sum<&'a Self>,
	Self: Index<usize, Output = R> + IndexMut<usize, Output = R>,
	Self: Select<Self::Mask>,
	Self: Add<Output = Self> + AddAssign,
	Self: Sub<Output = Self> + SubAssign,
	Self: Mul<Output = Self> + MulAssign,
	Self: Div<Output = Self> + DivAssign,
	Self: Rem<Output = Self> + RemAssign,
	for<'a> Self: Add<&'a Self, Output = Self> + AddAssign<&'a Self>,
	for<'a> Self: Sub<&'a Self, Output = Self> + SubAssign<&'a Self>,
	for<'a> Self: Mul<&'a Self, Output = Self> + MulAssign<&'a Self>,
	for<'a> Self: Div<&'a Self, Output = Self> + DivAssign<&'a Self>,
	for<'a> Self: Rem<&'a Self, Output = Self> + RemAssign<&'a Self>,
	Self: Neg<Output = Self>,
{
	/// Associated bits representation vector.
	type Bits: SimdBits<R::Bits, N, Mask = Self::Mask>;
	/// Associated mask vector.
	type Mask: SimdMask<N>;

	/// Number of lanes in this vector.
	const N: usize = N;

	/// Get the number of lanes in this vector.
	#[must_use]
	#[inline]
	fn len(&self) -> usize {
		N
	}

	/// Constructs a SIMD vector by setting all lanes to the given value.
	#[must_use]
	fn splat(value: R) -> Self;

	/// Split a slice into a prefix, a middle of aligned SIMD vectors, and a suffix.
	///
	/// You're only assured that `slice.len() == prefix.len() + middle.len() * N + suffix.len()`.
	///
	/// Notably, all of the following are possible:
	///
	///   * `prefix.len() >= N`,
	///   * `middle.is_empty()` despite `slice.len() >= 3 * N`,
	///   * `suffix.len() >= N`.
	///
	/// That said, this is a safe method, so if you're only writing safe code, then this can at most
	/// cause incorrect logic, not unsoundness.
	///
	/// # Panics
	///
	/// Panic if the size of the SIMD vector is different from `N` times that of the scalar.
	#[must_use]
	fn as_simd(slice: &[R]) -> (&[R], &[Self], &[R]);

	/// Split a mutable slice into a mutable prefix, a middle of aligned SIMD vectors, and a mutable
	/// suffix.
	///
	/// You're only assured that `slice.len() == prefix.len() + middle.len() * N + suffix.len()`.
	///
	/// Notably, all of the following are possible:
	///
	///   * `prefix.len() >= N`,
	///   * `middle.is_empty()` despite `slice.len() >= 3 * N`,
	///   * `suffix.len() >= N`.
	///
	/// That said, this is a safe method, so if you're only writing safe code, then this can at most
	/// cause incorrect logic, not unsoundness.
	///
	/// This is the mutable version of [`Self::as_simd`].
	///
	/// # Panics
	///
	/// Panic if the size of the SIMD vector is different from `N` times that of the scalar.
	#[must_use]
	fn as_simd_mut(slice: &mut [R]) -> (&mut [R], &mut [Self], &mut [R]);

	/// Returns an array reference containing the entire SIMD vector.
	#[must_use]
	fn as_array(&self) -> &[R; N];
	/// Returns a mutable array reference containing the entire SIMD vector.
	#[must_use]
	fn as_mut_array(&mut self) -> &mut [R; N];
	/// Converts an array to a SIMD vector.
	#[must_use]
	fn from_array(array: [R; N]) -> Self;
	/// Converts a SIMD vector to an array.
	#[must_use]
	fn to_array(self) -> [R; N];

	/// Converts a slice to a SIMD vector containing `slice[..N]`
	///
	/// # Panics
	///
	/// Panics if the slice's `len` is less than the vector's `Simd::N`.
	#[must_use]
	fn from_slice(slice: &[R]) -> Self;

	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// If an index is out-of-bounds, the lane is instead selected from the `or` vector.
	#[must_use]
	fn gather_or(slice: &[R], idxs: Simd<usize, N>, or: Self) -> Self
	where
		LaneCount<N>: SupportedLaneCount;
	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// If an index is out-of-bounds, the lane is set to the default value for the type.
	#[must_use]
	fn gather_or_default(slice: &[R], idxs: Simd<usize, N>) -> Self
	where
		R: Default,
		LaneCount<N>: SupportedLaneCount;
	/// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
	///
	/// The mask `enable`s all `true` lanes and disables all `false` lanes.
	/// If an index is disabled or is out-of-bounds, the lane is selected from the `or` vector.
	#[must_use]
	fn gather_select(slice: &[R], enable: Mask<isize, N>, idxs: Simd<usize, N>, or: Self) -> Self
	where
		LaneCount<N>: SupportedLaneCount;
	/// Writes the values in a SIMD vector to potentially discontiguous indices in `slice`.
	///
	/// If two lanes in the scattered vector would write to the same index only the last lane is
	/// guaranteed to actually be written.
	fn scatter(self, slice: &mut [R], idxs: Simd<usize, N>)
	where
		LaneCount<N>: SupportedLaneCount;
	/// Writes the values in a SIMD vector to multiple potentially discontiguous indices in `slice`.
	///
	/// The mask `enable`s all `true` lanes and disables all `false` lanes. If an enabled index is
	/// out-of-bounds, the lane is not written. If two enabled lanes in the scattered vector would
	/// write to the same index, only the last lane is guaranteed to actually be written.
	fn scatter_select(self, slice: &mut [R], enable: Mask<isize, N>, idxs: Simd<usize, N>)
	where
		LaneCount<N>: SupportedLaneCount;

	/// Raw transmutation from an unsigned integer vector type with the same size and number of
	/// lanes.
	#[must_use]
	fn from_bits(bits: Self::Bits) -> Self;
	/// Raw transmutation to an unsigned integer vector type with the same size and number of lanes.
	#[must_use]
	fn to_bits(self) -> Self::Bits;

	/// Inserts `value` at `lane`.
	#[must_use]
	#[inline]
	fn insert(mut self, lane: usize, value: R) -> Self {
		self[lane] = value;
		self
	}

	/// Reducing wrapping add. Returns the sum of the lanes of the vector, with wrapping addition.
	#[must_use]
	fn reduce_sum(self) -> R;
	/// Reducing wrapping multiply. Returns the product of the lanes of the vector, with wrapping
	/// multiplication.
	#[must_use]
	fn reduce_product(self) -> R;
	/// Reducing minimum. Returns the minimum lane in the vector.
	///
	/// Returns values based on equality, so a vector containing both `0.0` and `-0.0` may return
	/// either. This function will not return NaN unless all lanes are NaN.
	#[must_use]
	fn reduce_min(self) -> R;
	/// Reducing maximum. Returns the maximum lane in the vector.
	///
	/// Returns values based on equality, so a vector containing both `0.0` and `-0.0` may return
	/// either. This function will not return NaN unless all lanes are NaN.
	#[must_use]
	fn reduce_max(self) -> R;

	/// Reverse the order of the lanes in the vector.
	#[must_use]
	fn reverse(self) -> Self;
	/// Rotates the vector such that the first `OFFSET` lanes of the slice move to the end while
	/// the last `Self::N - OFFSET` lanes move to the front. The lane previously in lane
	/// `OFFSET` will become the first lane in the slice.
	#[must_use]
	fn simd_rotate_left<const OFFSET: usize>(self) -> Self;
	/// Rotates the vector such that the first `Self::N - OFFSET` lanes of the vector move to
	/// the end while the last `OFFSET` lanes move to the front. The lane previously at index
	/// `Self::N - OFFSET` will become the first lane in the slice.
	#[must_use]
	fn simd_rotate_right<const OFFSET: usize>(self) -> Self;
	/// Interleaves two vectors.
	///
	/// Produces two vectors with lanes taken alternately from `self` and `other`.
	///
	/// The first result contains the first `Self::N / 2` lanes from `self` and `other`,
	/// alternating, starting with the first lane of `self`.
	///
	/// The second result contains the last `Self::N / 2` lanes from `self` and `other`,
	/// alternating, starting with the lane `Self::N / 2` from the start of `self`.
	#[must_use]
	fn interleave(self, other: Self) -> (Self, Self);
	/// Deinterleaves two vectors.
	///
	/// The first result takes every other lane of `self` and then `other`, starting with the first
	/// lane.
	///
	/// The second result takes every other lane of `self` and then `other`, starting with the
	/// second lane.
	#[must_use]
	fn deinterleave(self, other: Self) -> (Self, Self);

	/// Creates new vector by copying lanes from selected lanes of `self`.
	#[must_use]
	fn swizzle<T: Swizzle<N>>(self) -> Self;
	/// Creates new vector by copying lanes from selected lanes of `self` and `other`.
	#[must_use]
	fn concat_swizzle<T: Swizzle<N>>(self, other: Self) -> Self;

	/// Tests lanes for approximate equality wrt `epsilon` or `ulp`, "or" in the sense of `||`.
	#[must_use]
	fn simd_approx_eq(self, other: Self, epsilon: Self, ulp: Self::Bits) -> Self::Mask {
		let (self_bits, other_bits) = (self.to_bits(), other.to_bits());
		(self - other).abs().simd_le(epsilon)
			| !(self.is_nan() | other.is_nan())
				& !(self.is_sign_negative() ^ other.is_sign_negative())
				& self_bits.abs_sub(other_bits).simd_le(ulp)
	}
	/// Tests lanes for approximate inequality wrt `epsilon` and `ulp`, "and" in the sense of `&&`.
	#[must_use]
	#[inline]
	fn simd_approx_ne(self, other: Self, epsilon: Self, ulp: Self::Bits) -> Self::Mask {
		!self.simd_approx_eq(other, epsilon, ulp)
	}

	/// Test if each lane is equal to the corresponding lane in `other`.
	#[must_use]
	fn simd_eq(self, other: Self) -> Self::Mask;
	/// Test if each lane is not equal to the corresponding lane in `other`.
	#[must_use]
	fn simd_ne(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than the corresponding lane in `other`.
	#[must_use]
	fn simd_lt(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than the corresponding lane in `other`.
	#[must_use]
	fn simd_gt(self, other: Self) -> Self::Mask;
	/// Test if each lane is less than or equal to the corresponding lane in `other`.
	#[must_use]
	fn simd_le(self, other: Self) -> Self::Mask;
	/// Test if each lane is greater than or equal to the corresponding lane in `other`.
	#[must_use]
	fn simd_ge(self, other: Self) -> Self::Mask;

	/// Returns true for each lane if it has a positive sign, including `+0.0`, NaNs with positive
	/// sign bit and positive infinity.
	#[must_use]
	fn is_sign_positive(self) -> Self::Mask;
	/// Returns true for each lane if it has a negative sign, including `-0.0`, NaNs with negative
	/// sign bit and negative infinity.
	#[must_use]
	fn is_sign_negative(self) -> Self::Mask;
	/// Returns true for each lane if its value is NaN.
	#[must_use]
	fn is_nan(self) -> Self::Mask;
	/// Returns true for each lane if its value is positive infinity or negative infinity.
	#[must_use]
	fn is_infinite(self) -> Self::Mask;
	/// Returns true for each lane if its value is neither infinite nor NaN.
	#[must_use]
	fn is_finite(self) -> Self::Mask;
	/// Returns true for each lane if its value is subnormal.
	#[must_use]
	fn is_subnormal(self) -> Self::Mask;
	/// Returns true for each lane if its value is neither neither zero, infinite, subnormal, or
	/// NaN.
	#[must_use]
	fn is_normal(self) -> Self::Mask;

	/// Produces a vector where every lane has the absolute value of the equivalently-indexed lane
	/// in `self`.
	#[must_use]
	fn abs(self) -> Self;
	/// Replaces each lane with a number that represents its sign.
	///
	///   * returns `1.0` if the number is positive, `+0.0`, or [`Real::INFINITY`].
	///   * returns `-1.0` if the number is negative, `-0.0`, or [`Real::NEG_INFINITY`].
	///   * returns [`Real::NAN`] if the number is NaN.
	#[must_use]
	fn signum(self) -> Self;
	/// Returns each lane with the magnitude of `self` and the sign of `sign`.
	///
	/// If any lane is a [`Real::NAN`], then a [`Real::NAN`] with the sign of `sign` is returned.
	#[must_use]
	fn copysign(self, sign: Self) -> Self;
	/// Returns the minimum of each lane.
	///
	/// If one of the values is [`Real::NAN`], then the other value is returned.
	#[must_use]
	fn simd_min(self, other: Self) -> Self;
	/// Returns the maximum of each lane.
	///
	/// If one of the values is [`Real::NAN`], then the other value is returned.
	#[must_use]
	fn simd_max(self, other: Self) -> Self;
	/// Restrict each lane to a certain interval unless it is NaN.
	///
	/// For each lane in `self`, returns the corresponding lane in `max` if the lane is
	/// greater than `max`, and the corresponding lane in `min` if the lane is less
	/// than `min`.  Otherwise returns the lane in `self`.
	#[must_use]
	fn simd_clamp(self, min: Self, max: Self) -> Self;

	/// Takes the reciprocal (inverse) of each lane, ${1 \over x}$.
	#[must_use]
	fn recip(self) -> Self;

	/// Converts each lane from radians to degrees.
	#[must_use]
	fn to_degrees(self) -> Self;
	/// Converts each lane from degrees to radians.
	#[must_use]
	fn to_radians(self) -> Self;

	/// Fused multiply-add. Computes `(self * a) + b` with only one rounding error, yielding a more
	/// accurate result than an unfused multiply-add.
	///
	/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
	/// architecture has a dedicated `fma` CPU instruction. However, this is not always true, and
	/// will be heavily dependant on designing algorithms with specific target hardware in mind.
	#[must_use]
	fn mul_add(self, a: Self, b: Self) -> Self;
	/// Produces a vector where every lane has the square root value of the equivalently-indexed
	/// lane in `self`
	#[must_use]
	fn sqrt(self) -> Self;
	/// Returns the largest integer value less than or equal to each lane.
	#[must_use]
	fn floor(self) -> Self;
	/// Returns the smallest integer greater than or equal to each lane.
	#[must_use]
	fn ceil(self) -> Self;
	/// Rounds to the nearest integer value. Ties round toward zero.
	#[must_use]
	fn round(self) -> Self;
	/// Returns the floating point's integer value, with its fractional part removed.
	#[must_use]
	fn trunc(self) -> Self;
	/// Returns the floating point's fractional value, with its integer part removed.
	#[must_use]
	fn fract(self) -> Self;

	/// Converts an array to a SIMD vector mask.
	#[must_use]
	#[inline]
	fn mask_from_array(array: [bool; N]) -> Self::Mask {
		Self::Mask::from_array(array)
	}
	/// Constructs a mask with `lane` set to `value` and all the other lanes set to `!value`.
	#[must_use]
	#[inline]
	fn mask_flag(lane: usize, value: bool) -> Self::Mask {
		Self::Mask::flag(lane, value)
	}
}
