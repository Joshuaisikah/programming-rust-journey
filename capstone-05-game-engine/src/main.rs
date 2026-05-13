// capstone-05-game-engine — 2D Game Engine with ECS
//
// Combines: Ch9 Structs | Ch11 Traits | Ch12 Operator Overloading
//           Ch13 Utility Traits | Ch19 Concurrency
//
// What you're building:
//   A minimal 2D game engine using an Entity Component System.
//   Entities are plain IDs. Data lives in components. Systems process them.

fn main() {
    println!("Game Engine — Capstone 5");
    println!();
    println!("Implement the modules in order:");
    println!("  1. src/math.rs    — Vec2: Add, Sub, Mul, length, normalize");
    println!("  2. src/ecs.rs     — Entity, World: spawn, despawn, components");
    println!("  3. src/physics.rs — Aabb collision, integrate(pos, vel, dt)");
    println!("  4. src/input.rs   — InputState: press, release, just_pressed");
    println!("  5. src/scene.rs   — Scene trait, SceneManager transitions");
    println!();
    println!("Run `cargo test -p capstone-05-game-engine` to track progress.");

    // Uncomment once implemented:
    //
    // use capstone_05_game_engine::{ecs::World, input::{InputState, Key}};
    //
    // let mut world = World::new();
    // let player = world.spawn();
    // println!("Spawned player entity: {:?}", player);
    //
    // let mut input = InputState::new();
    // input.press(Key::Space);
    // println!("Space held: {}", input.is_held(Key::Space));
}
