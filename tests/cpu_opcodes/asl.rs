use nes_emulator_rust::cpu::CPU;

// --------------- ASL --------------------
#[test]
fn test_0x0a_asl_accumulator_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8; // 34(10)
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA9, initial_accumulator_value, 0x0A, 0x00]; // LDA #$22; ASL;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x0a_asl_accumulator_mode_should_shift_left_contents_of_accumulator_and_set_carry_flag() {
  // arrange
  let expected_status_flags = 0b0000_0001;
  let initial_accumulator_value = 0x81u8; // 129(10)
  let expected_accumulator_value = 0x02u8; // 258(10) - 256(10) = 2(10)
  let program = vec![0xA9, initial_accumulator_value, 0x0A, 0x00]; // LDA #$81; ASL;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x06_asl_zeropage_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let some_memory_location = 0x45u8;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0x06, some_memory_location, 0x00]; // ASL $45;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(some_memory_location as u16, value_to_be_written_at_some_memory_location);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x16_asl_zeropage_x_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let x_register_value = 0x02u8;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let some_memory_location = 0x45u8;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA2, x_register_value, 0x16, some_memory_location, 0x00]; // LDX #$02; ASL $45, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((some_memory_location + x_register_value) as u16, value_to_be_written_at_some_memory_location);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x0e_asl_absolute_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let lsb_some_memory_location = 0x33u8;
  let msb_some_memory_location = 0x55u8;
  let absolute_some_memory_location = 0x5533u16;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0x0E, lsb_some_memory_location, msb_some_memory_location, 0x00]; // ASL $5533;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_some_memory_location, value_to_be_written_at_some_memory_location as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x1e_asl_zeropage_x_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let x_register_value = 0x02u8;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let lsb_some_memory_location = 0x33u8;
  let msb_some_memory_location = 0x55u8;
  let absolute_some_memory_location = 0x5533u16;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA2, x_register_value, 0x1E, lsb_some_memory_location, msb_some_memory_location, 0x00]; // LDX #$02; ASL $5544, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_some_memory_location + x_register_value as u16, value_to_be_written_at_some_memory_location as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}