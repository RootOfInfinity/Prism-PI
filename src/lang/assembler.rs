use super::asm::Instruction;
//formatting will be fixed later
pub fn to_le_bytes(inst: &Instruction) -> [u8; 6] {
      let mut inst_slice: [u8; 6] = [0; 6];
      match inst {
          &Instruction::Ret => inst_slice[0] = 1u8,
          &Instruction::Push(x1b, y2b) => {
              inst_slice[0] = 2u8;
              inst_slice[1] = x1b;
              let y = y2b.to_le_bytes();
              inst_slice[2] = y[0];
              inst_slice[3] = y[1];
          }
          &Instruction::Pop => inst_slice[0] = 3u8,
          &Instruction::Mov(x2b, y1b, z2b) => {
              inst_slice[0] = 4u8;
              let x = x2b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = y1b;
              let z = z2b.to_le_bytes();
              inst_slice[4] = z[0];
              inst_slice[5] = z[1];
          }
          &Instruction::Alloc(x1b, y2b) => {
              inst_slice[0] = 5u8;
              let y = y2b.to_le_bytes();
              inst_slice[1] = y[0];
              inst_slice[2] = y[1];
          }
          &Instruction::Freet => inst_slice[0] = 6u8,
          &Instruction::Add => inst_slice[0] = 7u8,
          &Instruction::Sub => inst_slice[0] = 8u8,
          &Instruction::Mul => inst_slice[0] = 9u8,
          &Instruction::Div => inst_slice[0] = 10u8,
          &Instruction::And => inst_slice[0] = 11u8,
          &Instruction::Or => inst_slice[0] = 13u8,
          &Instruction::Xor => inst_slice[0] = 14u8,
          &Instruction::Cmp => inst_slice[0] = 15u8,
          &Instruction::Jmp(x4b) => {
              inst_slice[0] = 16u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Je(x4b) => {
              inst_slice[0] = 17u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Jl(x4b) => {
              inst_slice[0] = 18u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Jle(x4b) => {
              inst_slice[0] = 19u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Jg(x4b) => {
              inst_slice[0] = 20u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Jge(x4b) => {
              inst_slice[0] = 21u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Call(x4b) => {
              inst_slice[0] = 22u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
          &Instruction::Fun(x4b) => {
              inst_slice[0] = 23u8;
              let x = x4b.to_le_bytes();
              inst_slice[1] = x[0];
              inst_slice[2] = x[1];
              inst_slice[3] = x[2];
              inst_slice[4] = x[3];
          }
      }
      inst_slice
}
