#[derive(Default)]
pub struct CPU {
  pub program_counter: u16,
  pub status: u8,
  pub accumulator: u8,
  pub stack_pointer: u8,
  pub register_x: u8,
  pub register_y: u8
}

impl CPU {
  pub fn new() -> Self {
    Self {
      program_counter: 0,
      status: 0,
      accumulator: 0,
      stack_pointer: 0,
      register_x: 0,
      register_y: 0
    }
  }

  pub fn interpret(&mut self, program: Vec<u8>) {
    self.program_counter = 0;

    let op_code = program[self.program_counter as usize];
    println!("{}", op_code);
  }
}