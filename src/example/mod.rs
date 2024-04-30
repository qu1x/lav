// Copyright © 2021-2024 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Portably SIMD-optimized 3D rotator implementation generic over lane type [`f32`] and [`f64`].
//!
//! ```
//! #![allow(non_snake_case)]
//! #![feature(portable_simd)]
//!
//! use core::{
//! 	mem::transmute,
//! 	ops::{
//! 		Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, ShlAssign, Sub, SubAssign,
//! 	},
//! };
//! use lav::{swizzle, ApproxEq, Real, SimdMask, SimdReal};
//!
//! #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! #[repr(transparent)]
//! pub struct Rotator3<R: Real> {
//! 	wxyz: R::Simd<4>,
//! }
//!
//! impl<R: Real> Rotator3<R> {
//! 	pub fn new(alpha: R, x: R, y: R, z: R) -> Self {
//! 		let (sin, cos) = (alpha * -R::FRAC_1_2).sin_cos();
//! 		(Self::from([R::ZERO, x, y, z]).unit() * sin).set_w(cos)
//! 	}
//! 	pub fn from_wxyz(wxyz: [R; 4]) -> Self {
//! 		Self { wxyz: wxyz.into() }
//! 	}
//! 	pub fn from_xyzw(xyzw: [R; 4]) -> Self {
//! 		Self {
//! 			wxyz: R::Simd::from_array(xyzw).simd_rotate_right::<1>(),
//! 		}
//! 	}
//! 	pub fn norm(&self) -> R {
//! 		self.norm_squared().sqrt()
//! 	}
//! 	pub fn norm_squared(&self) -> R {
//! 		(self.wxyz * self.wxyz).reduce_sum()
//! 	}
//! 	pub fn unit(self) -> Self {
//! 		self / self.norm()
//! 	}
//! 	pub fn inv(self) -> Self {
//! 		self.rev() / self.norm_squared()
//! 	}
//! 	pub fn rev(self) -> Self {
//! 		let tfff = R::Simd::mask_flag(0, true);
//! 		Self {
//! 			wxyz: tfff.select(self.wxyz, -self.wxyz),
//! 		}
//! 	}
//! 	pub fn constrain(self) -> Self {
//! 		if self.w().is_sign_negative() {
//! 			-self
//! 		} else {
//! 			self
//! 		}
//! 	}
//! 	pub fn to_wxyz(self) -> [R; 4] {
//! 		self.wxyz.to_array()
//! 	}
//! 	pub fn to_xyzw(self) -> [R; 4] {
//! 		self.wxyz.simd_rotate_left::<1>().to_array()
//! 	}
//! 	pub fn w(&self) -> R {
//! 		self.wxyz[0]
//! 	}
//! 	pub fn x(&self) -> R {
//! 		self.wxyz[1]
//! 	}
//! 	pub fn y(&self) -> R {
//! 		self.wxyz[2]
//! 	}
//! 	pub fn z(&self) -> R {
//! 		self.wxyz[3]
//! 	}
//! 	pub fn set_w(mut self, w: R) -> Self {
//! 		self.wxyz[0] = w;
//! 		self
//! 	}
//! 	pub fn set_x(mut self, x: R) -> Self {
//! 		self.wxyz[1] = x;
//! 		self
//! 	}
//! 	pub fn set_y(mut self, y: R) -> Self {
//! 		self.wxyz[2] = y;
//! 		self
//! 	}
//! 	pub fn set_z(mut self, z: R) -> Self {
//! 		self.wxyz[3] = z;
//! 		self
//! 	}
//! 	pub fn point_fn(self) -> impl Fn(&mut Point3<R>) {
//! 		// +(+[+1ww+1zz+(+1xx+1yy)]~w+[+0zy+0wx]~w+[+0yw-0zx]~w)e123
//! 		// +(+[+1xx+1ww-(+1yy+1zz)]~X+[+2wz+2xy]~Y+[+2zx-2wy]~Z)e032
//! 		// +(+[+1yy+1ww-(+1zz+1xx)]~Y+[+2wx+2yz]~Z+[+2xy-2wz]~X)e013
//! 		// +(+[+1zz+1ww-(+1xx+1yy)]~Z+[+2wy+2zx]~X+[+2yz-2wx]~Y)e021
//! 		let wxyz = self.wxyz;
//! 		let zwww = swizzle!(wxyz, [3, 0, 0, 0]);
//! 		let xyzx = swizzle!(wxyz, [1, 2, 3, 1]);
//! 		let yzxy = swizzle!(wxyz, [2, 3, 1, 2]);
//! 		let zttt = R::Simd::from_array([R::ZERO, R::TWO, R::TWO, R::TWO]);
//! 		let fttt = R::Simd::mask_flag(0, false);
//! 		let pin0 = xyzx.mul_add(xyzx, yzxy * yzxy);
//! 		let pin0 = zwww.mul_add(zwww, fttt.negate(pin0));
//! 		let pin0 = wxyz.mul_add(wxyz, pin0);
//! 		let pin1 = zwww.mul_add(yzxy, wxyz * xyzx) * zttt;
//! 		let pin2 = yzxy.mul_add(wxyz, -(zwww * xyzx)) * zttt;
//! 		move |point3| {
//! 			let wXYZ = point3.wXYZ;
//! 			let wYZX = swizzle!(wXYZ, [0, 2, 3, 1]);
//! 			let wZXY = swizzle!(wXYZ, [0, 3, 1, 2]);
//! 			let wXYZ = pin0.mul_add(wXYZ, pin1.mul_add(wYZX, pin2 * wZXY));
//! 			point3.wXYZ = wXYZ;
//! 		}
//! 	}
//! 	pub fn points_fn(self) -> impl Fn(&mut [Point3<R>]) {
//! 		let rotate = self.point_fn();
//! 		move |points| {
//! 			for point3 in points {
//! 				rotate(point3);
//! 			}
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> Default for Rotator3<R> {
//! 	fn default() -> Self {
//! 		Self::from([R::ONE, R::ZERO, R::ZERO, R::ZERO])
//! 	}
//! }
//!
//! impl<R: Real> From<[R; 4]> for Rotator3<R> {
//! 	fn from(wxyz: [R; 4]) -> Self {
//! 		Self::from_wxyz(wxyz)
//! 	}
//! }
//!
//! impl<R: Real> Into<[R; 4]> for Rotator3<R> {
//! 	fn into(self) -> [R; 4] {
//! 		self.to_wxyz()
//! 	}
//! }
//!
//! impl<R: Real> ApproxEq<R> for Rotator3<R> {
//! 	fn approx_eq(&self, other: &Self, epsilon: R, ulp: R::Bits) -> bool {
//! 		self.wxyz.approx_eq(&other.wxyz, epsilon, ulp)
//! 	}
//! }
//!
//! impl<R: Real> Add for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn add(self, other: Self) -> Self::Output {
//! 		Self {
//! 			wxyz: self.wxyz + other.wxyz,
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> AddAssign for Rotator3<R> {
//! 	fn add_assign(&mut self, other: Self) {
//! 		*self = *self + other;
//! 	}
//! }
//!
//! impl<R: Real> Div<R> for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn div(self, other: R) -> Self::Output {
//! 		Self {
//! 			wxyz: self.wxyz / other.splat(),
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> DivAssign<R> for Rotator3<R> {
//! 	fn div_assign(&mut self, other: R) {
//! 		*self = *self / other;
//! 	}
//! }
//!
//! impl<R: Real> Mul for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn mul(self, other: Self) -> Self::Output {
//! 		// +(+1LwRw-1LxRx-(+1LyRy+1LzRz))e
//! 		// +(+1LwRx-1LyRz+(+1LxRw+1LzRy))e23
//! 		// +(+1LwRy-1LzRx+(+1LxRz+1LyRw))e31
//! 		// +(+1LwRz-1LxRy+(+1LyRx+1LzRw))e12
//! 		let tfff = R::Simd::mask_flag(0, true);
//! 		let wxyz = swizzle!(self.wxyz, [0, 0, 0, 0]).mul_add(
//! 			other.wxyz,
//! 			swizzle!(self.wxyz, [1, 2, 3, 1]).mul_add(
//! 				-swizzle!(other.wxyz, [1, 3, 1, 2]),
//! 				tfff.negate(swizzle!(self.wxyz, [2, 1, 1, 2]).mul_add(
//! 					swizzle!(other.wxyz, [2, 0, 3, 1]),
//! 					swizzle!(self.wxyz, [3, 3, 2, 3]) * swizzle!(other.wxyz, [3, 2, 0, 0]),
//! 				)),
//! 			),
//! 		);
//! 		Self { wxyz }
//! 	}
//! }
//!
//! impl<R: Real> Mul<R> for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn mul(self, other: R) -> Self::Output {
//! 		Self {
//! 			wxyz: self.wxyz * other.splat(),
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> MulAssign<R> for Rotator3<R> {
//! 	fn mul_assign(&mut self, other: R) {
//! 		*self = *self * other;
//! 	}
//! }
//!
//! impl<R: Real> Neg for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn neg(self) -> Self::Output {
//! 		Self { wxyz: -self.wxyz }
//! 	}
//! }
//!
//! impl<R: Real> Sub for Rotator3<R> {
//! 	type Output = Self;
//!
//! 	fn sub(self, other: Self) -> Self::Output {
//! 		Self {
//! 			wxyz: self.wxyz - other.wxyz,
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> SubAssign for Rotator3<R> {
//! 	fn sub_assign(&mut self, other: Self) {
//! 		*self = *self - other;
//! 	}
//! }
//!
//! #[derive(Debug, Copy, Clone, PartialEq, Eq)]
//! #[repr(transparent)]
//! pub struct Point3<R: Real> {
//! 	wXYZ: R::Simd<4>,
//! }
//!
//! impl<R: Real> Point3<R> {
//! 	pub fn new(w: R, X: R, Y: R, Z: R) -> Self {
//! 		Self::from([w, X, Y, Z])
//! 	}
//! 	pub fn from_wXYZ(wXYZ: [R; 4]) -> Self {
//! 		Self { wXYZ: wXYZ.into() }
//! 	}
//! 	pub fn from_XYZw(XYZw: [R; 4]) -> Self {
//! 		Self {
//! 			wXYZ: R::Simd::from_array(XYZw).simd_rotate_right::<1>(),
//! 		}
//! 	}
//! 	pub fn as_points(points: &[R]) -> &[Self] {
//! 		let (prefix, points, suffix) = R::as_simd::<4>(points);
//! 		assert!(prefix.is_empty() && suffix.is_empty(), "misaligned");
//! 		// Safe due to `#[repr(transparent)]`.
//! 		unsafe { transmute::<&[R::Simd<4>], &[Point3<R>]>(points) }
//! 	}
//! 	pub fn as_points_mut(points: &mut [R]) -> &mut [Self] {
//! 		let (prefix, points, suffix) = R::as_simd_mut::<4>(points);
//! 		assert!(prefix.is_empty() && suffix.is_empty(), "misaligned");
//! 		// Safe due to `#[repr(transparent)]`.
//! 		unsafe { transmute::<&mut [R::Simd<4>], &mut [Point3<R>]>(points) }
//! 	}
//! 	pub fn norm(&self) -> R {
//! 		self.w().abs()
//! 	}
//! 	pub fn norm_squared(&self) -> R {
//! 		let w = self.w();
//! 		w * w
//! 	}
//! 	pub fn unit(self) -> Self {
//! 		self / self.norm()
//! 	}
//! 	pub fn inv(self) -> Self {
//! 		self.rev() / self.norm_squared()
//! 	}
//! 	pub fn rev(self) -> Self {
//! 		-self
//! 	}
//! 	pub fn to_wXYZ(self) -> [R; 4] {
//! 		self.wXYZ.to_array()
//! 	}
//! 	pub fn to_XYZw(self) -> [R; 4] {
//! 		self.wXYZ.simd_rotate_left::<1>().to_array()
//! 	}
//! 	pub fn w(&self) -> R {
//! 		self.wXYZ[0]
//! 	}
//! 	pub fn X(&self) -> R {
//! 		self.wXYZ[1]
//! 	}
//! 	pub fn Y(&self) -> R {
//! 		self.wXYZ[2]
//! 	}
//! 	pub fn Z(&self) -> R {
//! 		self.wXYZ[3]
//! 	}
//! 	pub fn set_w(mut self, w: R) -> Self {
//! 		self.wXYZ[0] = w;
//! 		self
//! 	}
//! 	pub fn set_X(mut self, X: R) -> Self {
//! 		self.wXYZ[1] = X;
//! 		self
//! 	}
//! 	pub fn set_Y(mut self, Y: R) -> Self {
//! 		self.wXYZ[2] = Y;
//! 		self
//! 	}
//! 	pub fn set_Z(mut self, Z: R) -> Self {
//! 		self.wXYZ[3] = Z;
//! 		self
//! 	}
//! }
//!
//! impl<R: Real> Default for Point3<R> {
//! 	fn default() -> Self {
//! 		Self::from([R::ONE, R::ZERO, R::ZERO, R::ZERO])
//! 	}
//! }
//!
//! impl<R: Real> From<[R; 4]> for Point3<R> {
//! 	fn from(wXYZ: [R; 4]) -> Self {
//! 		Self::from_wXYZ(wXYZ)
//! 	}
//! }
//!
//! impl<R: Real> Into<[R; 4]> for Point3<R> {
//! 	fn into(self) -> [R; 4] {
//! 		self.to_wXYZ()
//! 	}
//! }
//!
//! impl<R: Real> ApproxEq<R> for Point3<R> {
//! 	fn approx_eq(&self, other: &Self, epsilon: R, ulp: R::Bits) -> bool {
//! 		self.wXYZ.approx_eq(&other.wXYZ, epsilon, ulp)
//! 	}
//! }
//!
//! impl<R: Real> Add for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn add(self, other: Self) -> Self::Output {
//! 		Self {
//! 			wXYZ: self.wXYZ + other.wXYZ,
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> AddAssign for Point3<R> {
//! 	fn add_assign(&mut self, other: Self) {
//! 		*self = *self + other;
//! 	}
//! }
//!
//! impl<R: Real> Div<R> for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn div(self, other: R) -> Self::Output {
//! 		Self {
//! 			wXYZ: self.wXYZ / other.splat(),
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> DivAssign<R> for Point3<R> {
//! 	fn div_assign(&mut self, other: R) {
//! 		*self = *self / other;
//! 	}
//! }
//!
//! impl<R: Real> Mul<R> for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn mul(self, other: R) -> Self::Output {
//! 		Self {
//! 			wXYZ: self.wXYZ * other.splat(),
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> MulAssign<R> for Point3<R> {
//! 	fn mul_assign(&mut self, other: R) {
//! 		*self = *self * other;
//! 	}
//! }
//!
//! impl<R: Real> Neg for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn neg(self) -> Self::Output {
//! 		Self { wXYZ: -self.wXYZ }
//! 	}
//! }
//!
//! impl<R: Real> Shl<Rotator3<R>> for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn shl(mut self, other: Rotator3<R>) -> Self::Output {
//! 		other.point_fn()(&mut self);
//! 		self
//! 	}
//! }
//!
//! impl<R: Real> ShlAssign<Rotator3<R>> for Point3<R> {
//! 	fn shl_assign(&mut self, other: Rotator3<R>) {
//! 		*self = *self << other
//! 	}
//! }
//!
//! impl<R: Real> Sub for Point3<R> {
//! 	type Output = Self;
//!
//! 	fn sub(self, other: Self) -> Self::Output {
//! 		Self {
//! 			wXYZ: self.wXYZ - other.wXYZ,
//! 		}
//! 	}
//! }
//!
//! impl<R: Real> SubAssign for Point3<R> {
//! 	fn sub_assign(&mut self, other: Self) {
//! 		*self = *self - other;
//! 	}
//! }
//!
//! let r000_ = Rotator3::default();
//! let r030x = Rotator3::new(030f64.to_radians(), 1.0, 0.0, 0.0);
//! let r060x = Rotator3::new(060f64.to_radians(), 1.0, 0.0, 0.0);
//! let r330x = Rotator3::new(330f64.to_radians(), 1.0, 0.0, 0.0);
//! assert!((r030x * r030x).approx_eq(&r060x, 0.0, 0));
//! assert!((r030x * 42.0).unit().approx_eq(&r030x, 0.0, 0));
//! assert!(((r030x * 42.0) * (r030x * 42.0).inv()).approx_eq(&r000_, f64::EPSILON, 0));
//! assert!((r030x * r030x.rev()).approx_eq(&Rotator3::default(), f64::EPSILON, 0));
//! assert!(r330x.constrain().approx_eq(&r030x.rev(), 0.0, 5));
//!
//! let r090x = Rotator3::new(090f64.to_radians(), 1.0, 0.0, 0.0);
//! let x5 = Point3::new(1.0, 5.0, 0.0, 0.0);
//! let y5 = Point3::new(1.0, 0.0, 5.0, 0.0);
//! let z5 = Point3::new(1.0, 0.0, 0.0, 5.0);
//! assert!((x5 << r090x).approx_eq(&x5, 0.0, 0));
//! assert!((y5 << r090x).approx_eq(&z5, 5.0 * f64::EPSILON, 0));
//! ```
