/// Map geometry — vertices, linedefs, sidedefs, sectors, things.
///
/// See: src/maploader/maploader.cpp, src/playsim/p_sectors.cpp

use gz_common::fixed::Fixed;
use glam::DVec2;

/// A BSP vertex.
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: DVec2,
}

/// A linedef — a wall segment connecting two vertices.
#[derive(Debug, Clone)]
pub struct Linedef {
    pub v1: u16,
    pub v2: u16,
    pub flags: u16,
    pub special: u16,
    pub tag: u16,
    pub front_sidedef: u16,
    pub back_sidedef: u16,  // 0xFFFF if one-sided
}

/// A sector — a convex region with floor/ceiling heights.
#[derive(Debug, Clone)]
pub struct Sector {
    pub floor_height: Fixed,
    pub ceil_height: Fixed,
    pub floor_tex: String,
    pub ceil_tex: String,
    pub light_level: i16,
    pub special: u16,
    pub tag: u16,
}

pub struct Map {
    pub vertices: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
    pub sectors: Vec<Sector>,
}
