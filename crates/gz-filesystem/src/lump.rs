/// A lump is a named blob of bytes — the atom of the virtual filesystem.
///
/// WAD lumps have 8-byte uppercase ASCII names.  PK3 lumps have full paths.

#[derive(Debug, Clone)]
pub struct LumpInfo {
    /// Lump name (normalised to uppercase, max 8 chars for WAD lumps).
    pub name: String,
    /// Index of the archive that owns this lump.
    pub archive_index: u32,
    /// Byte offset within the archive file (for lazy reading).
    pub offset: u64,
    /// Uncompressed size in bytes.
    pub size: u64,
}

/// A fully-loaded lump.
#[derive(Debug)]
pub struct Lump {
    pub info: LumpInfo,
    pub data: Vec<u8>,
}
