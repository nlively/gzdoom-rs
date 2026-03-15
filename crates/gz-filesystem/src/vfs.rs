/// Virtual Filesystem — merges WADs, PK3s, and directories with precedence.
///
/// Analogous to FileSystem in src/common/filesystem/filesystem.h.
///
/// Archives are loaded in order; later archives shadow earlier ones for lumps
/// with the same name (PWAD override semantics).

use crate::lump::LumpInfo;

pub struct VirtualFs {
    /// All lumps from all loaded archives, in load order.
    lumps: Vec<LumpInfo>,
}

impl VirtualFs {
    pub fn new() -> Self {
        VirtualFs { lumps: Vec::new() }
    }

    pub fn add_lumps(&mut self, new_lumps: impl IntoIterator<Item = LumpInfo>) {
        self.lumps.extend(new_lumps);
    }

    /// Find the last (highest-precedence) lump with the given name.
    pub fn find_lump(&self, name: &str) -> Option<&LumpInfo> {
        let upper = name.to_uppercase();
        self.lumps.iter().rev().find(|l| l.name == upper)
    }

    pub fn lump_count(&self) -> usize {
        self.lumps.len()
    }
}

impl Default for VirtualFs {
    fn default() -> Self { Self::new() }
}
