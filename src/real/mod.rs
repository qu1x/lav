// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Derivative work of `core::{f32, f64}` licensed under `MIT OR Apache-2.0`.

use super::{ApproxEq, Bits, SimdReal};
use core::{
	cmp::Ordering,
	convert::FloatToInt,
	fmt::Debug,
	iter::{Product, Sum},
	num::{FpCategory, ParseFloatError},
	ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
	simd::{LaneCount, SimdElement, SupportedLaneCount},
	str::FromStr,
};

mod f32;
mod f64;

/// Real number of [`prim@f32`] or [`prim@f64`] with associated [`Bits`] representation and
/// [`SimdReal`] vector.
pub trait Real
where
	Self: Send + Sync + Clone + Copy + Default,
	Self: ApproxEq<Self, Self> + PartialEq + PartialOrd,
	Self: From<u8> + From<i8>,
	Self: From<u16> + From<i16>,
	Self: FromStr<Err = ParseFloatError>,
	Self: Product<Self> + Sum<Self>,
	for<'a> Self: Product<&'a Self> + Sum<&'a Self>,
	Self: FloatToInt<usize> + FloatToInt<isize>,
	Self: FloatToInt<u128> + FloatToInt<i128>,
	Self: FloatToInt<u64> + FloatToInt<i64>,
	Self: FloatToInt<u32> + FloatToInt<i32>,
	Self: FloatToInt<u16> + FloatToInt<i16>,
	Self: FloatToInt<u8> + FloatToInt<i8>,
	Self: Debug,
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
	Self: SimdElement,
{
	/// Associated bits representation.
	type Bits: Bits;
	/// Associated vector.
	type Simd<const N: usize>: SimdReal<Self, N>
	where
		LaneCount<N>: SupportedLaneCount;

	/// Native lane count of current build target or `1` if unknown.
	#[cfg(feature = "target-features")]
	const NATIVE_LANE_COUNT: usize;

	/// $0$
	const ZERO: Self;
	/// $1$
	const ONE: Self;
	/// $2$
	const TWO: Self;

	/// $\pi$
	const PI: Self;
	/// $\tau$
	const TAU: Self;
	/// $\sqrt{2}$
	const SQRT_2: Self;

	/// $\frac{1}{2}$
	const FRAC_1_2: Self;
	/// $\frac{1}{3}$
	const FRAC_1_3: Self;
	/// $\frac{1}{4}$
	const FRAC_1_4: Self;
	/// $\frac{1}{6}$
	const FRAC_1_6: Self;
	/// $\frac{1}{8}$
	const FRAC_1_8: Self;

	/// $\frac{\pi}{2}$
	const FRAC_PI_2: Self;
	/// $\frac{\pi}{3}$
	const FRAC_PI_3: Self;
	/// $\frac{\pi}{4}$
	const FRAC_PI_4: Self;
	/// $\frac{\pi}{6}$
	const FRAC_PI_6: Self;
	/// $\frac{\pi}{8}$
	const FRAC_PI_8: Self;

	/// $\frac{1}{\pi}$
	const FRAC_1_PI: Self;
	/// $\frac{1}{\tau}$
	const FRAC_1_TAU: Self;
	/// $\frac{1}{\sqrt{2}}$
	const FRAC_1_SQRT_2: Self;
	/// $\frac{2}{\pi}$
	const FRAC_2_PI: Self;
	/// $\frac{2}{\sqrt{\pi}}$
	const FRAC_2_SQRT_PI: Self;

	/// [Machine epsilon] $\epsilon$ of floating-point type.
	///
	/// [Machine epsilon]: https://en.wikipedia.org/wiki/Machine_epsilon
	const EPSILON: Self;
	/// $\sqrt{\epsilon}$
	const SQRT_EPSILON: Self;
	/// $\sqrt\[3]{\epsilon}$
	const CBRT_EPSILON: Self;

	/// The radix or base of the internal representation of floating-point type.
	const RADIX: u32;
	/// Number of significant digits in base $2$.
	const MANTISSA_DIGITS: u32;
	/// Approximate number of significant digits in base $10$.
	const DIGITS: u32;
	/// Smallest finite floating-point type value.
	const MIN: Self;
	/// Smallest positive normal floating-point type value.
	const MIN_POSITIVE: Self;
	/// Largest finite floating-point type value.
	const MAX: Self;
	/// One greater than the minimum possible normal power of $2$ exponent.
	const MIN_EXP: i32;
	/// Maximum possible power of $2$ exponent.
	const MAX_EXP: i32;
	/// Minimum possible normal power of $10$ exponent.
	const MIN_10_EXP: i32;
	/// Maximum possible power of $10$ exponent.
	const MAX_10_EXP: i32;

	/// Not a number (NaN).
	const NAN: Self;
	/// Infinity $\infty$.
	const INFINITY: Self;
	/// Negative infinity $-\infty$.
	const NEG_INFINITY: Self;

	/// Raw transmutation from `u64`.
	///
	/// This is currently identical to [`transmute::<u64, f64>(v)`](core::mem::transmute) on all
	/// platforms. It turns out this is incredibly portable, for two reasons:
	///
	/// * Floats and Ints have the same endianness on all supported platforms.
	/// * IEEE-754 very precisely specifies the bit layout of floats.
	///
	/// However there is one caveat: prior to the 2008 version of IEEE-754, how to interpret the NaN
	/// signaling bit wasn't actually specified. Most platforms (notably x86 and ARM) picked the
	/// interpretation that was ultimately standardized in 2008, but some didn't (notably MIPS). As
	/// a result, all signaling NaNs on MIPS are quiet NaNs on x86, and vice-versa.
	///
	/// Rather than trying to preserve signaling-ness cross-platform, this implementation favors
	/// preserving the exact bits. This means that any payloads encoded in NaNs will be preserved
	/// even if the result of this method is sent over the network from an x86 machine to a MIPS
	/// one.
	///
	/// If the results of this method are only manipulated by the same architecture that produced
	/// them, then there is no portability concern.
	///
	/// If the input isn't NaN, then there is no portability concern.
	///
	/// If you don't care about signaling-ness (very likely), then there is no portability concern.
	///
	/// Note that this function is distinct from `as` casting, which attempts to preserve the
	/// *numeric* value, and not the bitwise value.
	#[must_use]
	fn from_bits(bits: Self::Bits) -> Self;
	/// Raw transmutation to `u64`.
	///
	/// This is currently identical to [`transmute::<f64, u64>(self)`](core::mem::transmute) on all
	/// platforms.
	///
	/// See [`from_bits`](Self::from_bits) for some discussion of the portability of this operation
	/// (there are almost no issues).
	///
	/// Note that this function is distinct from `as` casting, which attempts to preserve the
	/// *numeric* value, and not the bitwise value.
	#[must_use]
	fn to_bits(self) -> Self::Bits;

	/// Returns `true` for each lane if it has a positive sign, including `+0.0`, NaNs with positive
	/// sign bit and positive infinity.
	#[must_use]
	fn is_sign_positive(self) -> bool;
	/// Returns `true` for each lane if it has a negative sign, including `-0.0`, NaNs with negative
	/// sign bit and negative infinity.
	#[must_use]
	fn is_sign_negative(self) -> bool;
	/// Returns `true` for each lane if its value is NaN.
	#[must_use]
	fn is_nan(self) -> bool;
	/// Returns `true` for each lane if its value is positive infinity or negative infinity.
	#[must_use]
	fn is_infinite(self) -> bool;
	/// Returns `true` for each lane if its value is neither infinite nor NaN.
	#[must_use]
	fn is_finite(self) -> bool;
	/// Returns `true` for each lane if its value is subnormal.
	#[must_use]
	fn is_subnormal(self) -> bool;
	/// Returns `true` for each lane if its value is neither neither zero, infinite, subnormal, or
	/// NaN.
	#[must_use]
	fn is_normal(self) -> bool;
	/// Returns the floating point category of the number.
	///
	/// If only one property is going to be tested, it is generally faster to use the specific
	/// predicate instead.
	#[must_use]
	fn classify(self) -> FpCategory;

	/// Returns the largest integer less than or equal to a number.
	#[must_use]
	fn floor(self) -> Self;
	/// Returns the smallest integer greater than or equal to a number.
	#[must_use]
	fn ceil(self) -> Self;
	/// Returns the nearest integer to a number. Round half-way cases away from `0.0`.
	#[must_use]
	fn round(self) -> Self;
	/// Returns the integer part of a number.
	#[must_use]
	fn trunc(self) -> Self;
	/// Returns the fractional part of a number.
	#[must_use]
	fn fract(self) -> Self;

	/// Computes the absolute value of `self`.
	///
	/// Returns [`Self::NAN`] if the number is NaN.
	#[must_use]
	fn abs(self) -> Self;
	/// Returns a number that represents the sign of `self`.
	///
	///  * Returns `1.0` if the number is positive, `+0.0` or [`Self::INFINITY`].
	///  * Returns `-1.0` if the number is negative, `-0.0` or [`Self::NEG_INFINITY`].
	///  * Returns [`Self::NAN`] if the number is NaN.
	#[must_use]
	fn signum(self) -> Self;
	/// Returns a number composed of the magnitude of `self` and the sign of `sign`.
	///
	/// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal to `-self`.
	///
	/// If `self` is NaN, then NaN with the sign of `sign` is returned.
	#[must_use]
	fn copysign(self, sign: Self) -> Self;
	/// Returns the minimum of each lane.
	///
	/// If one of the values is NaN, then the other value is returned.
	#[must_use]
	fn min(self, other: Self) -> Self;
	/// Returns the maximum of each lane.
	///
	/// If one of the values is NaN, then the other value is returned.
	#[must_use]
	fn max(self, other: Self) -> Self;
	/// Restrict each lane to a certain interval unless it is NaN.
	///
	/// For each lane in `self`, returns the corresponding lane in `max` if the lane is greater than
	/// `max`, and the corresponding lane in `min` if the lane is less than `min`. Otherwise,
	/// returns the lane in `self`.
	#[must_use]
	fn clamp(self, min: Self, max: Self) -> Self;

	/// Takes the reciprocal (inverse) of a number, `1 / self`.
	#[must_use]
	fn recip(self) -> Self;

	/// Converts degrees to radians.
	#[must_use]
	fn to_radians(self) -> Self;
	/// Converts radians to degrees.
	#[must_use]
	fn to_degrees(self) -> Self;

	/// Fused multiply-add. Computes `(self * a) + b` with only one rounding error, yielding a more
	/// accurate result than an unfused multiply-add.
	///
	/// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
	/// architecture has a dedicated `fma` CPU instruction. However, this is not always true, and
	/// will be heavily dependant on designing algorithms with specific target hardware in mind.
	#[must_use]
	fn mul_add(self, a: Self, b: Self) -> Self;

	/// Calculates Euclidean division, the matching method for [`Self::rem_euclid()`].
	///
	/// This computes the integer `n` such that `self = n * rhs + self.rem_euclid(rhs)`. In other
	/// words, the result is `self / rhs` rounded to the integer `n` such that `self >= n * rhs`.
	#[must_use]
	fn div_euclid(self, rhs: Self) -> Self;
	/// Calculates the least non-negative remainder of `self (mod rhs)`.
	///
	/// In particular, the return value `r` satisfies `0.0 <= r < rhs.abs()` in most cases. However,
	/// due to a floating point round-off error it can result in `r == rhs.abs()`, violating the
	/// mathematical definition, if `self` is much smaller than `rhs.abs()` in magnitude and
	/// `self < 0.0`. This result is not an element of the function's codomain, but it is the
	/// closest floating point number in the real numbers and thus fulfills the property
	/// `self == self.div_euclid(rhs) * rhs + self.rem_euclid(rhs)` approximatively.
	#[must_use]
	fn rem_euclid(self, rhs: Self) -> Self;

	/// Raises a number to a floating-point power.
	#[must_use]
	fn powf(self, n: Self) -> Self;
	/// Returns $e^x$.
	#[must_use]
	fn exp(self) -> Self;
	/// Returns $e^x - 1$ in a way that is accurate even if the number is close to zero.
	#[must_use]
	fn exp_m1(self) -> Self;
	/// Returns $2^x$.
	#[must_use]
	fn exp2(self) -> Self;
	/// Returns the natural logarithm of the number.
	#[must_use]
	fn ln(self) -> Self;
	/// Returns the natural logarithm of the number plus one more accurately than if the operations
	/// were performed separately.
	#[must_use]
	fn ln_1p(self) -> Self;
	/// Returns the logarithm of the number with respect to an arbitrary base.
	///
	/// The result might not be correctly rounded owing to implementation details:
	///
	///   * [`Self::log2()`] can produce more accurate results for base $2$, and
	///   * [`Self::log10()`] can produce more accurate results for base $10$.
	#[must_use]
	fn log(self, base: Self) -> Self;
	/// Returns the base $2$ logarithm of the number.
	#[must_use]
	fn log2(self) -> Self;
	/// Returns the base $10$ logarithm of the number.
	#[must_use]
	fn log10(self) -> Self;

	/// Returns the square root of a number.
	///
	/// Returns NaN if `self` is a negative number.
	#[must_use]
	fn sqrt(self) -> Self;
	/// Returns the cube root of a number.
	#[must_use]
	fn cbrt(self) -> Self;

	/// Calculates the length of the hypotenuse of a right-angle triangle given legs of length
	/// `self` and `other`.
	#[must_use]
	fn hypot(self, other: Self) -> Self;

	/// Computes the sine of a number in radians.
	#[must_use]
	fn sin(self) -> Self;
	/// Computes the hyperbolic sine of a number in radians.
	#[must_use]
	fn sinh(self) -> Self;
	/// Computes the cosine of a number in radians.
	#[must_use]
	fn cos(self) -> Self;
	/// Computes the hyperbolic cosine of a number in radians.
	#[must_use]
	fn cosh(self) -> Self;
	/// Simultaneously computes the sine and cosine of `self`.
	///
	/// Returns `(self.sin(), self.cos())`.
	#[must_use]
	fn sin_cos(self) -> (Self, Self);
	/// Computes the tangent of a number in radians.
	#[must_use]
	fn tan(self) -> Self;
	/// Computes the arcsine of a number.
	///
	/// Return value is in radians in the range $[-{\pi \over 2}, {\pi \over 2}]$ or NaN if the
	/// number is outside the range $[-1, 1]$.
	#[must_use]
	fn asin(self) -> Self;
	/// Inverse hyperbolic sine function.
	#[must_use]
	fn asinh(self) -> Self;
	/// Computes the arccosine of a number.
	///
	/// Return value is in radians in the range $[0, \pi]$ or NaN if the number is outside the range
	/// $[-1, 1]$.
	#[must_use]
	fn acos(self) -> Self;
	/// Inverse hyperbolic cosine function.
	#[must_use]
	fn acosh(self) -> Self;
	/// Computes the arctangent of a number.
	///
	/// Return value is in radians in the range $[-{\pi \over 2}, {\pi \over 2}]$.
	#[must_use]
	fn atan(self) -> Self;
	/// Inverse hyperbolic tangent function.
	#[must_use]
	fn atanh(self) -> Self;
	/// Computes the four quadrant arctangent of `self` as $y$ and `other` as $x$ in radians.
	///
	/// $$
	/// \arctan(y, x) = \begin{cases}
	///   0 & \text{if } x = 0 \wedge y = 0 \\\\
	///   \arctan({y \over x}) \in [-{\pi \over 2}, {\pi \over 2}] & \text{if } x \ge 0 \\\\
	///   \arctan({y \over x}) + \pi \in ({\pi \over 2}, \pi] & \text{if } y \ge 0 \\\\
	///   \arctan({y \over x}) - \pi \in (-{\pi \over 2}, -{\pi \over 2}) & \text{if } y \lt 0
	/// \end{cases}
	/// $$
	#[must_use]
	fn atan2(self, other: Self) -> Self;

	/// Returns an ordering between self and other values.
	///
	/// Unlike the standard partial comparison between floating point numbers, this comparison
	/// always produces an ordering in accordance to the *totalOrder* predicate as defined in IEEE
	/// 754 (2008 revision) floating point standard. The values are ordered in following order:
	///
	///   * Negative quiet NaN
	///   * Negative signaling NaN
	///   * Negative infinity
	///   * Negative numbers
	///   * Negative subnormal numbers
	///   * Negative zero
	///   * Positive zero
	///   * Positive subnormal numbers
	///   * Positive numbers
	///   * Positive infinity
	///   * Positive signaling NaN
	///   * Positive quiet NaN
	///
	/// Note that this function does not always agree with the [`PartialOrd`] and [`PartialEq`]
	/// implementations of floating-point type. In particular, they regard negative and positive
	/// zero as equal, while [`Self::total_cmp()`] does not.
	#[must_use]
	fn total_cmp(&self, other: &Self) -> Ordering;

	/// Constructs a SIMD vector by setting all lanes to the given value.
	#[must_use]
	#[inline]
	fn splat<const N: usize>(self) -> Self::Simd<N>
	where
		LaneCount<N>: SupportedLaneCount,
	{
		Self::Simd::splat(self)
	}
}

impl<R: Real> ApproxEq<R> for R {
	fn approx_eq(&self, other: &R, epsilon: R, ulp: R::Bits) -> bool {
		Real::abs(*self - other) <= epsilon
			|| !self.is_nan()
				&& !other.is_nan()
				&& self.is_sign_negative() == self.is_sign_negative()
				&& self.to_bits().abs_sub(other.to_bits()) <= ulp
	}
}
