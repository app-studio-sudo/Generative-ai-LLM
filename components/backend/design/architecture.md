# Architecture: State-of-the-art Rosetta Backend (MVP)

Purpose
-------
Provide a modular backend and translation layer (a 'Rosetta Stone') that maps constructs from multiple source languages to a single, typed SSA-based intermediate representation (IR), applies optimizations, and emits efficient code for multiple targets (macOS x86_64/arm64, Linux, and WASM). This MVP is a scaffold to demonstrate the core components and to iterate rapidly.

Core components (MVP scope)
---------------------------
- Universal IR (typed, SSA-like, serializable) implemented in Rust (mvp/ir).
- Minimal frontends (placeholders and docs) for C++ (via clang), Rust, and Python: parse or lower small examples to the IR (mvp/frontend/*).
- Backend lowering to LLVM using Rust bindings (mvp/backend/llvm).
- Rosetta mapping data model sketches + examples in the design doc.
- CI to build the Rust crates on macOS and Linux.

Technology choices
------------------
- Implementation language: Rust for safety and performance.
- Codegen: LLVM (inkwell) for initial CPU codegen; consider MLIR later.
- Parsing frontends: clang/LLVM for C/C++; rustc or tree-sitter for Rust; CPython AST or tree-sitter for Python.
- Storage: simple file-format (JSON) or SQLite for mapping data.

IR design (minimal sketch)
-------------------------
- Module -> Functions -> BasicBlocks -> Instructions (SSA values)
- Types: Int(width), Float(width), Ptr(T), Struct, Array, FuncType
- Metadata annotations: source language, source location, ABI tags
- Serialization: serde + JSON for MVP

MVP plan (6 sprints)
--------------------
1. Spec + repo scaffold (this commit).
2. Implement a minimal IR crate with data types and serializers (mvp/ir).
3. Create a tiny Rust frontend that lowers a simple subset to IR (mvp/frontend/rust).
4. Implement LLVM lowering for a small function and emit object code (mvp/backend/llvm).
5. Add Rosetta mapping examples for a handful of constructs and test cases.
6. Iterate: add C++/Python lowering, optimizations, and WASM target.

Next steps
----------
- Review this scaffold.
- I can add the initial IR implementation and a small example translation (Rust function -> IR -> LLVM).
