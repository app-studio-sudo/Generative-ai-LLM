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
MIT License

Copyright (c) 2025 app-studio-sudo

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
