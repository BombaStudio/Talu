use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<raylib::prelude::Vector2> for Vector2 {
    fn from(v: raylib::prelude::Vector2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Vector2> for raylib::prelude::Vector2 {
    fn from(v: Vector2) -> raylib::prelude::Vector2 {
        raylib::prelude::Vector2 { x: v.x, y: v.y }
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            *self / len
        } else {
            *self
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).length()
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, vec: Vector2) -> Vector2 {
        Vector2 { x: self * vec.x, y: self * vec.y }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self { x: self.x / scalar, y: self.y / scalar }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}