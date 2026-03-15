/// High-resolution timing — analogous to i_time.h.
///
/// GZDoom runs at a fixed gametic rate (35 Hz by default) and uses a separate
/// wall-clock timer for rendering interpolation.

use std::time::Instant;

pub struct GameClock {
    start: Instant,
    /// Number of elapsed game ticks (35 Hz).
    pub gametic: u64,
    /// Fractional position between the last two game ticks (for interpolation).
    pub frac: f64,
}

impl GameClock {
    pub fn new() -> Self {
        GameClock { start: Instant::now(), gametic: 0, frac: 0.0 }
    }

    /// Returns elapsed time in seconds since the clock was created.
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for GameClock {
    fn default() -> Self { Self::new() }
}
