// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Select;
use core::{
	fmt::Debug,
	ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Neg, Not},
};

mod i32;
mod i64;

/// Mask vector of [`Mask<i32, LANES>`] or [`Mask<i64, LANES>`].
///
/// [`Mask<i32, LANES>`]: `core::simd::Mask`
/// [`Mask<i64, LANES>`]: `core::simd::Mask`
pub trait SimdMask<const LANES: usize>
where
	Self: Clone + Copy + Default,
	Self: PartialEq + PartialOrd,
	Self: Debug,
	Self: From<[bool; LANES]>,
	Self: Select<Self>,
	Self: BitAnd<Output = Self> + BitAndAssign + BitAnd<bool, Output = Self> + BitAndAssign<bool>,
	Self: BitOr<Output = Self> + BitOrAssign + BitOr<bool, Output = Self> + BitOrAssign<bool>,
	Self: BitXor<Output = Self> + BitXorAssign + BitXor<bool, Output = Self> + BitXorAssign<bool>,
	Self: Not<Output = Self>,
{
	/// Number of lanes in this vector.
	const LANES: usize = LANES;

	/// Get the number of lanes in this vector.
	#[must_use]
	#[inline]
	fn lanes(&self) -> usize {
		LANES
	}

	/// Constructs a mask by setting all lanes to the given value.
	#[must_use]
	fn splat(value: bool) -> Self;

	/// Converts an array to a SIMD vector mask.
	#[must_use]
	fn from_array(array: [bool; LANES]) -> Self;
	/// Converts a SIMD vector mask to an array.
	#[must_use]
	fn to_array(self) -> [bool; LANES];

	/// Constructs a mask with `lane` set to `value` and all the other lanes set to `!value`.
	#[must_use]
	#[inline]
	fn flag(lane: usize, value: bool) -> Self {
		let mut array = [!value; LANES];
		array[lane] = value;
		Self::from_array(array)
	}

	/// Returns true if all lanes are set, or false otherwise.
	#[must_use]
	fn all(self) -> bool;
	/// Returns true if any lane is set, or false otherwise.
	#[must_use]
	fn any(self) -> bool;

	/// Sets the value of the specified lane.
	///
	/// # Panics
	///
	/// Panics if `lane` is greater than or equal to the number of lanes in the vector.
	fn set(&mut self, lane: usize, value: bool);
	/// Tests the value of the specified lane.
	///
	/// # Panics
	///
	/// Panics if `lane` is greater than or equal to the number of lanes in the vector.
	#[must_use]
	fn test(&self, lane: usize) -> bool;

	/// Chooses lanes from two vectors.
	///
	/// For each lane in the mask, choose the corresponding lane from `true_values` if
	/// that lane mask is true, and `false_values` if that lane mask is false.
	#[must_use]
	#[inline]
	fn select<S: Select<Self>>(self, true_values: S, false_values: S) -> S {
		Select::select(self, true_values, false_values)
	}
	/// Negates lanes if their lane mask is true.
	#[must_use]
	#[inline]
	fn negate<S: Select<Self> + Neg<Output = S> + Copy>(self, values: S) -> S {
		self.select(-values, values)
	}
}
