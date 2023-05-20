use derive_more::{Add, AddAssign, Sub, SubAssign, Neg};
use std::ops::{Mul, MulAssign};
use uom::si::f64::{Angle, Length, Area};
use uom::si::{angle::radian, length::meter};

#[derive(Copy, Clone, Debug, Default, PartialEq, Add, AddAssign, Sub, SubAssign, Neg)]
struct Vec2d {
	x: Length,
	y: Length
}

impl Mul<Vec2d> for Vec2d {
	type Output = Area;

	fn mul(self, rhs: Self) -> Self::Output {
		self.x * rhs.x + self.y * rhs.y
	}
}
impl Mul<f64> for Vec2d {
	type Output = Vec2d;

	fn mul(self, rhs: f64) -> Self::Output {
		Vec2d {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}
impl Mul<Vec2d> for f64 {
	type Output = Vec2d;

	fn mul(self, rhs: Vec2d) -> Self::Output {
		rhs * self
	}
}
impl MulAssign<f64> for Vec2d {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl Vec2d {
	pub fn new(x: Length, y: Length) -> Self {
		Vec2d {
			x: x,
			y: y
		}
	}

	pub fn horizontal(x: Length) -> Self {
		Self::new(x, Length::new::<meter>(0.))
	}

	pub fn vertical(y: Length) -> Self {
		Self::new(Length::new::<meter>(0.), y)
	}

	pub fn zero() -> Self {
		Self::new(Length::new::<meter>(0.), Length::new::<meter>(0.))
	}

	pub fn unit(theta: Angle) -> Self {
		Self::horizontal(Length::new::<meter>(1.)).rotate(theta)
	}

	pub fn rotate(&self, angle: Angle) -> Self {
		let (sin, cos) = angle.value.sin_cos();
		Self::new(
			self.x * cos - sin * self.y,
			self.x * sin + cos * self.y
		)
	}

	pub fn get_angle(&self) -> Angle {
		self.y.atan2(self.x)
	}

	pub fn get_norm(&self) -> Length {
		self.x.hypot(self.y)
	}
}