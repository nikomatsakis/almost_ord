// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Trait for types that form a total order when a few values are disallowed.
///
/// `is_outside_order()` must return `true` for these outliers and `false` for anything else.
///
/// `std::cmp::PartialOrd::partial_cmp(a,b)` must return `Some(_)` if a,b are both inside order and `None` if only one is outside order. Return value for two variables outside order is undefined.
pub trait AlmostOrd: PartialOrd<Self> + PartialEq<Self> {
	fn is_outside_order(&self) -> bool;
}

impl<'a, A> AlmostOrd for &'a A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(*self).is_outside_order()
	}
}

impl<'a, A> AlmostOrd for &'a mut A where A: AlmostOrd {
	fn is_outside_order(&self) -> bool {
		(**self).is_outside_order()
	}
}

impl AlmostOrd for f64 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}

impl AlmostOrd for f32 {
	fn is_outside_order(&self) -> bool {
		self.is_nan()
	}
}
