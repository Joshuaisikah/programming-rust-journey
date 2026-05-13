// Ch12 — Operator Overloading
//
// CONCEPTS:
//   Add / Sub / Mul — implement +, -, * between Vec2 values
//   PartialEq       — element-wise equality
//   Copy            — Vec2 is small; derive Copy so it passes by value

use std::ops::{Add, Mul, Sub};

// ── Vec2 ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        todo!()
    }

    pub fn zero() -> Self {
        todo!()
    }

    /// Euclidean length.
    pub fn length(&self) -> f32 {
        todo!()
    }

    /// Return a unit vector in the same direction.
    /// Returns Vec2::zero() if length is 0.
    pub fn normalize(&self) -> Self {
        todo!()
    }

    /// Dot product.
    pub fn dot(&self, other: Vec2) -> f32 {
        todo!()
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        todo!()
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        todo!()
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        todo!()
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Vec2::new"]
    fn test_new() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    #[ignore = "implement Vec2::length"]
    fn test_length() {
        assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
    }

    #[test]
    #[ignore = "implement Vec2::normalize"]
    fn test_normalize() {
        let n = Vec2::new(3.0, 0.0).normalize();
        assert_eq!(n.x, 1.0);
        assert_eq!(n.y, 0.0);
    }

    #[test]
    #[ignore = "implement Add for Vec2"]
    fn test_add() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);
        assert_eq!(a + b, Vec2::new(4.0, 6.0));
    }

    #[test]
    #[ignore = "implement Mul<f32> for Vec2"]
    fn test_scale() {
        assert_eq!(Vec2::new(2.0, 3.0) * 2.0, Vec2::new(4.0, 6.0));
    }
}
