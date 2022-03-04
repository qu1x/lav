// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{ApproxEq, Select, SimdReal};
use core::simd::{LaneCount, Mask, Simd, SupportedLaneCount, Swizzle, Swizzle2};

#[cfg(feature = "libm")]
use super::Real;
#[cfg(not(feature = "libm"))]
use std::simd::StdFloat;

impl<const LANES: usize> SimdReal<f64, LANES> for Simd<f64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	type Bits = Simd<u64, LANES>;
	type Mask = Mask<i64, LANES>;

	#[inline]
	fn splat(value: f64) -> Self {
		Self::splat(value)
	}

	#[inline]
	fn as_array(&self) -> &[f64; LANES] {
		self.as_array()
	}
	#[inline]
	fn as_mut_array(&mut self) -> &mut [f64; LANES] {
		self.as_mut_array()
	}
	#[inline]
	fn from_array(array: [f64; LANES]) -> Self {
		Self::from_array(array)
	}
	#[inline]
	fn to_array(self) -> [f64; LANES] {
		Self::to_array(self)
	}

	#[inline]
	fn from_slice(slice: &[f64]) -> Self {
		Self::from_slice(slice)
	}

	#[inline]
	fn gather_or(slice: &[f64], idxs: Simd<usize, LANES>, or: Self) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or(slice, idxs, or)
	}
	#[inline]
	fn gather_or_default(slice: &[f64], idxs: Simd<usize, LANES>) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or_default(slice, idxs)
	}
	#[inline]
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
	#[inline]
	fn scatter(self, slice: &mut [f64], idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter(slice, idxs);
	}
	#[inline]
	fn scatter_select(self, slice: &mut [f64], enable: Mask<isize, LANES>, idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter_select(slice, enable, idxs);
	}

	#[inline]
	fn from_bits(bits: Self::Bits) -> Self {
		Self::from_bits(bits)
	}
	#[inline]
	fn to_bits(self) -> Self::Bits {
		self.to_bits()
	}

	#[inline]
	fn horizontal_sum(self) -> f64 {
		self.horizontal_sum()
	}
	#[inline]
	fn horizontal_product(self) -> f64 {
		self.horizontal_product()
	}
	#[inline]
	fn horizontal_min(self) -> f64 {
		self.horizontal_min()
	}
	#[inline]
	fn horizontal_max(self) -> f64 {
		self.horizontal_max()
	}

	#[inline]
	fn reverse(self) -> Self {
		self.reverse()
	}
	#[inline]
	fn rotate_lanes_left<const OFFSET: usize>(self) -> Self {
		self.rotate_lanes_left::<OFFSET>()
	}
	#[inline]
	fn rotate_lanes_right<const OFFSET: usize>(self) -> Self {
		self.rotate_lanes_right::<OFFSET>()
	}
	#[inline]
	fn interleave(self, other: Self) -> (Self, Self) {
		self.interleave(other)
	}
	#[inline]
	fn deinterleave(self, other: Self) -> (Self, Self) {
		self.deinterleave(other)
	}

	#[inline]
	fn swizzle<T: Swizzle<LANES, LANES>>(self) -> Self {
		T::swizzle(self)
	}
	#[inline]
	fn swizzle2<T: Swizzle2<LANES, LANES>>(self, other: Self) -> Self {
		T::swizzle2(self, other)
	}

	#[inline]
	fn lanes_eq(self, other: Self) -> Self::Mask {
		self.lanes_eq(other)
	}
	#[inline]
	fn lanes_ne(self, other: Self) -> Self::Mask {
		self.lanes_ne(other)
	}
	#[inline]
	fn lanes_lt(self, other: Self) -> Self::Mask {
		self.lanes_lt(other)
	}
	#[inline]
	fn lanes_gt(self, other: Self) -> Self::Mask {
		self.lanes_gt(other)
	}
	#[inline]
	fn lanes_le(self, other: Self) -> Self::Mask {
		self.lanes_le(other)
	}
	#[inline]
	fn lanes_ge(self, other: Self) -> Self::Mask {
		self.lanes_ge(other)
	}

	#[inline]
	fn is_sign_positive(self) -> Self::Mask {
		self.is_sign_positive()
	}
	#[inline]
	fn is_sign_negative(self) -> Self::Mask {
		self.is_sign_negative()
	}
	#[inline]
	fn is_nan(self) -> Self::Mask {
		self.is_nan()
	}
	#[inline]
	fn is_infinite(self) -> Self::Mask {
		self.is_infinite()
	}
	#[inline]
	fn is_finite(self) -> Self::Mask {
		self.is_finite()
	}
	#[inline]
	fn is_subnormal(self) -> Self::Mask {
		self.is_subnormal()
	}
	#[inline]
	fn is_normal(self) -> Self::Mask {
		self.is_normal()
	}

	#[inline]
	fn abs(self) -> Self {
		self.abs()
	}
	#[inline]
	fn signum(self) -> Self {
		self.signum()
	}
	#[inline]
	fn copysign(self, sign: Self) -> Self {
		self.copysign(sign)
	}
	#[inline]
	fn min(self, other: Self) -> Self {
		self.min(other)
	}
	#[inline]
	fn max(self, other: Self) -> Self {
		self.max(other)
	}
	#[inline]
	fn clamp(self, min: Self, max: Self) -> Self {
		self.clamp(min, max)
	}

	#[inline]
	fn recip(self) -> Self {
		self.recip()
	}

	#[inline]
	fn to_degrees(self) -> Self {
		self.to_degrees()
	}
	#[inline]
	fn to_radians(self) -> Self {
		self.to_radians()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn mul_add(self, a: Self, b: Self) -> Self {
		self * a + b
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn mul_add(self, a: Self, b: Self) -> Self {
		StdFloat::mul_add(self, a, b)
	}
	#[cfg(feature = "libm")]
	fn sqrt(self) -> Self {
		self.to_array().map(Real::sqrt).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn sqrt(self) -> Self {
		StdFloat::sqrt(self)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn floor(self) -> Self {
		self.to_array().map(Real::floor).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn floor(self) -> Self {
		StdFloat::floor(self)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn ceil(self) -> Self {
		self.to_array().map(Real::ceil).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn ceil(self) -> Self {
		StdFloat::ceil(self)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn round(self) -> Self {
		self.to_array().map(Real::round).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn round(self) -> Self {
		StdFloat::round(self)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn trunc(self) -> Self {
		self.to_array().map(Real::trunc).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn trunc(self) -> Self {
		StdFloat::trunc(self)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn fract(self) -> Self {
		self.to_array().map(Real::fract).into()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn fract(self) -> Self {
		StdFloat::fract(self)
	}
}

impl<const LANES: usize> Select<Mask<i64, LANES>> for Simd<f64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	#[inline]
	fn select(mask: Mask<i64, LANES>, true_values: Self, false_values: Self) -> Self {
		mask.select(true_values, false_values)
	}
}

impl<const LANES: usize> ApproxEq<f64> for Simd<f64, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	#[inline]
	fn approx_eq(&self, other: &Self, epsilon: f64, ulp: u64) -> bool {
		self.lanes_approx_eq(*other, Simd::splat(epsilon), Simd::splat(ulp))
			.all()
	}
}
