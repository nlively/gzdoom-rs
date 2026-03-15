# GZDoom → Rust Port: Master Plan

## Scale reality check

| Component | C++ lines | Notes |
|-----------|-----------|-------|
| `src/` (game engine) | ~435K | What you're porting |
| `libraries/ZMusic` | ~478K | Music/MIDI subsystem — port last or wrap via FFI |
| `libraries/ZVulkan` | bundled | Thin Vulkan wrapper — `ash` replaces it |
| `libraries/ZWidget` | bundled | UI widgets — port alongside menus |
| **Total** | ~913K+ | Multi-year project |

This is a marathon. Plan accordingly.

---

## Workspace layout

```
gzdoom-rs/
├── Cargo.toml              # workspace root + shared dep versions
└── crates/
    ├── gz-common/          # math, types, containers, fixed-point
    ├── gz-filesystem/      # WAD, PK3, VFS
    ├── gz-platform/        # window, input, timing
    ├── gz-audio/           # sound + music
    ├── gz-render/          # Vulkan renderer
    ├── gz-playsim/         # actors, map, BSP, physics
    ├── gz-scripting/       # ZScript VM, ACS, DECORATE compiler
    └── gzdoom/             # main binary / game loop
```

---

## Dependency map (build order)

```
gz-common
    └── gz-filesystem
            ├── gz-platform
            ├── gz-audio
            ├── gz-render
            │       └── gz-platform
            ├── gz-scripting
            └── gz-playsim
                    └── gz-scripting
                            └── gzdoom (binary)
```

Always work bottom-up. Each crate should be independently testable before you build on it.

---

## Phase roadmap

### Phase 0 — Tooling (done)
- [x] Cargo workspace scaffolded
- [x] All crates compile clean
- [ ] CI (GitHub Actions: `cargo check`, `cargo test`, `cargo clippy`)
- [ ] `rustfmt` + `clippy` configured (`.clippy.toml`)

---

### Phase 1 — `gz-common`: Core utilities
**Analogues**: `src/common/utility/` (zstring.h, tarray.h, name.h, vectors.h, m_fixed.h)

Key tasks:
- [ ] Name interning table (FName → `Name(u32)`) with thread-safe global interner
- [ ] Fixed-point math (16.16) — needed for reading raw WAD data
- [ ] Palette/colour types (`PalEntry`, `ColorRGB`)
- [ ] Angle types (BAM — Binary Angle Measure, degrees, radians wrappers)
- [ ] `TArray<T>` → just `Vec<T>` with helpers; no custom type needed
- [ ] String utilities (GZDoom uses a custom `FString`; just use `String`/`&str`)

**Milestone**: unit tests pass for name interning, fixed-point arithmetic.

---

### Phase 2 — `gz-filesystem`: Virtual Filesystem
**Analogues**: `src/common/filesystem/` (~15 files, self-contained)

Key tasks:
- [ ] WAD reader (directory parsing, lump lookup by name)
- [ ] PK3/ZIP reader (using the `zip` crate)
- [ ] Directory container (load loose files from a folder)
- [ ] `VirtualFs` merge layer (PWAD override semantics)
- [ ] Lump namespace support (markers like `P_START`/`P_END`)

**Milestone**: load `doom.wad`, enumerate lumps, read a flat by name.

> **This is the recommended starting point** — it's self-contained, well-defined, has no
> external UI or rendering dependencies, and gives you something to test immediately.
> `cargo run -- /path/to/doom.wad` already prints the lump table.

---

### Phase 3 — `gz-platform`: Window and Input
**Analogues**: `src/common/platform/posix/`, `src/common/engine/i_time.h`

Key tasks:
- [ ] Create a window with `winit`
- [ ] Game clock at 35 Hz (gametic counter + fractional interpolation position)
- [ ] Input event translation: `winit::event` → `InputEvent`
- [ ] Mouse capture (relative mode for FPS look)
- [ ] Key binding table

**Milestone**: open a window, handle quit/resize, print key events to console.

---

### Phase 4 — `gz-render`: Vulkan bootstrap
**Analogues**: `libraries/ZVulkan/` (Vulkan instance, device, swapchain)

Key tasks:
- [ ] `ash::Entry` + `VkInstance` creation
- [ ] Surface from `winit` raw handle (`ash::ext::surface`)
- [ ] Physical device selection (prefer discrete GPU, check for required features)
- [ ] Logical device + graphics/present queues
- [ ] Swapchain creation + image views
- [ ] Render pass + framebuffers
- [ ] Basic triangle (the "hello world" of Vulkan)
- [ ] Clear screen with configurable colour

**Milestone**: window shows a solid-colour frame without validation errors.

---

### Phase 5 — `gz-filesystem` extension: Texture system
**Analogues**: `src/common/textures/`

Key tasks:
- [ ] Doom picture format decoder (column-based RLE)
- [ ] Flat decoder (raw 64×64 bytes)
- [ ] TEXTURE1/TEXTURE2 composite texture builder
- [ ] PNG/JPG loading (using `image` crate) for hi-res textures
- [ ] Texture manager: name → GPU image (upload via staging buffer)

**Milestone**: render a Doom wall texture to the screen.

---

### Phase 6 — `gz-playsim` Part 1: Map loading
**Analogues**: `src/maploader/maploader.cpp` (3.5K lines)

Key tasks:
- [ ] Parse THINGS, LINEDEFS, SIDEDEFS, VERTEXES, SEGS, SSECTORS, NODES, SECTORS
- [ ] Handle both Doom and Hexen map formats (different linedef/thing layouts)
- [ ] UDMF map format (text-based, needs a simple parser)
- [ ] Build the sector graph (portal relationships for 3D floors later)
- [ ] BSP node tree

**Milestone**: parse E1M1, print sector/linedef counts.

---

### Phase 7 — `gz-render` Part 2: Software renderer
**Analogues**: `src/rendering/swrenderer/`

Doing the software renderer before the hardware renderer is pedagogically useful —
it forces you to understand the BSP traversal and column renderer before adding GPU complexity.

Key tasks:
- [ ] BSP traversal (front-to-back sector ordering)
- [ ] Column renderer (wall spans, floors, ceilings)
- [ ] Sprite renderer (thing sorting, clipping)
- [ ] Blit software framebuffer to Vulkan swapchain image

**Milestone**: render E1M1 from a fixed camera position.

---

### Phase 8 — `gz-playsim` Part 2: Actor system
**Analogues**: `src/playsim/p_mobj.cpp` (8.8K), `actor.h`

Design decision: **use an ECS** (recommended) or trait objects.

Recommended: add `hecs` as a dependency and model actors as entity + component bundles.

Key tasks:
- [ ] `Position`, `Velocity`, `Health`, `Sprite`, `Flags` components
- [ ] Spawn actors from THINGS lump data
- [ ] Think tick: velocity integration, gravity, floor/ceiling collision
- [ ] Basic state machine (actor state tables from DECORATE)
- [ ] Player movement + mouse look

**Milestone**: player can walk around E1M1 with collision.

---

### Phase 9 — `gz-audio`: Sound effects
**Analogues**: `src/common/audio/sound/`

Key tasks:
- [ ] `cpal` output stream initialization
- [ ] Load DMX sound lumps (Doom's raw PCM format)
- [ ] Channel pool: spawn/stop sounds, priority eviction
- [ ] Distance attenuation

**Milestone**: footsteps and door sounds play.

---

### Phase 10 — `gz-scripting` Part 1: ACS interpreter
**Analogues**: `src/playsim/p_acs.cpp` (11K lines)

ACS is simpler than ZScript — it's a stack-based bytecode with ~200 opcodes.
Port it before ZScript.

Key tasks:
- [ ] Parse BEHAVIOR lump (ACS bytecode header + script table)
- [ ] Bytecode interpreter loop (push/pop stack, branch, call)
- [ ] Script triggers (open scripts, line-cross, thing-enter)
- [ ] World/map/global variable arrays

**Milestone**: Hexen/Doom 2 ACS scripts execute (doors, lifts, scripted events).

---

### Phase 11 — `gz-scripting` Part 2: DECORATE parser
**Analogues**: `src/scripting/decorate/`, `src/gamedata/`

> **Upgrade Rust to >= 1.88 first**, then re-enable `chumsky` and `cranelift` in Cargo.toml.

Key tasks:
- [ ] DECORATE lexer (logos)
- [ ] DECORATE parser: actor definitions, state tables, property assignments
- [ ] Build actor class hierarchy (flat table, not C++ inheritance)
- [ ] State machine evaluation (A_* action function dispatch)
- [ ] Action function implementations (A_Chase, A_FirePistol, A_Pain, …)

**Milestone**: monsters from DECORATE spawn and behave (chase, shoot, die).

---

### Phase 12 — `gz-scripting` Part 3: ZScript compiler + VM
**Analogues**: `src/common/scripting/` — the largest and hardest subsystem

This is the hardest part of the entire port.  ZScript has:
- A full language (classes, inheritance, methods, closures, generics-lite)
- A bytecode VM with 100+ opcodes
- A JIT compiler (via Cranelift, replacing asmjit)
- Deep integration with the actor/object model

Key tasks:
- [ ] ZScript lexer (logos)
- [ ] ZScript parser → AST (chumsky)
- [ ] Type system and class table
- [ ] Semantic analysis / type checking
- [ ] Bytecode emission
- [ ] VM interpreter loop
- [ ] (Later) Cranelift JIT for hot paths

**Milestone**: ZScript classes from gzdoom.pk3 compile and execute.

---

### Phase 13 — `gz-audio` Part 2: Music
**Analogues**: `libraries/ZMusic/`

ZMusic is itself ~478K lines covering OPL synthesis, GUS patches, FluidSynth,
tracker modules (MOD/XM/IT/S3M), and more.  Options:

1. **Wrap the existing ZMusic library via FFI** — fastest path to working music
2. **Port incrementally** — start with OGG/MP3 (use `rodio`), then OPL, then MIDI

Key tasks:
- [ ] MUS lump format (Doom's MIDI subset) → standard MIDI
- [ ] OGG/MP3/FLAC via `rodio`/`symphonia`
- [ ] OPL2/3 synthesis (port nuked-opl3 or wrap via FFI)
- [ ] FluidSynth integration for MIDI

---

### Phase 14 — Game systems
Once all subsystems are working, port the higher-level game code:

- [ ] HUD / statusbar (`src/g_statusbar/`)
- [ ] Automap (`src/am_map.cpp`)
- [ ] Menus (`src/menu/`, `libraries/ZWidget/`)
- [ ] Intermission screens (`src/intermission/`)
- [ ] Save/load game (serializer)
- [ ] Networking (`src/d_net.cpp` — the 3.6K-line netcode)
- [ ] Console (`src/console/`)
- [ ] Demo recording/playback

---

## Hard problems in Rust

### 1. No class inheritance (the big one)
GZDoom has a deep actor class tree: `DObject → AActor → AInventory → AWeapon → …`

**Solution**: replace with an ECS or a flat component table.
- ECS (`hecs` or `bevy_ecs`): actors are entity IDs; properties are components.
- Class metadata lives in a separate `ClassDef` table keyed by `Name`.

### 2. Shared mutable state
GZDoom uses many globals (`level`, `players[]`, `StatusBar`, …).

**Solution**: thread-local or `Arc<Mutex<>>` where needed, or a top-level `GameState`
struct passed around explicitly.  The borrow checker will force good architecture here.

### 3. Virtual dispatch / ZScript reflection
ZScript uses C++ virtual functions and RTTI extensively.

**Solution**: use `dyn Trait` for polymorphism, `std::any::Any` for downcasting where
needed.  The VM call convention needs a Rust-native design.

### 4. `unsafe` budget
Avoid unsafe except for:
- FFI boundaries (ZMusic, Vulkan)
- Performance-critical inner loops (renderer column drawing)
Document every `unsafe` block with a safety comment.

---

## Toolchain note

Your current Rust: **1.85.0**

To re-enable the JIT and parser crates, upgrade to >= 1.88:
```bash
rustup update stable
```

Then uncomment in `Cargo.toml`:
- `cranelift-codegen`, `cranelift-frontend`, `cranelift-jit`, `cranelift-module`
- `chumsky`

---

## Suggested reading order (GZDoom source)

| You're working on | Read first |
|---|---|
| VFS | `src/common/filesystem/include/filesystem.h` |
| WAD parsing | `src/common/filesystem/source/file_wad.cpp` |
| Map loading | `src/maploader/maploader.cpp` |
| Actor system | `src/playsim/actor.h`, `p_mobj.cpp` |
| ACS | `src/playsim/p_acs.cpp` (read the header block at the top) |
| ZScript VM | `src/common/scripting/vm/vmexec.h` |
| Renderer | `src/rendering/hwrenderer/`, `libraries/ZVulkan/` |
