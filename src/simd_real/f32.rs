// Copyright © 2021-2022 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{ApproxEq, Select, SimdReal};
use core::simd::{
	cmp::{SimdPartialEq, SimdPartialOrd},
	num::SimdFloat,
	LaneCount, Mask, Simd, SupportedLaneCount, Swizzle,
};

#[cfg(feature = "libm")]
use super::Real;
#[cfg(not(feature = "libm"))]
use std::simd::StdFloat;

impl<const LANES: usize> SimdReal<f32, LANES> for Simd<f32, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	type Bits = Simd<u32, LANES>;
	type Mask = Mask<i32, LANES>;

	#[inline]
	fn splat(value: f32) -> Self {
		Self::splat(value)
	}

	#[inline]
	fn as_array(&self) -> &[f32; LANES] {
		self.as_array()
	}
	#[inline]
	fn as_mut_array(&mut self) -> &mut [f32; LANES] {
		self.as_mut_array()
	}
	#[inline]
	fn from_array(array: [f32; LANES]) -> Self {
		Self::from_array(array)
	}
	#[inline]
	fn to_array(self) -> [f32; LANES] {
		Self::to_array(self)
	}

	#[inline]
	fn from_slice(slice: &[f32]) -> Self {
		Self::from_slice(slice)
	}

	#[inline]
	fn gather_or(slice: &[f32], idxs: Simd<usize, LANES>, or: Self) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or(slice, idxs, or)
	}
	#[inline]
	fn gather_or_default(slice: &[f32], idxs: Simd<usize, LANES>) -> Self
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		Self::gather_or_default(slice, idxs)
	}
	#[inline]
	fn gather_select(
		slice: &[f32],
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
	fn scatter(self, slice: &mut [f32], idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter(slice, idxs);
	}
	#[inline]
	fn scatter_select(self, slice: &mut [f32], enable: Mask<isize, LANES>, idxs: Simd<usize, LANES>)
	where
		LaneCount<LANES>: SupportedLaneCount,
	{
		self.scatter_select(slice, enable, idxs);
	}

	#[inline]
	fn from_bits(bits: Self::Bits) -> Self {
		SimdFloat::from_bits(bits)
	}
	#[inline]
	fn to_bits(self) -> Self::Bits {
		SimdFloat::to_bits(self)
	}

	#[inline]
	fn reduce_sum(self) -> f32 {
		SimdFloat::reduce_sum(self)
	}
	#[inline]
	fn reduce_product(self) -> f32 {
		SimdFloat::reduce_product(self)
	}
	#[inline]
	fn reduce_min(self) -> f32 {
		SimdFloat::reduce_min(self)
	}
	#[inline]
	fn reduce_max(self) -> f32 {
		SimdFloat::reduce_max(self)
	}

	#[inline]
	fn reverse(self) -> Self {
		self.reverse()
	}
	#[inline]
	fn rotate_lanes_left<const OFFSET: usize>(self) -> Self {
		self.rotate_elements_left::<OFFSET>()
	}
	#[inline]
	fn rotate_lanes_right<const OFFSET: usize>(self) -> Self {
		self.rotate_elements_right::<OFFSET>()
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
	fn swizzle<T: Swizzle<LANES>>(self) -> Self {
		T::swizzle(self)
	}
	#[inline]
	fn concat_swizzle<T: Swizzle<LANES>>(self, other: Self) -> Self {
		T::concat_swizzle(self, other)
	}

	#[inline]
	fn simd_eq(self, other: Self) -> Self::Mask {
		SimdPartialEq::simd_eq(self, other)
	}
	#[inline]
	fn simd_ne(self, other: Self) -> Self::Mask {
		SimdPartialEq::simd_ne(self, other)
	}
	#[inline]
	fn simd_lt(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_lt(self, other)
	}
	#[inline]
	fn simd_gt(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_gt(self, other)
	}
	#[inline]
	fn simd_le(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_le(self, other)
	}
	#[inline]
	fn simd_ge(self, other: Self) -> Self::Mask {
		SimdPartialOrd::simd_ge(self, other)
	}

	#[inline]
	fn is_sign_positive(self) -> Self::Mask {
		SimdFloat::is_sign_positive(self)
	}
	#[inline]
	fn is_sign_negative(self) -> Self::Mask {
		SimdFloat::is_sign_negative(self)
	}
	#[inline]
	fn is_nan(self) -> Self::Mask {
		SimdFloat::is_nan(self)
	}
	#[inline]
	fn is_infinite(self) -> Self::Mask {
		SimdFloat::is_infinite(self)
	}
	#[inline]
	fn is_finite(self) -> Self::Mask {
		SimdFloat::is_finite(self)
	}
	#[inline]
	fn is_subnormal(self) -> Self::Mask {
		SimdFloat::is_subnormal(self)
	}
	#[inline]
	fn is_normal(self) -> Self::Mask {
		SimdFloat::is_normal(self)
	}

	#[inline]
	fn abs(self) -> Self {
		SimdFloat::abs(self)
	}
	#[inline]
	fn signum(self) -> Self {
		SimdFloat::signum(self)
	}
	#[inline]
	fn copysign(self, sign: Self) -> Self {
		SimdFloat::copysign(self, sign)
	}
	#[inline]
	fn simd_min(self, other: Self) -> Self {
		SimdFloat::simd_min(self, other)
	}
	#[inline]
	fn simd_max(self, other: Self) -> Self {
		SimdFloat::simd_max(self, other)
	}
	#[inline]
	fn simd_clamp(self, min: Self, max: Self) -> Self {
		SimdFloat::simd_clamp(self, min, max)
	}

	#[inline]
	fn recip(self) -> Self {
		SimdFloat::recip(self)
	}

	#[inline]
	fn to_degrees(self) -> Self {
		SimdFloat::to_degrees(self)
	}
	#[inline]
	fn to_radians(self) -> Self {
		SimdFloat::to_radians(self)
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

impl<const LANES: usize> Select<Mask<i32, LANES>> for Simd<f32, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	#[inline]
	fn select(mask: Mask<i32, LANES>, true_values: Self, false_values: Self) -> Self {
		mask.select(true_values, false_values)
	}
}

impl<const LANES: usize> ApproxEq<f32> for Simd<f32, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	#[inline]
	fn approx_eq(&self, other: &Self, epsilon: f32, ulp: u32) -> bool {
		self.simd_approx_eq(*other, Self::splat(epsilon), Simd::splat(ulp))
			.all()
	}
}
