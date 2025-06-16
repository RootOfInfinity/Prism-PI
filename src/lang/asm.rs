use std::collections::HashMap;

use super::tokens::Type;

pub struct AsmFile {
    consts: (Vec<u8>, Vec<Type>),
    pool: Vec<String>,
    code: Vec<Instruction>,
}
impl AsmFile {
    pub fn new(consts: (Vec<u8>, Vec<Type>), pool: Vec<String>, code: Vec<Instruction>) -> Self {
        AsmFile { consts, pool, code }
    }
    pub fn extract_consts_and_pool(&self) -> (&(Vec<u8>, Vec<Type>), &Vec<String>) {
        (&self.consts, &self.pool)
    }
    pub fn remove_labels(&self) -> Vec<NoLabelInst> {
        let mut label_map: HashMap<&String, usize> = HashMap::new();
        let mut new_code: Vec<&Instruction> = Vec::new();
        let mut removed = 0;
        for (i, inst) in self.code.iter().enumerate() {
            let Instruction::Label(label) = inst else {
                new_code.push(inst);
                continue;
            };
            label_map.insert(label, i - removed);
            removed += 1;
        }
        let mut no_labels: Vec<NoLabelInst> = Vec::new();
        for inst in new_code {
            no_labels.push(match inst {
                Instruction::Jmp(x) => NoLabelInst::Jmp(*label_map.get(x).unwrap() as u32),
                Instruction::Je(x) => NoLabelInst::Je(*label_map.get(x).unwrap() as u32),
                Instruction::Jl(x) => NoLabelInst::Jl(*label_map.get(x).unwrap() as u32),
                Instruction::Jle(x) => NoLabelInst::Jle(*label_map.get(x).unwrap() as u32),
                Instruction::Jg(x) => NoLabelInst::Jg(*label_map.get(x).unwrap() as u32),
                Instruction::Jge(x) => NoLabelInst::Jge(*label_map.get(x).unwrap() as u32),
                Instruction::Call(x) => NoLabelInst::Call(*label_map.get(x).unwrap() as u32),
                Instruction::Ret => NoLabelInst::Ret,
                Instruction::Pop => NoLabelInst::Pop,
                Instruction::Freet => NoLabelInst::Freet,
                Instruction::Add => NoLabelInst::Add,
                Instruction::Sub => NoLabelInst::Sub,
                Instruction::Mul => NoLabelInst::Mul,
                Instruction::Div => NoLabelInst::Div,
                Instruction::And => NoLabelInst::And,
                Instruction::Or => NoLabelInst::Or,
                Instruction::Xor => NoLabelInst::Xor,
                Instruction::Cmp => NoLabelInst::Cmp,
                Instruction::Push(x, y) => NoLabelInst::Push(*x, *y),
                Instruction::Mov(x, y, z) => NoLabelInst::Mov(*x, *y, *z),
                Instruction::Alloc(x, y) => NoLabelInst::Alloc(*x, *y),
                Instruction::Fun(x) => NoLabelInst::Fun(*x),
                Instruction::Label(_) => unreachable!(),
            });
        }
        no_labels
    }
}
//bytetext asm has labels
pub enum Instruction {
    Ret,
    Push(u8, u16),
    Pop,
    Mov(u16, u8, u16),
    Alloc(u8, u16),
    Freet,
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Cmp,
    Jmp(String),
    Je(String),
    Jl(String),
    Jle(String),
    Jg(String),
    Jge(String),
    Call(String),
    Fun(u16),
    Label(String),
}

pub enum NoLabelInst {
    Ret,
    Push(u8, u16),
    Pop,
    Mov(u16, u8, u16),
    Alloc(u8, u16),
    Freet,
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Cmp,
    Jmp(u32),
    Je(u32),
    Jl(u32),
    Jle(u32),
    Jg(u32),
    Jge(u32),
    Call(u32),
    Fun(u16),
}
