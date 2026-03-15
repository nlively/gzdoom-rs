// DECORATE and ZScript parser front-end.
//
// DECORATE is the older, simpler actor-definition language.  ZScript is its
// superset, adding full class definitions, methods, and expressions.
//
// Compiler pipeline:
//   source text -> logos lexer -> chumsky parser -> AST
//   -> type checker -> gz-scripting bytecode emitter -> (optional) Cranelift JIT

// Stub.
