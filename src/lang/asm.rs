use std::{collections::HashMap, fmt};

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
    // THIS WONT WORK
    // needs the BYTE INDEX, not the VEC INDEX
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
                Instruction::Jz(x) => NoLabelInst::Jz(*label_map.get(x).unwrap() as u32),
                Instruction::Jnz(x) => NoLabelInst::Jnz(*label_map.get(x).unwrap() as u32),
                Instruction::Call(x) => NoLabelInst::Call(*label_map.get(x).unwrap() as u32),
                Instruction::Ret(x) => NoLabelInst::Ret(*x),
                Instruction::Pop => NoLabelInst::Pop,
                Instruction::Add => NoLabelInst::Add,
                Instruction::Sub => NoLabelInst::Sub,
                Instruction::Mul => NoLabelInst::Mul,
                Instruction::Div => NoLabelInst::Div,
                Instruction::Mod => NoLabelInst::Mod,
                Instruction::And => NoLabelInst::And,
                Instruction::Or => NoLabelInst::Or,
                Instruction::Not => NoLabelInst::Not,
                Instruction::Xor => NoLabelInst::Xor,
                Instruction::Eq => NoLabelInst::Eq,
                Instruction::L => NoLabelInst::L,
                Instruction::Le => NoLabelInst::Le,
                Instruction::G => NoLabelInst::G,
                Instruction::Ge => NoLabelInst::Ge,
                Instruction::Push(x, y) => NoLabelInst::Push(*x, *y),
                Instruction::Mov(x) => NoLabelInst::Mov(*x),
                Instruction::Fun(x) => NoLabelInst::Fun(*x),
                Instruction::Label(_) => unreachable!(),
            });
        }
        no_labels
    }
}
//bytetext asm has labels
pub enum Instruction {
    Ret(u16),
    Push(u8, u16),
    Pop,
    Mov(u16),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Xor,
    Eq,
    L,
    Le,
    G,
    Ge,
    Jmp(String),
    Jz(String),
    Jnz(String),
    Call(String),
    Fun(u16),
    Label(String),
}
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ans = String::new();
        let mut others = Vec::new();
        ans += match self {
            Instruction::Ret(x) => {
                others.push(x.to_string());
                "ret"
            }
            Instruction::Push(x, y) => {
                others.push(x.to_string());
                others.push(y.to_string());
                "push"
            }
            Instruction::Pop => "pop",
            Instruction::Mov(x) => {
                others.push(x.to_string());
                "mov"
            }
            Instruction::Add => "add",
            Instruction::Sub => "sub",
            Instruction::Mul => "mul",
            Instruction::Div => "div",
            Instruction::Mod => "mod",
            Instruction::And => "and",
            Instruction::Or => "or",
            Instruction::Not => "not",
            Instruction::Xor => "xor",
            Instruction::Eq => "eq",
            Instruction::L => "l",
            Instruction::Le => "le",
            Instruction::G => "g",
            Instruction::Ge => "ge",
            Instruction::Jmp(s) => {
                others.push(s.clone());
                "jmp"
            }
            Instruction::Jz(s) => {
                others.push(s.clone());
                "jz"
            }
            Instruction::Jnz(s) => {
                others.push(s.clone());
                "jnz"
            }
            Instruction::Call(s) => {
                others.push(s.clone());
                "call"
            }
            Instruction::Label(s) => {
                others.push(s.clone());
                "label"
            }
            Instruction::Fun(x) => {
                others.push(x.to_string());
                "fun"
            }
        };
        for thing in others {
            ans += " ";
            ans.push_str(&thing);
        }
        write!(f, "{}", ans)
    }
}

pub fn print_instructions(inst_vec: Vec<Instruction>) {
    for (i, inst) in inst_vec.iter().enumerate() {
        println!("{} -- {}", i, inst);
    }
}

pub enum NoLabelInst {
    Ret(u16),
    Push(u8, u16),
    Pop,
    Mov(u16),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Xor,
    Eq,
    L,
    Le,
    G,
    Ge,
    Jmp(u32),
    Jz(u32),
    Jnz(u32),
    Call(u32),
    Fun(u16),
}
