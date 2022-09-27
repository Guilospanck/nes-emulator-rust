use nes_emulator_rust::cpu::CPU;

// --------------- LDY --------------------
#[test]
fn test_0xa0_ldy_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x10;
  let program = vec![0xA0, expected_value, 0x00]; // LDY #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xa4_ldy_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xA4, addr, 0x00]; // LDY $10  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xb4_ldy_zeropage_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xB4, addr, 0x00]; // LDY $10, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xac_ldy_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAC, addr_lsb, addr_msb, 0x00]; // LDY $0307;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xbc_ldy_absolute_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xBC, addr_lsb, addr_msb, 0x00]; // LDY $0307, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}