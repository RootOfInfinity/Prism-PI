use std::{collections::HashMap, fmt};

use super::tokens::Type;

const RET_NUM: u8 = 1;
const PUSH_NUM: u8 = 2;
const POP_NUM: u8 = 3;
const MOV_NUM: u8 = 4;
const ADD_NUM: u8 = 5;
const SUB_NUM: u8 = 6;
const MUL_NUM: u8 = 7;
const DIV_NUM: u8 = 8;
const MOD_NUM: u8 = 9;
const AND_NUM: u8 = 10;
const OR_NUM: u8 = 11;
const NOT_NUM: u8 = 12;
const XOR_NUM: u8 = 13;
const EQ_NUM: u8 = 14;
const L_NUM: u8 = 15;
const LE_NUM: u8 = 16;
const G_NUM: u8 = 17;
const GE_NUM: u8 = 18;
const JMP_NUM: u8 = 19;
const JZ_NUM: u8 = 20;
const JNZ_NUM: u8 = 21;
const CALL_NUM: u8 = 22;
const FUN_NUM: u8 = 23;

pub struct Assembler {
    consts: (Vec<u8>, Vec<Type>),
    pool: Vec<String>,
    code: Vec<Instruction>,
}
impl Assembler {
    pub fn new(consts: (Vec<u8>, Vec<Type>), pool: Vec<String>, code: Vec<Instruction>) -> Self {
        Assembler { consts, pool, code }
    }
    pub fn extract_consts_and_pool(&self) -> (&(Vec<u8>, Vec<Type>), &Vec<String>) {
        (&self.consts, &self.pool)
    }
    pub fn assemble(mut self) -> ((Vec<u8>, Vec<Type>), Vec<String>, Vec<u8>) {
        let no_labels = self.real_rm_labels();
        let mut bc: Vec<u8> = Vec::new();
        for inst in no_labels {
            match inst {
                NoLabelInst::Ret(x) => {
                    bc.push(RET_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
                NoLabelInst::Push(x, y) => {
                    bc.push(PUSH_NUM);
                    bc.push(x);
                    bc.extend_from_slice(&y.to_le_bytes());
                }
                NoLabelInst::Pop => bc.push(POP_NUM),
                NoLabelInst::Mov(x) => {
                    bc.push(MOV_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }

                NoLabelInst::Add => bc.push(ADD_NUM),
                NoLabelInst::Sub => bc.push(SUB_NUM),
                NoLabelInst::Mul => bc.push(MUL_NUM),
                NoLabelInst::Div => bc.push(DIV_NUM),
                NoLabelInst::Mod => bc.push(MOD_NUM),
                NoLabelInst::And => bc.push(AND_NUM),
                NoLabelInst::Or => bc.push(OR_NUM),
                NoLabelInst::Not => bc.push(NOT_NUM),
                NoLabelInst::Xor => bc.push(XOR_NUM),
                NoLabelInst::Eq => bc.push(EQ_NUM),
                NoLabelInst::L => bc.push(L_NUM),
                NoLabelInst::Le => bc.push(LE_NUM),
                NoLabelInst::G => bc.push(G_NUM),
                NoLabelInst::Ge => bc.push(GE_NUM),
                NoLabelInst::Jmp(x) => {
                    bc.push(JMP_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
                NoLabelInst::Jz(x) => {
                    bc.push(JZ_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
                NoLabelInst::Jnz(x) => {
                    bc.push(JNZ_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
                NoLabelInst::Call(x) => {
                    bc.push(CALL_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
                NoLabelInst::Fun(x) => {
                    bc.push(FUN_NUM);
                    bc.extend_from_slice(&x.to_le_bytes());
                }
            }
        }
        (self.consts, self.pool, bc)
    }
    fn real_rm_labels(&self) -> Vec<NoLabelInst> {
        let mut map: HashMap<String, u32> = HashMap::new();
        let mut cur_byte: u32 = 0;
        let mut out = Vec::new();
        for inst in self.code.iter() {
            if let Instruction::Label(s) = &inst {
                let s = s.to_owned();
                map.insert(s, cur_byte);
            }
            cur_byte += inst.size();
        }
        let mut code = Vec::new();
        for inst in self.code.iter() {
            if !matches!(inst, Instruction::Label(_)) {
                code.push(inst);
            }
        }
        for inst in code {
            out.push(
                (match inst {
                    Instruction::Ret(x) => NoLabelInst::Ret(*x),
                    Instruction::Push(x, y) => NoLabelInst::Push(*x, *y),
                    Instruction::Pop => NoLabelInst::Pop,
                    Instruction::Mov(x) => NoLabelInst::Mov(*x),
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
                    Instruction::Jmp(s) => NoLabelInst::Jmp(*map.get(s).expect("Invalid asm")),
                    Instruction::Jz(s) => NoLabelInst::Jz(*map.get(s).expect("Invalid asm")),
                    Instruction::Jnz(s) => NoLabelInst::Jnz(*map.get(s).expect("Invalid asm")),
                    Instruction::Call(s) => NoLabelInst::Call(*map.get(s).expect("Invalid asm")),
                    Instruction::Fun(x) => NoLabelInst::Fun(*x),
                    Instruction::Label(_) => unreachable!(),
                }),
            )
        }
        out
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

//bytetext asm has labels
#[derive(Debug)]
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
impl Instruction {
    fn size(&self) -> u32 {
        match self {
            Instruction::Ret(_) => 3,
            Instruction::Push(_, _) => 4,
            Instruction::Pop => 1,
            Instruction::Mov(_) => 3,
            Instruction::Add => 1,
            Instruction::Sub => 1,
            Instruction::Mul => 1,
            Instruction::Div => 1,
            Instruction::Mod => 1,
            Instruction::And => 1,
            Instruction::Or => 1,
            Instruction::Not => 1,
            Instruction::Xor => 1,
            Instruction::Eq => 1,
            Instruction::L => 1,
            Instruction::Le => 1,
            Instruction::G => 1,
            Instruction::Ge => 1,
            Instruction::Jmp(_) => 5,
            Instruction::Jz(_) => 5,
            Instruction::Jnz(_) => 5,
            Instruction::Call(_) => 5,
            Instruction::Fun(_) => 3,
            Instruction::Label(_) => 0,
        }
    }
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

pub fn print_instructions(inst_vec: &Vec<Instruction>) {
    println!("INSTRUCTIONS:");
    for (i, inst) in inst_vec.iter().enumerate() {
        println!("{} -- {}", i, inst);
    }
}
