// Ch9 + Ch11 — Structs + Traits
//
// CONCEPTS:
//   Entity          — just an ID (u64); no data lives on the entity itself
//   Component trait — any data type that can be attached to an entity
//   World           — holds all entities and their component storage
//   System          — a function that operates on a set of components

// ── Entity ────────────────────────────────────────────────────

/// A unique identifier for a game object.
/// Data is stored separately in component arrays, not here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

// ── Component trait ───────────────────────────────────────────

/// Marker trait: any type that can be stored as a component.
pub trait Component: 'static {}

// ── Built-in components ───────────────────────────────────────

use crate::math::Vec2;

/// Position in 2D world space.
pub struct Position(pub Vec2);
impl Component for Position {}

/// Velocity in units per second.
pub struct Velocity(pub Vec2);
impl Component for Velocity {}

/// Axis-aligned bounding box for collision.
pub struct Collider {
    pub half_extents: Vec2,
}
impl Component for Collider {}

// ── World ─────────────────────────────────────────────────────

/// Stores all entities and their component data.
pub struct World {
    next_id: u64,
    // TODO: add component storage (one Vec per component type, or a HashMap)
}

impl World {
    pub fn new() -> Self {
        todo!()
    }

    /// Spawn a new entity and return its ID.
    pub fn spawn(&mut self) -> Entity {
        todo!()
    }

    /// Remove an entity and all its components.
    pub fn despawn(&mut self, entity: Entity) {
        todo!()
    }

    /// Return the number of live entities.
    pub fn entity_count(&self) -> usize {
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
    #[ignore = "implement World::new and spawn"]
    fn test_spawn_increases_count() {
        let mut world = World::new();
        assert_eq!(world.entity_count(), 0);
        world.spawn();
        world.spawn();
        assert_eq!(world.entity_count(), 2);
    }

    #[test]
    #[ignore = "implement World::despawn"]
    fn test_despawn_decreases_count() {
        let mut world = World::new();
        let e = world.spawn();
        world.despawn(e);
        assert_eq!(world.entity_count(), 0);
    }

    #[test]
    #[ignore = "implement spawn with unique IDs"]
    fn test_entity_ids_are_unique() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        assert_ne!(a, b);
    }
}
