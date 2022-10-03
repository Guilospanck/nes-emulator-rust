use core::panic;
use std::collections::HashMap;

use crate::opcodes;

const MEMORY_SIZE: u16 = 0xFFFF;
const PROGRAM_ROM_MEMORY_ADDRESS_START: u16 = 0x8000;
const RESET_INTERRUPT_ADDR: u16 = 0xFFFC;

/// See `studies/addressing.asm` for more info.
#[derive(Debug, PartialEq, Eq)]
pub enum AddressingMode {
  Accumulator,
  Relative,
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
    self.stack_pointer = 0xFF;
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

  fn update_zero_flag(&mut self, result: u8) {
    if result == 0 {
      self.status |= 0b0000_0010; // set zero flag to 1
    } else {
      self.status &= 0b1111_1101; // set zero flag to 0
    }
  }

  fn update_negative_flag(&mut self, result: u8) {
    if result & 0b1000_0000 != 0 {
      self.status |= 0b1000_0000; // set negative flag to 1
    } else {
      self.status &= 0b0111_1111; // set negative flag to 0
    }
  }

  fn update_negative_and_zero_flags(&mut self, result: u8) {
    self.update_zero_flag(result);
    self.update_negative_flag(result);
  }

  fn set_carry_flag(&mut self) {
    self.status |= 0b0000_0001;
  }

  fn update_carry_and_overflow_flags(&mut self, result: Option<u8>) {
    match result {
      Some(_e) => self.status &= 0b1011_1110,
      None => self.status |= 0b0100_0001,
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
      AddressingMode::Relative => self.program_counter,
      AddressingMode::NoneAddressing => panic!("Mode not known"),
      _ => panic!("Mode not known"),
    }
  }

  fn get_address_from_stack(&self, stack_pointer: u8) -> u16 {
    let absolute_stack_pointer = format!("01{:X}", stack_pointer);
    u16::from_str_radix(&absolute_stack_pointer, 16).unwrap()   
  }

  fn insert_address_into_stack(&mut self) -> u16 {
    let address = self.get_address_from_stack(self.stack_pointer);
    self.stack_pointer -= 1;
    address
  }

  fn remove_address_into_stack(&mut self) -> u16 {
    self.stack_pointer += 1;
    self.get_address_from_stack(self.stack_pointer)
  }

  fn adc(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    let old_accumulator = self.accumulator;

    self.accumulator = self.accumulator.wrapping_add(param);

    self.update_negative_and_zero_flags(self.accumulator);
    self.update_carry_and_overflow_flags(old_accumulator.checked_add(param));
  }

  fn and(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    self.accumulator &= param;

    self.update_negative_and_zero_flags(self.accumulator);
  }

  fn asl(&mut self, mode: &AddressingMode) {
    let seventh_bit = format!("{:08b}", self.accumulator)
      .chars()
      .collect::<Vec<char>>()[0];

    if *mode == AddressingMode::Accumulator {
      self.accumulator <<= 1;
    } else {
      let operand_addr = self.get_operand_addr(mode);
      let param = self.mem_read(operand_addr);

      self.accumulator = param << 1;
    }

    self.update_negative_and_zero_flags(self.accumulator);
    // Update carry flag with old seventh bit
    let bits = format!("0000000{}", seventh_bit);
    let new_bits = u8::from_str_radix(&bits, 2).unwrap();
    self.status |= new_bits;
  }

  fn bcc(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    let step = param + 1;

    if self.status & 0b0000_0001 == 0 {
      self.program_counter = self.program_counter.wrapping_add(step as u16);
    }
  }

  fn bcs(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    let step = param + 1;

    if self.status & 0b0000_0001 == 1 {
      self.program_counter = self.program_counter.wrapping_add(step as u16);
    }
  }

  fn beq(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    let step = param + 1;

    if self.status & 0b0000_0010 == 2 { // if zero flag is set
      self.program_counter = self.program_counter.wrapping_add(step as u16);
    }
  }

  fn cmp(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);
    let param = self.mem_read(operand_addr);

    if self.accumulator == param {
      self.set_carry_flag();
      self.update_zero_flag(0u8);      
    }

    if self.accumulator > param {
      self.set_carry_flag();
    }
    
    self.update_negative_flag(self.accumulator);
  }

  fn jsr(&mut self, mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(mode);

    // save old program counter to stack
    let old_program_counter = self.program_counter + 1;
    let lsb = (old_program_counter & 0xFF) as u8;
    let hsb = (old_program_counter >> 8) as u8;
    self.memory[self.insert_address_into_stack() as usize] = hsb;
    self.memory[self.insert_address_into_stack() as usize] = lsb;

    // update program counter (to jump to a subroutine)
    self.program_counter = operand_addr;    
  }

  fn lda(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    let param = self.mem_read(operand_addr);

    self.accumulator = param;
    self.update_negative_and_zero_flags(self.accumulator);
  }

  fn ldx(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    let param = self.mem_read(operand_addr);

    self.register_x = param;
    self.update_negative_and_zero_flags(self.register_x);
  }

  fn ldy(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    let param = self.mem_read(operand_addr);

    self.register_y = param;
    self.update_negative_and_zero_flags(self.register_y);
  }

  fn rts(&mut self) {
    // retrieve program counter from stack
    let lsb_address = self.memory[self.remove_address_into_stack() as usize];
    let msb_address = self.memory[self.remove_address_into_stack() as usize];

    let absolute_address = (msb_address as u16) << 8 | lsb_address as u16;

    // return from subroutine
    self.program_counter = absolute_address + 1; 
  }

  fn sta(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    self.mem_write(operand_addr, self.accumulator);
  }

  fn stx(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    self.mem_write(operand_addr, self.register_x);
  }

  fn sty(&mut self, addressing_mode: &AddressingMode) {
    let operand_addr = self.get_operand_addr(addressing_mode);
    self.mem_write(operand_addr, self.register_y);
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
        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
          self.adc(&current_opcode.addressing_mode);
        }
        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
          self.and(&current_opcode.addressing_mode);
        }
        0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
          self.asl(&current_opcode.addressing_mode);
        }
        0x90 => self.bcc(&current_opcode.addressing_mode),
        0xB0 => self.bcs(&current_opcode.addressing_mode),
        0xF0 => self.beq(&current_opcode.addressing_mode),
        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
          self.cmp(&current_opcode.addressing_mode);
        }
        0x20 => self.jsr(&current_opcode.addressing_mode),
        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
          self.lda(&current_opcode.addressing_mode);
        }
        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
          self.ldx(&current_opcode.addressing_mode);
        }
        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
          self.ldy(&current_opcode.addressing_mode);
        }
        0x60 => self.rts(),
        0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
          self.sta(&current_opcode.addressing_mode);
        }
        0x86 | 0x96 | 0x8E => {
          self.stx(&current_opcode.addressing_mode);
        }
        0x84 | 0x94 | 0x8C => {
          self.sty(&current_opcode.addressing_mode);
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
