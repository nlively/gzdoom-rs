/// gz-filesystem — Virtual filesystem for WAD, PK3 (ZIP), and directories.
///
/// C++ analogues:
///   src/common/filesystem/  (filesystem.h, wadfile, zipfile, …)
///
/// A WAD file is a flat archive of named "lumps".  PK3 files are ZIPs.
/// The virtual FS merges multiple archives into a unified namespace with
/// precedence rules (later-loaded archives override earlier ones).

pub mod lump;
pub mod wad;
pub mod pk3;
pub mod vfs;
