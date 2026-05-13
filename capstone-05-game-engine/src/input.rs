// Ch9 — Structs | Ch10 — Enums & Patterns
//
// CONCEPTS:
//   Enum for key codes     — named keys instead of raw integers
//   HashSet for held keys  — O(1) "is this key down?" check
//   InputState struct      — snapshot of keyboard/mouse for one frame

use std::collections::HashSet;

// ── Key ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Space,
    Escape,
}

// ── InputState ────────────────────────────────────────────────

/// Keyboard snapshot for one frame.
pub struct InputState {
    held: HashSet<Key>,
    just_pressed: HashSet<Key>,
}

impl InputState {
    pub fn new() -> Self {
        todo!()
    }

    /// Called by the platform layer when a key goes down.
    pub fn press(&mut self, key: Key) {
        todo!()
    }

    /// Called by the platform layer when a key goes up.
    pub fn release(&mut self, key: Key) {
        todo!()
    }

    /// True while the key is held down.
    pub fn is_held(&self, key: Key) -> bool {
        todo!()
    }

    /// True only on the frame the key was first pressed.
    pub fn just_pressed(&self, key: Key) -> bool {
        todo!()
    }

    /// Clear just_pressed — call at the end of each frame.
    pub fn end_frame(&mut self) {
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
    #[ignore = "implement press / is_held"]
    fn test_key_held_after_press() {
        let mut state = InputState::new();
        state.press(Key::Space);
        assert!(state.is_held(Key::Space));
    }

    #[test]
    #[ignore = "implement release"]
    fn test_key_not_held_after_release() {
        let mut state = InputState::new();
        state.press(Key::Left);
        state.release(Key::Left);
        assert!(!state.is_held(Key::Left));
    }

    #[test]
    #[ignore = "implement just_pressed / end_frame"]
    fn test_just_pressed_clears_after_end_frame() {
        let mut state = InputState::new();
        state.press(Key::Up);
        assert!(state.just_pressed(Key::Up));
        state.end_frame();
        assert!(!state.just_pressed(Key::Up));
    }
}
