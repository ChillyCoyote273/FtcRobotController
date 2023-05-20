#![allow(dead_code)]

pub mod geometry;

use uom::si::f64::Angle;
use uom::si::angle::radian;

use std::f64::consts::PI;

pub fn normalize(angle: Angle) -> Angle {
	let mut radians = angle.value;
	radians %= 2. * PI;
	Angle::new::<radian>(
		if radians >= PI {
			radians - 2. * PI
		}
		else if radians < -PI {
			radians + 2. * PI
		}
		else {
			radians
		}
	)
}