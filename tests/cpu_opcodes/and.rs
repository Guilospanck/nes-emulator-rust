use nes_emulator_rust::cpu::CPU;

// --------------- AND --------------------
#[test]
fn test_0x29_and_immediate_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x29, value_to_and, 0x00]; // LDA #$22; AND #$03;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x29_and_immediate_mode_should_and_compare_value_to_accumulator_and_set_zero_flag() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let initial_accumulator_value = 0x10u8;
  let value_to_and = 0x04u8;
  let program = vec![0xA9, initial_accumulator_value, 0x29, value_to_and, 0x00]; // LDA #$10; AND #$04;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, 0);
}

#[test]
fn test_0x25_and_zeropage_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x25, address_to_and, 0x00]; // LDA #$22; AND $44;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(address_to_and as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x35_and_zeropage_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x35,
    address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND $44, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((address_to_and + x_register_value) as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x2d_and_absolute_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x2D, lsb_address_to_and, msb_address_to_and, 0x00]; // LDA #$22; AND $2244;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x3d_and_absolute_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x3D,
    lsb_address_to_and,
    msb_address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND $2244, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and + x_register_value as u16, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x39_and_absolute_y_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let y_register_value = 0x05u8;
  let program = vec![
    0xA0,
    y_register_value,
    0xA9,
    initial_accumulator_value,
    0x39,
    lsb_address_to_and,
    msb_address_to_and,
    0x00,
  ]; // LDY #$05; LDA #$22; AND $2244, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and + y_register_value as u16, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x21_and_indirect_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let addr_at_indirect_addr = 0x55u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x21,
    address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND ($44, X);  BRK
  let mut cpu = CPU::new();
  // $0044 + x_register (#$05) = $0049
  cpu.mem_write((address_to_and + x_register_value) as u16, addr_at_indirect_addr);
  // addr to read value = $00 addr_at_indirect_addr = $0005
  cpu.mem_write(addr_at_indirect_addr as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x31_and_indirect_y_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_at_address_to_and = 0x77u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let y_register_value = 0x05u8;
  let addr_at_indirect_addr = 0x55u8;
  let program = vec![
    0xA2,
    y_register_value,
    0xA9,
    initial_accumulator_value,
    0x31,
    address_to_and,
    0x00,
  ]; // LDY #$05; LDA #$22; AND ($44), Y;  BRK
  let mut cpu = CPU::new();
  // get the value at the "address_to_and" ($0044)
  cpu.mem_write(address_to_and as u16, value_at_address_to_and);
  cpu.mem_write(value_at_address_to_and as u16, addr_at_indirect_addr + y_register_value);
  cpu.mem_write((addr_at_indirect_addr + y_register_value) as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}
