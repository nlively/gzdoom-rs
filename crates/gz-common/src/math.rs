/// Re-export glam types under the names GZDoom code uses.
///
/// GZDoom uses DVector2/3/4, FVector2/3/4, DAngle, etc.
/// In Rust we just use glam directly; aliases live here for reference.
pub use glam::{DVec2, DVec3, DVec4, Vec2, Vec3, Vec4, Mat4, Quat};
