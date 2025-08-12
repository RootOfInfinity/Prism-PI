use core::panic;
use std::any::TypeId;

use super::array::Array;
use super::tokens::Type;
use super::wrapped_val::WrappedVal;

// Constant identifiers for instructions
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
const CAST_NUM: u8 = 24;
const ARRLEN_NUM: u8 = 25;
const ARRPOP_NUM: u8 = 26;
const ARRPUSH_NUM: u8 = 27;
const FREEARR_NUM: u8 = 28;
const ARRIND_NUM: u8 = 29;

// Constant identifiers for types
const INT_NUM: u8 = 1;
const DCML_NUM: u8 = 2;
const BOOL_NUM: u8 = 3;
const STRING_NUM: u8 = 4;
const CALLSTACK_NUM: u8 = 5;
const ARRAY_NUM: u8 = 6;

pub struct VM {
    ip: usize,
    consts: Vec<u8>,
    inst: Vec<u8>,
    stack: Vec<u8>,
    pool: Vec<String>,
    array_stack: Vec<Array>,
}
impl VM {
    pub fn new(pool: Vec<String>, consts: Vec<u8>, inst: Vec<u8>) -> Self {
        VM {
            ip: 0,
            consts,
            inst,
            pool,
            stack: Vec::new(),
            array_stack: Vec::new(),
        }
    }
    pub fn execute_order_66(&mut self) -> i32 {
        loop {
            if let ProgState::Halt(x) = self.eval_inst() {
                break x;
            }
            // println!("Whole Stack: {:#?}", self.get_entire_stack_wrapped());
            // println!("New IP: {}", self.ip);
        }
    }
    fn eval_inst(&mut self) -> ProgState {
        let ip = self.ip;
        let len = get_inst_size(self.inst[ip]);
        self.ip += len;
        let st = ip + 1; // easy shorthand to be the start of the data of the inst
        match self.inst[ip] {
            RET_NUM => {
                let how_much_bytes_to_pop = u16::from_le_bytes(
                    self.inst[(st)..(st + size_of::<u16>())].try_into().unwrap(),
                );
                let ret_val = self.pop_stack_top_wrapped();
                // println!("Function returned: {:#?}", ret_val);
                for _ in 0..(how_much_bytes_to_pop - ret_val.type_enum().size() as u16) {
                    self.stack.pop();
                }
                // after this, there should be a number on the stack
                // that will point to where the function was called from.
                // It will have a unique identifier, and if that identifier is not
                // found, then we are in main, and return with the top thing on the stack.
                // println!(
                //     "RIGHT BEFORE RETURN WHOLE STACK! {:#?}",
                //     self.get_entire_stack_wrapped()
                // );
                if self.stack.len() > 0 && self.stack[self.stack.len() - 1] == CALLSTACK_NUM {
                    self.ip = u32::from_le_bytes(
                        self.stack
                            [(self.stack.len() - 1 - size_of::<u32>())..(self.stack.len() - 1)]
                            .try_into()
                            .unwrap(),
                    ) as usize;
                    for _ in 0..(size_of::<u32>() + 1) {
                        self.stack.pop();
                    }
                    self.push_wrapped(ret_val);
                } else {
                    let WrappedVal::Int(ret_val) = ret_val else {
                        unreachable!();
                    };
                    return ProgState::Halt(ret_val);
                }
            }
            PUSH_NUM => {
                const PUSH_FROM_STACK: u8 = 0;
                const PUSH_FROM_CONSTS: u8 = 1;
                match self.inst[st] {
                    PUSH_FROM_STACK => {
                        let offset = u16::from_le_bytes(
                            self.inst[(st + 1)..(st + 1 + size_of::<u16>())]
                                .try_into()
                                .unwrap(),
                        );
                        let wrapped_val = self.wrap_stack_val(offset);
                        self.push_wrapped(wrapped_val);
                    }
                    PUSH_FROM_CONSTS => {
                        let index = u16::from_le_bytes(
                            self.inst[(st + 1)..(st + 1 + size_of::<u16>())]
                                .try_into()
                                .unwrap(),
                        );
                        let wrapped_val = self.get_const_wrapped(index);
                        self.push_wrapped(wrapped_val);
                    }
                    _ => unreachable!(),
                }
            }
            POP_NUM => {
                let size = match self.stack[self.stack.len() - 1] {
                    INT_NUM => 4,
                    DCML_NUM => 8,
                    BOOL_NUM => 1,
                    STRING_NUM => 2,
                    CALLSTACK_NUM => 4,
                    _ => unreachable!(),
                } + 1;
                for _ in 0..size {
                    self.stack.pop();
                }
            }
            MOV_NUM => {
                let offset = u16::from_le_bytes(
                    self.inst[(st)..(st + size_of::<u16>())].try_into().unwrap(),
                );
                self.mutate_var(offset);
            }
            ADD_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left + right;
                self.push_wrapped(ans);
            }
            SUB_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left - right;
                self.push_wrapped(ans);
            }
            MUL_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left * right;
                self.push_wrapped(ans);
            }
            DIV_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left / right;
                self.push_wrapped(ans);
            }
            MOD_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left % right;
                self.push_wrapped(ans);
            }
            AND_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left & right;
                self.push_wrapped(ans);
            }
            OR_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left | right;
                self.push_wrapped(ans);
            }
            XOR_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left ^ right;
                self.push_wrapped(ans);
            }
            EQ_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left == right;
                self.push_wrapped(WrappedVal::Bool(ans));
            }
            L_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left < right;
                self.push_wrapped(WrappedVal::Bool(ans));
            }
            LE_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left <= right;
                self.push_wrapped(WrappedVal::Bool(ans));
            }
            G_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left > right;
                self.push_wrapped(WrappedVal::Bool(ans));
            }
            GE_NUM => {
                let right = self.pop_stack_top_wrapped();
                let left = self.pop_stack_top_wrapped();
                let ans = left >= right;
                self.push_wrapped(WrappedVal::Bool(ans));
            }
            // FINALLY DONE WITH OPS!!!
            JMP_NUM => {
                let jump_to =
                    u32::from_le_bytes(self.inst[st..(st + size_of::<u32>())].try_into().unwrap());
                self.ip = jump_to as usize;
            }
            JZ_NUM => {
                if let WrappedVal::Bool(true) = self.pop_stack_top_wrapped() {
                    let jump_to = u32::from_le_bytes(
                        self.inst[st..(st + size_of::<u32>())].try_into().unwrap(),
                    );
                    self.ip = jump_to as usize;
                }
            }
            JNZ_NUM => {
                if let WrappedVal::Bool(false) = self.pop_stack_top_wrapped() {
                    let jump_to = u32::from_le_bytes(
                        self.inst[st..(st + size_of::<u32>())].try_into().unwrap(),
                    );
                    self.ip = jump_to as usize;
                }
            }
            CALL_NUM => {
                let callstack = self.ip as u32;
                self.stack.extend_from_slice(&callstack.to_le_bytes());
                self.stack.push(CALLSTACK_NUM);
                let jump_to =
                    u32::from_le_bytes(self.inst[st..(st + size_of::<u32>())].try_into().unwrap());
                self.ip = jump_to as usize;
            }
            FUN_NUM => {
                let is_main = self.stack.len() == 0;

                if !is_main {
                    let callstack = self.pop_stack_top_wrapped();
                    let mut argvec = Vec::new();
                    let arg_amount = u16::from_le_bytes(
                        self.inst[st..(st + size_of::<u16>())].try_into().unwrap(),
                    );
                    for _ in 0..arg_amount {
                        argvec.push(self.pop_stack_top_wrapped());
                    }
                    self.push_wrapped(callstack);
                    for wrap in argvec.into_iter().rev() {
                        self.push_wrapped(wrap);
                    }
                }
            }
            CAST_NUM => {
                let to_type = self.inst[st];
                let val = self.pop_stack_top_wrapped();
                let new_val = match val {
                    WrappedVal::Int(int) => {
                        if to_type == DCML_NUM {
                            WrappedVal::Dcml(int as f64)
                        } else {
                            unreachable!()
                        }
                    }
                    WrappedVal::Dcml(dcml) => {
                        if to_type == INT_NUM {
                            WrappedVal::Int(dcml as i32)
                        } else {
                            unreachable!()
                        }
                    }
                    WrappedVal::Bool(boolean) => {
                        if to_type == INT_NUM {
                            WrappedVal::Int(boolean as i32)
                        } else {
                            unreachable!()
                        }
                    }
                    _ => unreachable!(),
                };
                self.push_wrapped(new_val);
            }
            ARRLEN_NUM => {
                let arr = self.pop_stack_top_wrapped();
                let WrappedVal::Array(arr_ind) = arr else {
                    panic!()
                };
                let arraylen = self.array_stack[arr_ind as usize].length();
                self.push_wrapped(WrappedVal::Int(arraylen));
            }
            ARRPOP_NUM => {
                let arr = self.wrap_stack_val(0);
                let WrappedVal::Array(arr_ind) = arr else {
                    panic!()
                };
                let array = &mut self.array_stack[arr_ind as usize];
                array.pop();
            }
            ARRPUSH_NUM => {
                let var = self.pop_stack_top_wrapped();
                let arr = self.wrap_stack_val(0);
                let WrappedVal::Array(arr_ind) = arr else {
                    panic!()
                };
                let array = &mut self.array_stack[arr_ind as usize];
                array.push_wrap(var);
            }
            ARRIND_NUM => {
                let WrappedVal::Int(index) = self.pop_stack_top_wrapped() else {
                    panic!()
                };
                let WrappedVal::Array(arr_ind) = self.pop_stack_top_wrapped() else {
                    panic!()
                };
                let array = &mut self.array_stack[arr_ind as usize];
                let wrap_val = array.index(index);
                self.push_wrapped(wrap_val);
            }
            FREEARR_NUM => {
                self.array_stack.pop();
            }
            _ => unreachable!(),
        }
        return ProgState::Running;
    }
    fn mutate_var(&mut self, offset_from_top: u16) {
        let var_ptr = self.stack.len() - 1 - offset_from_top as usize;
        let var_type = self.stack[var_ptr];
        let new_ptr = self.stack.len() - 1;
        let new_type = self.stack[new_ptr];
        if var_type != new_type {
            panic!();
        }
        match var_type {
            INT_NUM => {
                let new_slice = &self.stack[new_ptr - 1 - size_of::<i32>()..new_ptr - 1];
                for i in 0..size_of::<i32>() {
                    self.stack[var_ptr - 1 - size_of::<i32>() + i] =
                        self.stack[new_ptr - 1 - size_of::<i32>() + i];
                }
            }
            DCML_NUM => {
                let new_slice = &self.stack[new_ptr - 1 - size_of::<f64>()..new_ptr - 1];
                for i in 0..size_of::<f64>() {
                    self.stack[var_ptr - 1 - size_of::<f64>() + i] =
                        self.stack[new_ptr - 1 - size_of::<f64>() + i];
                }
            }
            BOOL_NUM => {
                let val = self.stack[new_ptr - 2];
                self.stack[var_ptr - 2] = val;
            }
            STRING_NUM => {
                let new_slice = &self.stack[new_ptr - 1 - size_of::<u16>()..new_ptr - 1];
                for i in 0..size_of::<u16>() {
                    self.stack[var_ptr - 1 - size_of::<u16>() + i] =
                        self.stack[new_ptr - 1 - size_of::<u16>() + i];
                }
            }
            _ => unreachable!(),
        }
    }
    fn get_const_wrapped(&self, byte_index_of_const: u16) -> WrappedVal {
        let datatype = self.consts[byte_index_of_const as usize];
        let data = &self.consts[(byte_index_of_const as usize + 1)
            ..(byte_index_of_const as usize + get_type_size(datatype))];
        match datatype {
            INT_NUM => {
                let int = i32::from_le_bytes(data.try_into().unwrap());
                WrappedVal::Int(int)
            }
            DCML_NUM => {
                let dcml = f64::from_le_bytes(data.try_into().unwrap());
                WrappedVal::Dcml(dcml)
            }
            BOOL_NUM => {
                let boolean = data[0] != 0;
                WrappedVal::Bool(boolean)
            }
            STRING_NUM => {
                let string_ind = u16::from_le_bytes(data.try_into().unwrap());
                WrappedVal::String(string_ind)
            }
            _ => unreachable!(),
        }
    }
    fn pop_stack_top_wrapped(&mut self) -> WrappedVal {
        let ans = self.wrap_stack_val(0);
        for _ in 0..get_type_size(self.stack[self.stack.len() - 1]) {
            self.stack.pop();
        }
        ans
    }
    fn wrap_stack_val(&self, offset_from_top: u16) -> WrappedVal {
        let off = offset_from_top as usize + 1;
        match self.stack[self.stack.len() - off] {
            INT_NUM => {
                let num = i32::from_le_bytes(
                    self.stack[self.stack.len() - off - size_of::<i32>()..self.stack.len() - off]
                        .try_into()
                        .unwrap(),
                );
                WrappedVal::Int(num)
            }
            DCML_NUM => {
                let float = f64::from_le_bytes(
                    self.stack[self.stack.len() - off - size_of::<f64>()..self.stack.len() - off]
                        .try_into()
                        .unwrap(),
                );
                WrappedVal::Dcml(float)
            }
            BOOL_NUM => {
                let boolean = self.stack[self.stack.len() - off - 1] != 0;
                WrappedVal::Bool(boolean)
            }
            STRING_NUM => {
                let string_ind = u16::from_le_bytes(
                    self.stack[self.stack.len() - off - size_of::<u16>()..self.stack.len() - off]
                        .try_into()
                        .unwrap(),
                );
                WrappedVal::String(string_ind)
            }
            CALLSTACK_NUM => {
                let new_ip = u32::from_le_bytes(
                    self.stack[self.stack.len() - off - size_of::<u32>()..self.stack.len() - off]
                        .try_into()
                        .unwrap(),
                );
                WrappedVal::CallStack(new_ip)
            }
            ARRAY_NUM => {
                let array_ind = u16::from_le_bytes(
                    self.stack[self.stack.len() - off - size_of::<u16>()..self.stack.len() - off]
                        .try_into()
                        .unwrap(),
                );
                WrappedVal::Array(array_ind)
            }
            _ => unreachable!(),
        }
    }
    fn push_wrapped(&mut self, wrap_val: WrappedVal) {
        match wrap_val {
            WrappedVal::Int(int) => {
                self.stack.extend_from_slice(&int.to_le_bytes());
                self.stack.push(INT_NUM);
            }
            WrappedVal::Dcml(dcml) => {
                self.stack.extend_from_slice(&dcml.to_le_bytes());
                self.stack.push(DCML_NUM);
            }
            WrappedVal::Bool(boolean) => {
                self.stack.push(boolean as u8);
                self.stack.push(BOOL_NUM);
            }
            WrappedVal::String(string_ind) => {
                self.stack.extend_from_slice(&string_ind.to_le_bytes());
                self.stack.push(STRING_NUM);
            }
            WrappedVal::CallStack(new_ip) => {
                self.stack.extend_from_slice(&new_ip.to_le_bytes());
                self.stack.push(CALLSTACK_NUM);
            }
            WrappedVal::Array(array_ind) => {
                self.stack.extend_from_slice(&array_ind.to_le_bytes());
                self.stack.push(ARRAY_NUM);
            }
        }
    }
    // costly
    fn get_entire_stack_wrapped(&mut self) -> Vec<WrappedVal> {
        let mut byte_off = 0;
        let mut vals = Vec::new();
        let len = self.stack.len();
        loop {
            if byte_off >= len {
                break;
            }
            let datatype = self.stack[len - byte_off - 1];
            let typesize = get_type_size(datatype);
            let wrap_val = self.wrap_stack_val(byte_off as u16);
            byte_off += typesize;
            vals.push(wrap_val);
        }
        vals.reverse();
        vals
    }
}

enum ProgState {
    Halt(i32),
    Running,
}

fn get_inst_size(instruction_num: u8) -> usize {
    match instruction_num {
        RET_NUM => 3,
        PUSH_NUM => 4,
        POP_NUM => 1,
        MOV_NUM => 3,
        ADD_NUM => 1,
        SUB_NUM => 1,
        MUL_NUM => 1,
        DIV_NUM => 1,
        MOD_NUM => 1,
        AND_NUM => 1,
        OR_NUM => 1,
        NOT_NUM => 1,
        XOR_NUM => 1,
        EQ_NUM => 1,
        L_NUM => 1,
        LE_NUM => 1,
        G_NUM => 1,
        GE_NUM => 1,
        JMP_NUM => 5,
        JZ_NUM => 5,
        JNZ_NUM => 5,
        CALL_NUM => 5,
        FUN_NUM => 3,
        _ => unreachable!(),
    }
}

pub fn get_type_size(type_num: u8) -> usize {
    (match type_num {
        INT_NUM => size_of::<i32>(),
        DCML_NUM => size_of::<f64>(),
        BOOL_NUM => size_of::<bool>(),
        STRING_NUM => size_of::<u16>(),
        CALLSTACK_NUM => size_of::<u32>(),
        _ => {
            // println!("Number: {}", type_num);
            unreachable!();
        }
    }) + 1
}
