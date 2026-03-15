/// Engine-level input events — analogous to event_t in d_event.h.
///
/// winit delivers OS events; this module translates them into GZDoom's
/// internal event representation.

#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyDown { key: Key },
    KeyUp { key: Key },
    MouseMove { dx: f64, dy: f64 },
    MouseButton { button: MouseButton, pressed: bool },
    Quit,
}

/// Simplified key enum — will need to be expanded to match GZDoom's full list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Escape, Return, Space, Tab,
    Left, Right, Up, Down,
    Char(char),
    F(u8),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton { Left, Right, Middle }
