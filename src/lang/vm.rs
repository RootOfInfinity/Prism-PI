use super::tokens::Type;

pub struct VM {
  ip: usize,
  flag: i32,
  consts: (Vec<u8>, Vec<Type>),
  inst: Vec<u8>,
  vars: (Vec<u8>, Vec<Type>),
  comp_frames: Vec<(Vec<u8>, Vec<Type>)>,
  pool: Vec<String>,
}
impl VM {
  pub fn new(pool: Vec<String>, consts: (Vec<u8>, Vec<Type>), inst: Vec<u8>) -> Self {
    VM { 
      ip: 0, 
      flag: 0, 
      consts, 
      inst, 
      vars: (Vec::new(), Vec::new()), 
      comp_frames: vec![(Vec::new(), Vec::new())], 
      pool,
    }
  }
  pub fn execute_order_66(&mut self) -> i32 {
    loop {
      if let ProgState::Halt(x) = self.eval_inst() {
        break x;
      }
    }
  }
  fn eval_inst(&mut self) -> ProgState {
    let ip = self.ip;
    let len = /* Bytecode::get_size(self.inst[ip]) This will be added as soon as I make bytecode struct*/ 1;
    self.ip += len;
    todo!()
  }  
}
enum ProgState {
  Halt(i32),
  Running,
}
