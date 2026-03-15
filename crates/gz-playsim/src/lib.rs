/// gz-playsim — Game simulation: actors, map geometry, BSP, physics.
///
/// C++ analogues:
///   src/playsim/     (p_mobj.cpp 8.8K, p_map.cpp 7.3K, p_acs.cpp 11K, …)
///   src/maploader/   (maploader.cpp 3.5K)
///   src/g_level.cpp
///
/// The actor system (AActor) is a deep C++ class hierarchy.  Rust has no
/// inheritance.  The two main approaches are:
///
///   1. ECS (Entity Component System): use `hecs` or `bevy_ecs` for actors,
///      with components for position, velocity, health, sprite, etc.
///      Behaviour (state machine, actions) becomes systems.
///
///   2. Trait objects + downcasting: define `Actor` as a trait, concrete
///      types implement it.  Downcasting via `std::any::Any`.  Closer to
///      the C++ structure but more painful in practice.
///
/// Recommendation: ECS.  It maps cleanly to Doom's think-tick model.

pub mod map;
pub mod actor;
pub mod bsp;
