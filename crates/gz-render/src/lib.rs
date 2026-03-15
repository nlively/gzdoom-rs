/// gz-render — Vulkan-based 2D/3D hardware renderer.
///
/// C++ analogues:
///   libraries/ZVulkan/   (Vulkan abstraction, device/swapchain management)
///   src/rendering/       (hwrenderer/, swrenderer/, r_utility.cpp)
///   src/common/rendering/
///
/// Strategy:
///   Use `ash` for raw Vulkan bindings (same philosophy as ZVulkan — thin
///   wrapper, full control) and `gpu-allocator` for memory management.
///   The software renderer can come later; start with hardware only.

pub mod instance;
pub mod device;
