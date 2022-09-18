use core::panic;
use std::collections::HashMap;

use crate::opcodes;

const MEMORY_SIZE: u16 = 0xFFFF;
const PROGRAM_ROM_MEMORY_ADDRESS_START: u16 = 0x8000;
const RESET_INTERRUPT_ADDR: u16 = 0xFFFC;

/// See `studies/addressing.asm` for more info.
#[derive(Debug, PartialEq, Eq)]
pub enum AddressingMode {
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
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

  pub fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }

  pub fn mem_read_u16(&self, addr: u16) -> u16 {
    let lsb = self.mem_read(addr) as u16;
    let hsb = self.mem_read(addr + 1) as u16;

    (hsb << 8) | (lsb as u16)
  }

  pub fn mem_write_u16(&mut self, addr: u16, data: u16) {
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

  fn get_indirect_lookup(&self, lookup_addr: u16) -> u16 {
    let lsb = self.mem_read(lookup_addr);
    let hsb = self.mem_read(lookup_addr.wrapping_add(1));

    (hsb as u16) << 8 | (lsb as u16)
  }

  fn get_operand_addr(&self, mode: &AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,
      AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
      AddressingMode::ZeroPageX => {
        let pos = self.mem_read(self.program_counter);
        pos.wrapping_add(self.register_x) as u16
      }
      AddressingMode::ZeroPageY => {
        let pos = self.mem_read(self.program_counter);
        pos.wrapping_add(self.register_y) as u16
      }
      AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
      AddressingMode::AbsoluteX => {
        let pos = self.mem_read_u16(self.program_counter);
        pos.wrapping_add(self.register_x as u16) as u16
      }
      AddressingMode::AbsoluteY => {
        let pos = self.mem_read_u16(self.program_counter);
        pos.wrapping_add(self.register_y as u16) as u16
      }
      AddressingMode::IndirectX => {
        let base = self.mem_read(self.program_counter);
        let lookup_addr = base.wrapping_add(self.register_x);

        // Indirect lookup
        self.get_indirect_lookup(lookup_addr as u16)
      }
      AddressingMode::IndirectY => {
        let lookup_addr = self.mem_read(self.program_counter);

        // Indirect Lookup
        let addr = self.get_indirect_lookup(lookup_addr as u16);

        addr.wrapping_add(self.register_y as u16)
      }
      AddressingMode::NoneAddressing => panic!("Mode not known"),
    }
  }

  fn lda(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    let param = self.mem_read(operand_addr);

    self.accumulator = param;
    self.update_negative_and_zero_flags(self.accumulator);
  }

  fn sta(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    self.mem_write(operand_addr, self.accumulator);
  }

  fn tax(&mut self) {
    self.register_x = self.accumulator;
    self.update_negative_and_zero_flags(self.register_x);
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_negative_and_zero_flags(self.register_x);
  }

  fn run(&mut self) {
    let all_op_codes: &HashMap<u8, &'static opcodes::Opcode> = &(*opcodes::OPCODES_MAP);

    loop {
      let code = self.mem_read(self.program_counter);
      self.program_counter += 1;
      let current_program_counter_state = self.program_counter;

      let current_opcode = all_op_codes
        .get(&code)
        .unwrap_or_else(|| panic!("OP code {:x} not found", code));

      match code {
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
          self.lda(&current_opcode.addressing_mode);
        }
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
          self.sta(&current_opcode.addressing_mode);
        }
        0xAA => self.tax(),
        0xE8 => self.inx(),
        0x00 => return,
        _ => todo!(),
      }

      if current_program_counter_state == self.program_counter {
        self.program_counter += (current_opcode.bytes - 1) as u16;
      }
    }
  }

  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run();
  }
}
