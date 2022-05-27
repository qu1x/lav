// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Real;
use core::{
	cmp::Ordering,
	num::FpCategory,
	simd::{LaneCount, Simd, SupportedLaneCount},
};

impl Real for f32 {
	type Bits = u32;
	type Simd<const LANES: usize> = Simd<Self, LANES>
	where
		LaneCount<LANES>: SupportedLaneCount;

	const ZERO: Self = 0.0;
	const ONE: Self = 1.0;
	const TWO: Self = 2.0;

	const PI: Self = core::f32::consts::PI;
	const TAU: Self = core::f32::consts::TAU;
	const SQRT_2: Self = core::f32::consts::SQRT_2;

	const FRAC_1_2: Self = 1.0 / 2.0;
	const FRAC_1_3: Self = 1.0 / 3.0;
	const FRAC_1_4: Self = 1.0 / 4.0;
	const FRAC_1_6: Self = 1.0 / 6.0;
	const FRAC_1_8: Self = 1.0 / 8.0;

	const FRAC_PI_2: Self = core::f32::consts::FRAC_PI_2;
	const FRAC_PI_3: Self = core::f32::consts::FRAC_PI_3;
	const FRAC_PI_4: Self = core::f32::consts::FRAC_PI_4;
	const FRAC_PI_6: Self = core::f32::consts::FRAC_PI_6;
	const FRAC_PI_8: Self = core::f32::consts::FRAC_PI_8;

	const FRAC_1_PI: Self = core::f32::consts::FRAC_1_PI;
	const FRAC_1_TAU: Self = Self::FRAC_1_PI * Self::FRAC_1_2;
	const FRAC_1_SQRT_2: Self = core::f32::consts::FRAC_1_SQRT_2;
	const FRAC_2_PI: Self = core::f32::consts::FRAC_2_PI;
	const FRAC_2_SQRT_PI: Self = core::f32::consts::FRAC_2_SQRT_PI;

	const EPSILON: Self = Self::EPSILON;
	const SQRT_EPSILON: Self = 0.000_345_266_98;
	const CBRT_EPSILON: Self = 0.004_921_566_7;

	const RADIX: u32 = Self::RADIX;
	const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
	const DIGITS: u32 = Self::DIGITS;
	const MIN: Self = Self::MIN;
	const MIN_POSITIVE: Self = Self::MIN_POSITIVE;
	const MAX: Self = Self::MAX;
	const MIN_EXP: i32 = Self::MIN_EXP;
	const MAX_EXP: i32 = Self::MAX_EXP;
	const MIN_10_EXP: i32 = Self::MIN_10_EXP;
	const MAX_10_EXP: i32 = Self::MAX_10_EXP;

	const NAN: Self = Self::NAN;
	const INFINITY: Self = Self::INFINITY;
	const NEG_INFINITY: Self = Self::NEG_INFINITY;

	#[inline]
	fn from_bits(bits: Self::Bits) -> Self {
		Self::from_bits(bits)
	}
	#[inline]
	fn to_bits(self) -> Self::Bits {
		self.to_bits()
	}

	#[inline]
	fn is_sign_positive(self) -> bool {
		self.is_sign_positive()
	}
	#[inline]
	fn is_sign_negative(self) -> bool {
		self.is_sign_negative()
	}
	#[inline]
	fn is_nan(self) -> bool {
		self.is_nan()
	}
	#[inline]
	fn is_infinite(self) -> bool {
		self.is_infinite()
	}
	#[inline]
	fn is_finite(self) -> bool {
		self.is_finite()
	}
	#[inline]
	fn is_subnormal(self) -> bool {
		self.is_subnormal()
	}
	#[inline]
	fn is_normal(self) -> bool {
		self.is_normal()
	}
	#[inline]
	fn classify(self) -> FpCategory {
		self.classify()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn floor(self) -> Self {
		libm::floorf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn floor(self) -> Self {
		self.floor()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn ceil(self) -> Self {
		libm::ceilf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn ceil(self) -> Self {
		self.ceil()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn round(self) -> Self {
		libm::roundf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn round(self) -> Self {
		self.round()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn trunc(self) -> Self {
		libm::truncf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn trunc(self) -> Self {
		self.trunc()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn fract(self) -> Self {
		self - self.trunc()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn fract(self) -> Self {
		self.fract()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn abs(self) -> Self {
		libm::fabsf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn abs(self) -> Self {
		self.abs()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn signum(self) -> Self {
		if self.is_nan() {
			Self::NAN
		} else {
			1.0_f32.copysign(self)
		}
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn signum(self) -> Self {
		self.signum()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn copysign(self, sign: Self) -> Self {
		libm::copysignf(self, sign)
	}
	#[cfg(not(feature = "libm"))]
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
	fn to_radians(self) -> Self {
		self.to_radians()
	}
	#[inline]
	fn to_degrees(self) -> Self {
		self.to_degrees()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn mul_add(self, a: Self, b: Self) -> Self {
		libm::fmaf(self, a, b)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn mul_add(self, a: Self, b: Self) -> Self {
		self.mul_add(a, b)
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn div_euclid(self, rhs: Self) -> Self {
		let q = (self / rhs).trunc();
		if self % rhs < 0.0 {
			return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
		}
		q
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn div_euclid(self, rhs: Self) -> Self {
		self.div_euclid(rhs)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn rem_euclid(self, rhs: Self) -> Self {
		let r = self % rhs;
		if r < 0.0 {
			r + rhs.abs()
		} else {
			r
		}
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn rem_euclid(self, rhs: Self) -> Self {
		self.rem_euclid(rhs)
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn powf(self, n: Self) -> Self {
		libm::powf(self, n)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn powf(self, n: Self) -> Self {
		self.powf(n)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn exp(self) -> Self {
		libm::expf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn exp(self) -> Self {
		self.exp()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn exp_m1(self) -> Self {
		libm::expm1f(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn exp_m1(self) -> Self {
		self.exp_m1()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn exp2(self) -> Self {
		libm::exp2f(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn exp2(self) -> Self {
		self.exp2()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn ln(self) -> Self {
		libm::logf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn ln(self) -> Self {
		self.ln()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn ln_1p(self) -> Self {
		libm::log1pf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn ln_1p(self) -> Self {
		self.ln_1p()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn log(self, base: Self) -> Self {
		self.ln() / base.ln()
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn log(self, base: Self) -> Self {
		self.log(base)
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn log2(self) -> Self {
		libm::log2f(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn log2(self) -> Self {
		self.log2()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn log10(self) -> Self {
		libm::log10f(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn log10(self) -> Self {
		self.log10()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn sqrt(self) -> Self {
		libm::sqrtf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn sqrt(self) -> Self {
		self.sqrt()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn cbrt(self) -> Self {
		libm::cbrtf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn cbrt(self) -> Self {
		self.cbrt()
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn hypot(self, other: Self) -> Self {
		libm::hypotf(self, other)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn hypot(self, other: Self) -> Self {
		self.hypot(other)
	}

	#[cfg(feature = "libm")]
	#[inline]
	fn sin(self) -> Self {
		libm::sinf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn sin(self) -> Self {
		self.sin()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn sinh(self) -> Self {
		libm::sinhf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn sinh(self) -> Self {
		self.sinh()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn cos(self) -> Self {
		libm::cosf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn cos(self) -> Self {
		self.cos()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn cosh(self) -> Self {
		libm::coshf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn cosh(self) -> Self {
		self.cosh()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn sin_cos(self) -> (Self, Self) {
		libm::sincosf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn sin_cos(self) -> (Self, Self) {
		self.sin_cos()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn tan(self) -> Self {
		libm::tanf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn tan(self) -> Self {
		self.tan()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn asin(self) -> Self {
		libm::asinf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn asin(self) -> Self {
		self.asin()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn asinh(self) -> Self {
		libm::asinhf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn asinh(self) -> Self {
		self.asinh()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn acos(self) -> Self {
		libm::acosf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn acos(self) -> Self {
		self.acos()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn acosh(self) -> Self {
		libm::acoshf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn acosh(self) -> Self {
		self.acosh()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn atan(self) -> Self {
		libm::atanf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn atan(self) -> Self {
		self.atan()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn atanh(self) -> Self {
		libm::atanhf(self)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn atanh(self) -> Self {
		self.atanh()
	}
	#[cfg(feature = "libm")]
	#[inline]
	fn atan2(self, other: Self) -> Self {
		libm::atan2f(self, other)
	}
	#[cfg(not(feature = "libm"))]
	#[inline]
	fn atan2(self, other: Self) -> Self {
		self.atan2(other)
	}

	#[inline]
	fn total_cmp(&self, other: &Self) -> Ordering {
		self.total_cmp(other)
	}
}
