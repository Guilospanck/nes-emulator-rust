use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::cpu::AddressingMode;

#[derive(Debug)]
pub struct Opcode {
  pub code: u8,
  pub name: &'static str,
  pub bytes: u8,
  _cycles: u8,
  pub addressing_mode: AddressingMode,
}

impl Opcode {
  pub fn new(
    code: u8,
    name: &'static str,
    bytes: u8,
    _cycles: u8,
    addressing_mode: AddressingMode,
  ) -> Self {
    Self {
      code,
      name,
      bytes,
      _cycles,
      addressing_mode,
    }
  }
}

lazy_static! {
  pub static ref CPU_OP_CODES: Vec<Opcode> = vec![
    Opcode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
    Opcode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0x6D, "ADC", 3, 4, AddressingMode::Absolute),
    Opcode::new(0x7D, "ADC", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
    Opcode::new(0x79, "ADC", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
    Opcode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
    Opcode::new(0x71, "ADC", 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

    Opcode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
    Opcode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0x2D, "AND", 3, 4, AddressingMode::Absolute),
    Opcode::new(0x3D, "AND", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
    Opcode::new(0x39, "AND", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
    Opcode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
    Opcode::new(0x31, "AND", 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

    Opcode::new(0x0A, "ASL", 1, 2, AddressingMode::Accumulator),
    Opcode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
    Opcode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
    Opcode::new(0x0E, "ASL", 3, 6, AddressingMode::Absolute),
    Opcode::new(0x1E, "ASL", 3, 7 /* +1 if page crossed */, AddressingMode::AbsoluteX),

    Opcode::new(0x90, "BCC", 2, 2 /* 2 (+1 if branch succeeds, +2 if to a new page) */, AddressingMode::Relative),
    
    Opcode::new(0xB0, "BCS", 2, 2 /* 2 (+1 if branch succeeds, +2 if to a new page) */, AddressingMode::Relative),
    
    Opcode::new(0xF0, "BEQ", 2, 2 /* 2 (+1 if branch succeeds, +2 if to a new page) */, AddressingMode::Relative),

    Opcode::new(0xC9, "CMP", 2, 2, AddressingMode::Immediate),
    Opcode::new(0xC5, "CMP", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0xD5, "CMP", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0xCD, "CMP", 3, 4, AddressingMode::Absolute),
    Opcode::new(0xDD, "CMP", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
    Opcode::new(0xD9, "CMP", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
    Opcode::new(0xC1, "CMP", 2, 6, AddressingMode::IndirectX),
    Opcode::new(0xD1, "CMP", 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

    Opcode::new(0x20, "JSR", 3, 6, AddressingMode::Absolute),

    Opcode::new(0xA9, "LDA", 2, 2, AddressingMode::Immediate),
    Opcode::new(0xA5, "LDA", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0xB5, "LDA", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0xAD, "LDA", 3, 4, AddressingMode::Absolute),
    Opcode::new(0xBD, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),
    Opcode::new(0xB9, "LDA", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),
    Opcode::new(0xA1, "LDA", 2, 6, AddressingMode::IndirectX),
    Opcode::new(0xB1, "LDA", 2, 5 /* +1 if page crossed */, AddressingMode::IndirectY),

    Opcode::new(0xA2, "LDX", 2, 2, AddressingMode::Immediate),
    Opcode::new(0xA6, "LDX", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0xB6, "LDX", 2, 4, AddressingMode::ZeroPageY),
    Opcode::new(0xAE, "LDX", 3, 4, AddressingMode::Absolute),
    Opcode::new(0xBE, "LDX", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteY),

    Opcode::new(0xA0, "LDY", 2, 2, AddressingMode::Immediate),
    Opcode::new(0xA4, "LDY", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0xB4, "LDY", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0xAC, "LDY", 3, 4, AddressingMode::Absolute),
    Opcode::new(0xBC, "LDY", 3, 4 /* +1 if page crossed */, AddressingMode::AbsoluteX),

    Opcode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

    Opcode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0x8D, "STA", 3, 4, AddressingMode::Absolute),
    Opcode::new(0x9D, "STA", 3, 5, AddressingMode::AbsoluteX),
    Opcode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
    Opcode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
    Opcode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),

    Opcode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
    Opcode::new(0x8E, "STX", 3, 4, AddressingMode::Absolute),

    Opcode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
    Opcode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
    Opcode::new(0x8C, "STY", 3, 4, AddressingMode::Absolute),

    Opcode::new(0xAA, "TXA", 1, 2, AddressingMode::NoneAddressing),

    Opcode::new(0xE8, "INX", 1, 2, AddressingMode::NoneAddressing),

    Opcode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
  ];

  pub static ref OPCODES_MAP: HashMap<u8, &'static Opcode> = {
    let mut map = HashMap::new();
    for opcode in &*CPU_OP_CODES {
      map.insert(opcode.code, opcode);
    }
    map
  };
}
