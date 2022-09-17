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

  fn update_negative_and_zero_flags(&mut self) {
    // Set Status Flags
    if self.accumulator == 0 {
      self.status = self.status | 0b0000_0010; // set zero flag to 1
    } else {
      self.status = self.status & 0b1111_1101; // set zero flag to 0
    }

    if self.accumulator & 0b1000_0000 != 0 {
      self.status = self.status | 0b1000_0000; // set negative flag to 1
    } else {
      self.status = self.status & 0b0111_1111; // set negative flag to 0
    }
  }

  pub fn interpret(&mut self, program: Vec<u8>) {
    self.program_counter = 0;

    loop {
      let op_code = program[self.program_counter as usize];
      self.program_counter += 1;
  
      match op_code {
        0xA9 => { // LDA
          let param = program[self.program_counter as usize];
          self.program_counter += 1;

          self.accumulator = param;

          self.update_negative_and_zero_flags();

        },
        0xAA => { // TAX
          self.register_x = self.accumulator.clone();

          self.update_negative_and_zero_flags();

        },
        0x00 => { // BRK
          return;
        },
        _ => todo!()
      }
    }
  }
}