# MVP: Rosetta Backend (Rust)

Requirements
- Rust (stable toolchain) installed (https://rustup.rs)
- clang (for C/C++ frontend work, optional)

Build (local)
1. Checkout branch State-of-the-art-rosetta-backend-mvp.
2. From repo root run:

   cd mvp
   cargo build --workspace

Run tests (if present):
   cargo test --workspace

Development notes
- The workspace contains: ir (IR implementation), backend/llvm (LLVM lowering stub), frontend/rust (example frontend).
- Frontends for C++ and Python are documented as placeholders; they will integrate Clang/libclang or tree-sitter.

How you can help review
- Read design/architecture.md and the IR crate source.
- Suggest additional core IR features or ABI annotations needed for your workflows.
