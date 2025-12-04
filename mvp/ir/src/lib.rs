//! Minimal typed SSA-like IR skeleton for the Rosetta backend MVP.
//! This file is intentionally small — it's a starting point to expand later.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Int(u8),
    Float(u8),
    Ptr(Box<Type>),
    Struct(Vec<Type>),
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instruction {
    ConstInt { dest: String, value: i128, bits: u8 },
    Add { dest: String, a: String, b: String },
    Call { dest: Option<String>, func: String, args: Vec<String> },
    Ret { value: Option<String> },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicBlock {
    pub name: String,
    pub instrs: Vec<Instruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<Value>,
    pub ret: Type,
    pub blocks: Vec<BasicBlock>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Function>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module { name: name.to_string(), functions: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_simple_module() {
        let mut m = Module::new("test");
        let f = Function {
            name: "add".to_string(),
            args: vec![Value{name: "a".to_string(), ty: Type::Int(64)}, Value{name: "b".to_string(), ty: Type::Int(64)}],
            ret: Type::Int(64),
            blocks: vec![BasicBlock { name: "entry".to_string(), instrs: vec![Instruction::Add { dest: "r0".to_string(), a: "a".to_string(), b: "b".to_string() }, Instruction::Ret { value: Some("r0".to_string()) } ] } ],
        };
        m.functions.insert(f.name.clone(), f);
        let _ = serde_json::to_string_pretty(&m).unwrap();
    }
}
