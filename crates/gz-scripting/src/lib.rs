/// gz-scripting — ZScript VM, ACS bytecode interpreter, DECORATE/ZScript compiler.
///
/// C++ analogues:
///   src/common/scripting/   (vm/, jit/, backend/, frontend/, core/)
///   src/scripting/          (zscript/, decorate/, thingdef*)
///   src/playsim/p_acs.cpp   (ACS bytecode — 11K lines)
///
/// This is the hardest subsystem to port.  ZScript has:
///   - A bytecode VM (src/common/scripting/vm/)
///   - A JIT compiler via asmjit (src/common/scripting/jit/)
///   - A full compiler pipeline: lexer → parser → AST → codegen (13K lines)
///   - A runtime type system (src/common/scripting/core/types.cpp, 3.7K lines)
///
/// Rust strategy:
///   JIT:    Cranelift (zero unsafe, pure Rust, same goal as asmjit)
///   Lexer:  logos (proc-macro based, very fast)
///   Parser: chumsky (combinator-based, good error recovery)

pub mod vm;
pub mod acs;
pub mod decorate;
