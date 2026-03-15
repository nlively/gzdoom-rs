/// Interned name table — analogous to FName in GZDoom.
///
/// GZDoom stores all identifiers (actor class names, lump names, etc.) in a
/// global interned string table so that comparisons are O(1) integer compares.
/// This is a stub; the real implementation will need a thread-safe interner.
///
/// See: src/common/utility/name.h

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name(u32);

impl Name {
    pub const NONE: Name = Name(0);
}
