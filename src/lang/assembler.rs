use super::asm::{Instruction, NoLabelInst};
//formatting will be fixed later
pub fn assemble(inst: &NoLabelInst) -> [u8; 6] {
    let mut inst_slice: [u8; 6] = [0; 6];
    match inst {
        &NoLabelInst::Ret => inst_slice[0] = 1u8,
        &NoLabelInst::Push(x1b, y2b) => {
            inst_slice[0] = 2u8;
            inst_slice[1] = x1b;
            let y = y2b.to_le_bytes();
            inst_slice[2] = y[0];
            inst_slice[3] = y[1];
        }
        &NoLabelInst::Pop => inst_slice[0] = 3u8,
        &NoLabelInst::Mov(x2b, y1b, z2b) => {
            inst_slice[0] = 4u8;
            let x = x2b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = y1b;
            let z = z2b.to_le_bytes();
            inst_slice[4] = z[0];
            inst_slice[5] = z[1];
        }
        &NoLabelInst::Alloc(x1b, y2b) => {
            inst_slice[0] = 5u8;
            inst_slice[1] = x1b;
            let y = y2b.to_le_bytes();
            inst_slice[2] = y[0];
            inst_slice[3] = y[1];
        }
        &NoLabelInst::Freet => inst_slice[0] = 6u8,
        &NoLabelInst::Add => inst_slice[0] = 7u8,
        &NoLabelInst::Sub => inst_slice[0] = 8u8,
        &NoLabelInst::Mul => inst_slice[0] = 9u8,
        &NoLabelInst::Div => inst_slice[0] = 10u8,
        &NoLabelInst::And => inst_slice[0] = 11u8,
        &NoLabelInst::Or => inst_slice[0] = 13u8,
        &NoLabelInst::Xor => inst_slice[0] = 14u8,
        &NoLabelInst::Cmp => inst_slice[0] = 15u8,
        &NoLabelInst::Jmp(x4b) => {
            inst_slice[0] = 16u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Je(x4b) => {
            inst_slice[0] = 17u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Jl(x4b) => {
            inst_slice[0] = 18u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Jle(x4b) => {
            inst_slice[0] = 19u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Jg(x4b) => {
            inst_slice[0] = 20u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Jge(x4b) => {
            inst_slice[0] = 21u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Call(x4b) => {
            inst_slice[0] = 22u8;
            let x = x4b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
            inst_slice[3] = x[2];
            inst_slice[4] = x[3];
        }
        &NoLabelInst::Fun(x2b) => {
            inst_slice[0] = 23u8;
            let x = x2b.to_le_bytes();
            inst_slice[1] = x[0];
            inst_slice[2] = x[1];
        }
    }
    inst_slice
}
