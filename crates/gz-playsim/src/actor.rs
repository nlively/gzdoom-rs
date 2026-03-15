/// Actor — the base entity in GZDoom's world simulation.
///
/// In C++: AActor (src/playsim/actor.h, p_mobj.cpp)
///
/// Design note: rather than replicating C++ inheritance, structure actors
/// as a set of components stored in parallel arrays or an ECS.
/// This file sketches the core data that every actor has.

use gz_common::name::Name;
use glam::DVec3;

/// Unique actor identifier within a map session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId(u32);

/// The minimal set of fields present on every actor (analogous to AActor's
/// non-virtual data members).
#[derive(Debug, Clone)]
pub struct ActorBase {
    pub id: ActorId,
    /// Class name (interned).
    pub class: Name,
    pub pos: DVec3,
    pub vel: DVec3,
    pub angle: f64,   // radians
    pub pitch: f64,
    pub health: i32,
    pub flags: ActorFlags,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ActorFlags: u64 {
        const SOLID       = 1 << 0;
        const SHOOTABLE   = 1 << 1;
        const NO_GRAVITY  = 1 << 2;
        const MISSILE     = 1 << 3;
        const PICKUP      = 1 << 4;
        // … many more in MF_* / MF2_* / MF3_* / MF4_* …
    }
}
