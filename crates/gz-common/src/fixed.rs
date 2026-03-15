/// Fixed-point arithmetic — analogous to fixed_t (16.16) in classic Doom.
///
/// GZDoom has largely migrated off fixed_t internally, but WAD data and
/// some legacy interfaces still use it.  Prefer f64 for new code.
///
/// See: src/common/utility/m_fixed.h

/// A 16.16 signed fixed-point number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fixed(i32);

impl Fixed {
    pub const FRACBITS: u32 = 16;
    pub const FRACUNIT: i32 = 1 << Self::FRACBITS;

    pub fn from_raw(raw: i32) -> Self { Fixed(raw) }
    pub fn from_int(n: i32) -> Self { Fixed(n << Self::FRACBITS) }
    pub fn to_f64(self) -> f64 { self.0 as f64 / Self::FRACUNIT as f64 }
    pub fn raw(self) -> i32 { self.0 }
}
