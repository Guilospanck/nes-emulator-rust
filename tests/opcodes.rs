use nes_emulator_rust::{cpu::AddressingMode, opcodes::{Opcode, OPCODES_MAP}};

#[test]
fn test_new_opcode() {
  // arrange
  let code: u8 = 0xFF;
  let name = "TEST";
  let bytes: u8 = 10;
  let _cycles: u8 = 8;
  let addressing_mode: AddressingMode = AddressingMode::NoneAddressing;

  // act
  let opcode = Opcode::new(code, name, bytes, _cycles, addressing_mode);
  
  // assert
  assert_eq!(opcode.code, code);
  assert_eq!(opcode.name, name);
  assert_eq!(opcode.bytes, bytes);
  assert_eq!(opcode.addressing_mode, AddressingMode::NoneAddressing);
}

#[test]
fn test_brk_from_opcodes_map_has_right_data(){
  // arrange
  let code: u8 = 0x00;
  let name = "BRK";
  let bytes: u8 = 1;
  let _cycles: u8 = 7;
  let addressing_mode = AddressingMode::NoneAddressing;
  let brk = Opcode::new(code, name, bytes, _cycles, addressing_mode);

  // act
  let opcodes = &*OPCODES_MAP;
  let brk_from_opcodes = opcodes.get(&0x00).unwrap();

  // assert
  assert_eq!(brk_from_opcodes.code, brk.code);
  assert_eq!(brk_from_opcodes.name, brk.name);
  assert_eq!(brk_from_opcodes.bytes, brk.bytes);
  assert_eq!(brk_from_opcodes.addressing_mode, brk.addressing_mode);
}