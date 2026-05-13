// Ch9 + Ch11 — Structs + Traits
//
// CONCEPTS:
//   Scene trait   — common interface for game states (menu, gameplay, gameover)
//   SceneManager  — owns the active scene and handles transitions
//   Box<dyn Scene>— trait object: store any Scene without knowing its type

use crate::input::InputState;

// ── Scene trait ───────────────────────────────────────────────

pub trait Scene {
    /// Called once per frame to update game state.
    /// Returns Some(next_scene) to trigger a transition, None to stay.
    fn update(&mut self, input: &InputState, dt: f32) -> Option<Box<dyn Scene>>;

    /// Return the scene's name (for debugging).
    fn name(&self) -> &str;
}

// ── SceneManager ──────────────────────────────────────────────

pub struct SceneManager {
    current: Box<dyn Scene>,
}

impl SceneManager {
    pub fn new(initial: Box<dyn Scene>) -> Self {
        todo!()
    }

    /// Advance the scene by one frame. Applies any transition.
    pub fn update(&mut self, input: &InputState, dt: f32) {
        todo!()
    }

    /// Return the name of the active scene.
    pub fn current_name(&self) -> &str {
        todo!()
    }
}

// ─────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    struct Dummy(&'static str);
    impl Scene for Dummy {
        fn update(&mut self, _: &InputState, _: f32) -> Option<Box<dyn Scene>> {
            None
        }
        fn name(&self) -> &str { self.0 }
    }

    #[test]
    #[ignore = "implement SceneManager::new and current_name"]
    fn test_initial_scene_name() {
        let mgr = SceneManager::new(Box::new(Dummy("menu")));
        assert_eq!(mgr.current_name(), "menu");
    }
}
