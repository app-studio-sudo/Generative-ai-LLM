// Small example frontend: manually construct a simple add() function in the IR.
use rosetta_ir::*;

fn main() {
    let mut m = Module::new("example");
    let f = Function {
        name: "add".to_string(),
        args: vec![Value{name: "a".to_string(), ty: Type::Int(64)}, Value{name: "b".to_string(), ty: Type::Int(64)}],
        ret: Type::Int(64),
        blocks: vec![BasicBlock { name: "entry".to_string(), instrs: vec![Instruction::Add { dest: "r0".to_string(), a: "a".to_string(), b: "b".to_string() }, Instruction::Ret { value: Some("r0".to_string()) } ] } ],
    };
    m.functions.insert(f.name.clone(), f);
    println!("IR:\n{{}}", serde_json::to_string_pretty(&m).unwrap());
}
