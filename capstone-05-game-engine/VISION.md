# Capstone 05 — Game Engine

## What You're Building

A minimal 2D game engine — the layer of code that sits between your game logic
and the hardware. Not a game itself, but the *engine* that makes games possible:

- An **Entity-Component System (ECS)** — the architecture used by Unity, Unreal,
  and Bevy to manage thousands of game objects efficiently
- A **game loop** — the heartbeat of every game: update the world, render it, repeat
- **Collision detection** — detecting when two objects overlap
- **A simple physics model** — velocity, gravity, and movement over time
- **An event system** — keyboard and mouse input flowing through the engine

## The Real Point

Games demand performance, flexibility, and correctness all at once:

- **ECS with generics** — instead of `Player` inheriting from `Entity`
  (the object-oriented way), an entity is just an ID. Components (`Position`,
  `Velocity`, `Sprite`) are data attached to that ID. Systems process all entities
  that have certain components. This is composition over inheritance — Rust's model.

- **Operator overloading** — `Vec2(1.0, 2.0) + Vec2(3.0, 4.0)` works because you
  implement the `Add` trait. Math-heavy code becomes readable.

- **Trait objects for systems** — every system (physics, rendering, input) implements
  a `System` trait. The engine loops over `Vec<Box<dyn System>>` and calls `update()`
  on each. Runtime polymorphism without inheritance.

- **Concurrency** — rendering and physics can run on separate threads.
  Channels communicate between them without shared mutable state.

## Architecture: How the Files Connect

The engine has no files scaffolded yet beyond `main.rs`. The structure below is
what you will build:

```
main.rs
  │
  │  creates the World, registers systems, runs the game loop
  │
  └── (modules to build)
        │
        ├── world.rs       ← the ECS container
        │                     stores entities and their components
        │                     central registry: everything else reads from here
        │
        ├── components.rs  ← data only: Position, Velocity, Sprite, Collider
        │                     pure structs with no methods (data lives here,
        │                     logic lives in systems.rs)
        │
        ├── systems.rs     ← the System trait + all system implementations
        │                     PhysicsSystem: reads Velocity, writes Position
        │                     CollisionSystem: reads Collider, emits events
        │                     RenderSystem: reads Position + Sprite, draws
        │
        ├── math.rs        ← Vec2, Rect, and their operator overloads
        │                     Add, Sub, Mul for Vec2
        │                     everything geometric lives here
        │
        ├── events.rs      ← input events (KeyPress, MouseClick) and
        │                     engine events (Collision, SceneLoad)
        │                     systems emit events; other systems listen
        │
        └── loop.rs        ← the game loop: fixed timestep, delta time,
                              calls systems in order each frame
```

## Why Each File Will Exist

**`main.rs`** — Wires everything together and starts the loop. Reads like a
game config: register components, register systems, run.

**`world.rs`** — The ECS heart. Stores a `HashMap<EntityId, ComponentMap>`.
Every system reads and writes through `World`. Separating it means no system
directly holds game state — they borrow it from the `World` for one frame at a time.

**`components.rs`** — Pure data, no logic. Separated so the data definitions
can be shared by every system without importing any behavior. If you add a new
component, you touch one file.

**`systems.rs`** — All the behavior. A `System` is a trait with one method:
`fn update(&mut self, world: &mut World, dt: f32)`. Each system is a struct
that implements it. Separated from components because data and behavior change
for different reasons — and keeping them apart is the whole point of ECS.

**`math.rs`** — `Vec2`, `Rect`, matrix transforms, and their operator overloads.
Separated so math types can be imported by `components`, `systems`, and `events`
without any of those modules importing each other.

**`events.rs`** — A decoupled communication system. Systems emit events;
other systems read them next frame. Separating events from systems means
`CollisionSystem` does not need to import `AudioSystem` to tell it to play a sound —
it just emits a `CollisionEvent` and the audio system reads it.

**`loop.rs`** — Fixed-timestep game loop: accumulate time, step the physics in
fixed intervals, render at the display refresh rate. Separated from `main.rs`
because loop logic (delta time, frame cap, sleep) is a solved problem you want
to write once and forget.

## Chapters It Combines

Ch09 Structs · Ch11 Traits & Generics · Ch12 Operator Overloading ·
Ch13 Utility Traits · Ch19 Concurrency

## Mental Model

Think of the Bevy game engine (written in Rust) or Unity's ECS mode. After this
capstone you will understand why game engines are built the way they are:
ECS exists because games have thousands of objects, 60fps deadlines, and code
that changes constantly — and ECS is the architecture that handles all three.
