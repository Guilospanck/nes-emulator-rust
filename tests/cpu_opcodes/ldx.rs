use nes_emulator_rust::cpu::CPU;

// --------------- LDX --------------------
#[test]
fn test_0xa2_ldx_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x10;
  let program = vec![0xA2, expected_value, 0x00]; // LDX #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xa6_ldx_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xA6, addr, 0x00]; // LDX $10  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xb6_ldx_zeropage_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xB6, addr, 0x00]; // LDX $10, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xae_ldx_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAE, addr_lsb, addr_msb, 0x00]; // LDX $0307;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xae_ldx_absolute_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAE, addr_lsb, addr_msb, 0x00]; // LDX $0307, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}