use super::{tokens::Type, wrapped_val::WrappedVal};

struct Array {
    data_type: Type,
    data: Vec<u8>,
}

impl Array {
    pub fn index(&self, index: i32) -> WrappedVal {
        let size = self.data_type.size() - 1;
        let byte_ind = index as usize * size;
        if byte_ind + size - 1 > self.data.len() {
            panic!("Array out of bounds");
        }
        match self.data_type {
            Type::Int => {
                let int =
                    i32::from_le_bytes(self.data[byte_ind..byte_ind + size].try_into().unwrap());
                WrappedVal::Int(int)
            }
            Type::Dcml => {
                let dcml =
                    f64::from_le_bytes(self.data[byte_ind..byte_ind + size].try_into().unwrap());
                WrappedVal::Dcml(dcml)
            }
            Type::Bool => {
                let boolean = self.data[byte_ind] != 0;
                WrappedVal::Bool(boolean)
            }
            Type::String => {
                let string_num =
                    u16::from_le_bytes(self.data[byte_ind..byte_ind + size].try_into().unwrap());
                WrappedVal::String(string_num)
            }
            _ => panic!(),
        }
    }
    pub fn push_wrap(&mut self, val: WrappedVal) {
        if self.data_type != val.type_enum() {
            panic!("Not right data type");
        }
        match val {
            WrappedVal::Int(int) => {
                self.data.extend_from_slice(&int.to_le_bytes());
            }
            WrappedVal::Dcml(dcml) => {
                self.data.extend_from_slice(&dcml.to_le_bytes());
            }
            WrappedVal::Bool(boolean) => {
                self.data.push(boolean as u8);
            }
            WrappedVal::String(string_num) => {
                self.data.extend_from_slice(&string_num.to_le_bytes());
            }
            _ => panic!(),
        }
    }
    pub fn pop(&mut self) {
        let size = self.data_type.size() - 1;
        for _ in 0..size {
            self.data.pop();
        }
    }
}
