const MEMORY_SIZE: u16 = 0xFFFF;
const PROGRAM_ROM_MEMORY_ADDRESS_START: u16 = 0x8000;
const RESET_INTERRUPT_ADDR: u16 = 0xFFFC;

/// See `studies/addressing.asm` for more info.
pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  Absolute,
  AbsoluteX, // Absolute, X
  AbsoluteY, // Absolute, Y
  IndirectX, // (Indirect, X)
  IndirectY, // (Indirect), Y
  NoneAddressing,
}

pub struct CPU {
  pub program_counter: u16,
  pub status: u8,
  pub accumulator: u8,
  pub stack_pointer: u8,
  pub register_x: u8,
  pub register_y: u8,
  memory: [u8; MEMORY_SIZE as usize],
}

impl Default for CPU {
  fn default() -> Self {
    let memory: [u8; MEMORY_SIZE as usize] = [0; MEMORY_SIZE as usize];
    Self {
      program_counter: 0,
      status: 0,
      accumulator: 0,
      stack_pointer: 0,
      register_x: 0,
      register_y: 0,
      memory,
    }
  }
}

impl CPU {
  pub fn new() -> Self {
    Self::default()
  }

  fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }

  fn mem_read_u16(&self, addr: u16) -> u16 {
    let lsb = self.mem_read(addr) as u16;
    let hsb = self.mem_read(addr + 1) as u16;

    (hsb << 8) | (lsb as u16)
  }

  fn mem_write_u16(&mut self, addr: u16, data: u16) {
    let lsb = (data & 0xFF) as u8;
    let hsb = (data >> 8) as u8;

    self.mem_write(addr, lsb);
    self.mem_write(addr + 1, hsb);
  }

  fn reset(&mut self) {
    self.stack_pointer = 0;
    self.accumulator = 0;
    self.register_x = 0;
    self.register_y = 0;
    self.status = 0b0000_0000;
    self.program_counter = self.mem_read_u16(RESET_INTERRUPT_ADDR);
  }

  fn load(&mut self, program: Vec<u8>) {
    self.memory[PROGRAM_ROM_MEMORY_ADDRESS_START as usize
      ..(PROGRAM_ROM_MEMORY_ADDRESS_START as usize + program.len())]
      .copy_from_slice(&program[..]); // puts the program into memory

    self.mem_write_u16(RESET_INTERRUPT_ADDR, PROGRAM_ROM_MEMORY_ADDRESS_START);
  }

  fn update_negative_and_zero_flags(&mut self, result: u8) {
    // Set Status Flags
    if result == 0 {
      self.status |= 0b0000_0010; // set zero flag to 1
    } else {
      self.status &= 0b1111_1101; // set zero flag to 0
    }

    if result & 0b1000_0000 != 0 {
      self.status |= 0b1000_0000; // set negative flag to 1
    } else {
      self.status &= 0b0111_1111; // set negative flag to 0
    }
  }

  fn get_operand_addr(&self, mode: AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,
      AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
      _ => todo!(),
    }
  }

  fn lda(&mut self, addressing_mode: AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    let param = self.mem_read(operand_addr);

    self.accumulator = param;
    self.update_negative_and_zero_flags(self.accumulator);
  }

  fn run(&mut self) {
    loop {
      let op_code = self.mem_read(self.program_counter);
      self.program_counter += 1;

      match op_code {
        0xA9 => {
          self.lda(AddressingMode::Immediate);
          self.program_counter += 1;
        }
        0xA5 => {
          self.lda(AddressingMode::ZeroPage);
          self.program_counter += 1;
        }
        0xAA => {
          // TAX
          self.register_x = self.accumulator.clone();
          self.update_negative_and_zero_flags(self.register_x);
        }
        0xE8 => {
          // INX
          self.register_x = self.register_x.wrapping_add(1);
          self.update_negative_and_zero_flags(self.register_x);
        }
        0x00 => {
          // BRK
          return;
        }
        _ => todo!(),
      }
    }
  }

  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run();
  }
}
