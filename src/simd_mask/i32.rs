// Copyright © 2021 Rouven Spreckels <rs@qu1x.dev>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::SimdMask;
use core::simd::{LaneCount, Mask, Select, SupportedLaneCount};

impl<const LANES: usize> SimdMask<LANES> for Mask<i32, LANES>
where
	LaneCount<LANES>: SupportedLaneCount,
{
	fn splat(value: bool) -> Self {
		Self::splat(value)
	}

	fn from_array(array: [bool; LANES]) -> Self {
		Self::from(array)
	}
	fn to_array(self) -> [bool; LANES] {
		self.to_array()
	}

	fn all(self) -> bool {
		self.all()
	}
	fn any(self) -> bool {
		self.any()
	}

	fn set(&mut self, lane: usize, value: bool) {
		self.set(lane, value);
	}
	fn test(&self, lane: usize) -> bool {
		self.test(lane)
	}

	fn select<S: Select<Self>>(self, true_values: S, false_values: S) -> S {
		self.select(true_values, false_values)
	}
}
