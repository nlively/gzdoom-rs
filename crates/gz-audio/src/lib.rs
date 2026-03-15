/// gz-audio — Sound effects and music playback.
///
/// C++ analogues:
///   src/common/audio/sound/   (OpenAL-based positional audio)
///   src/common/audio/music/   (ZMusic integration: OPL, MIDI, MOD, …)
///
/// Strategy:
///   `cpal` owns the output device and stream.
///   `rodio` handles decoding and mixing for sound effects.
///   Music (OPL synthesis, MIDI, tracker modules) is a major sub-project;
///   consider wrapping libopnmidi/fluidsynth via FFI or porting the ZMusic
///   library incrementally.

pub mod sound;
pub mod music;
