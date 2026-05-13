// Ch11 + Ch12 — Traits + Operator Overloading
//
// CONCEPTS:
//   AABB collision   — axis-aligned bounding box overlap test
//   Physics step     — apply velocity to position each frame
//   Restitution      — bounce coefficient on collision response

use crate::math::Vec2;

// ── AABB ──────────────────────────────────────────────────────

/// Axis-aligned bounding box defined by centre + half-extents.
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub centre: Vec2,
    pub half: Vec2,
}

impl Aabb {
    pub fn new(centre: Vec2, half: Vec2) -> Self {
        todo!()
    }

    /// Return true if this box overlaps `other`.
    pub fn overlaps(&self, other: &Aabb) -> bool {
        todo!()
    }

    /// Move the box by `delta`.
    pub fn translate(&self, delta: Vec2) -> Aabb {
        todo!()
    }
}

// ── Physics step ──────────────────────────────────────────────

/// Move `position` by `velocity * dt`.
pub fn integrate(position: Vec2, velocity: Vec2, dt: f32) -> Vec2 {
    todo!()
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "implement Aabb::overlaps"]
    fn test_overlapping_boxes() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        let b = Aabb::new(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0));
        assert!(a.overlaps(&b));
    }

    #[test]
    #[ignore = "implement Aabb::overlaps"]
    fn test_non_overlapping_boxes() {
        let a = Aabb::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        let b = Aabb::new(Vec2::new(5.0, 5.0), Vec2::new(1.0, 1.0));
        assert!(!a.overlaps(&b));
    }

    #[test]
    #[ignore = "implement integrate"]
    fn test_integrate_moves_position() {
        let pos = Vec2::new(0.0, 0.0);
        let vel = Vec2::new(10.0, 0.0);
        let result = integrate(pos, vel, 0.1);
        assert_eq!(result, Vec2::new(1.0, 0.0));
    }
}
