//! LLVM backend stub for the Rosetta MVP.
//!
//! This is a placeholder crate to be expanded: it will lower the IR to LLVM IR using inkwell or other bindings.

pub fn backend_version() -> &'static str {
    "rosetta-backend-llvm v0.1.0 (stub)"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version() {
        assert!(backend_version().contains("stub"));
    }
}
