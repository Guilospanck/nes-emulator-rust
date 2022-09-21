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
