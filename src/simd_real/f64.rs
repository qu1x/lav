// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::SimdReal;
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount, Swizzle, Swizzle2};

impl<const LANES: usize> SimdReal<f64, LANES> for Simd<f64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	type Bits = Simd<u64, LANES>;
	type Mask = Mask<i64, LANES>;

	fn splat(value: f64) -> Self {
		Self::splat(value)
	}

	fn as_array(&self) -> &[f64; LANES] {
		self.as_array()
	}
	fn as_mut_array(&mut self) -> &mut [f64; LANES] {
		self.as_mut_array()
	}
	fn from_array(array: [f64; LANES]) -> Self {
		Self::from_array(array)
	}
	fn to_array(self) -> [f64; LANES] {
		Self::to_array(self)
	}

	fn from_slice(slice: &[f64]) -> Self {
		Self::from_slice(slice)
	}

	fn gather_or(slice: &[f64], idxs: Simd<usize, LANES>, or: Self) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or(slice, idxs, or)
	}
	fn gather_or_default(slice: &[f64], idxs: Simd<usize, LANES>) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or_default(slice, idxs)
	}
	fn gather_select(
		slice: &[f64],
		enable: Mask<isize, LANES>,
		idxs: Simd<usize, LANES>,
		or: Self,
	) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_select(slice, enable, idxs, or)
	}
	fn scatter(self, slice: &mut [f64], idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter(slice, idxs)
	}
	fn scatter_select(self, slice: &mut [f64], enable: Mask<isize, LANES>, idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter_select(slice, enable, idxs)
	}

	fn from_bits(bits: Self::Bits) -> Self {
		Self::from_bits(bits)
	}
	fn to_bits(self) -> Self::Bits {
		self.to_bits()
	}

	fn horizontal_sum(self) -> f64 {
		self.horizontal_sum()
	}
	fn horizontal_product(self) -> f64 {
		self.horizontal_product()
	}
	fn horizontal_min(self) -> f64 {
		self.horizontal_min()
	}
	fn horizontal_max(self) -> f64 {
		self.horizontal_max()
	}

	fn reverse(self) -> Self {
		self.reverse()
	}
	fn rotate_lanes_left<const OFFSET: usize>(self) -> Self {
		self.rotate_lanes_left::<OFFSET>()
	}
	fn rotate_lanes_right<const OFFSET: usize>(self) -> Self {
		self.rotate_lanes_right::<OFFSET>()
	}
	fn interleave(self, other: Self) -> (Self, Self) {
		self.interleave(other)
	}
	fn deinterleave(self, other: Self) -> (Self, Self) {
		self.deinterleave(other)
	}

	fn swizzle<T: Swizzle<LANES, LANES>>(self) -> Self {
		T::swizzle(self)
	}
	fn swizzle2<T: Swizzle2<LANES, LANES>>(self, other: Self) -> Self {
		T::swizzle2(self, other)
	}

	fn lanes_eq(self, other: Self) -> Self::Mask {
		self.lanes_eq(other)
	}
	fn lanes_ne(self, other: Self) -> Self::Mask {
		self.lanes_ne(other)
	}
	fn lanes_lt(self, other: Self) -> Self::Mask {
		self.lanes_lt(other)
	}
	fn lanes_gt(self, other: Self) -> Self::Mask {
		self.lanes_gt(other)
	}
	fn lanes_le(self, other: Self) -> Self::Mask {
		self.lanes_le(other)
	}
	fn lanes_ge(self, other: Self) -> Self::Mask {
		self.lanes_ge(other)
	}

	fn is_sign_positive(self) -> Self::Mask {
		self.is_sign_positive()
	}
	fn is_sign_negative(self) -> Self::Mask {
		self.is_sign_negative()
	}
	fn is_nan(self) -> Self::Mask {
		self.is_nan()
	}
	fn is_infinite(self) -> Self::Mask {
		self.is_infinite()
	}
	fn is_finite(self) -> Self::Mask {
		self.is_finite()
	}
	fn is_subnormal(self) -> Self::Mask {
		self.is_subnormal()
	}
	fn is_normal(self) -> Self::Mask {
		self.is_normal()
	}

	fn abs(self) -> Self {
		self.abs()
	}
	fn signum(self) -> Self {
		self.signum()
	}
	fn copysign(self, sign: Self) -> Self {
		self.copysign(sign)
	}
	fn min(self, other: Self) -> Self {
		self.min(other)
	}
	fn max(self, other: Self) -> Self {
		self.max(other)
	}
	fn clamp(self, min: Self, max: Self) -> Self {
		self.clamp(min, max)
	}

	fn recip(self) -> Self {
		self.recip()
	}

	fn to_degrees(self) -> Self {
		self.to_degrees()
	}
	fn to_radians(self) -> Self {
		self.to_radians()
	}
}
