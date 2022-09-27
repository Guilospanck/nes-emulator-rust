use nes_emulator_rust::cpu::CPU;

// --------------- LDA --------------------

#[test]
fn test_0xa9_lda_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let program = vec![0xA9, 0x10, 0x00]; // LDA #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xa9_lda_immediate_mode_should_get_instruction_and_brk_and_set_zero_status_flag() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let program = vec![0xA9, 0x00, 0x00]; // LDA #$00 ; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xa5_lda_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA5, 0x33, 0x00]; // LDA $33; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xa5_lda_zeropage_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x22;
  cpu.mem_write(0x33, expected_value);
  let program = vec![0xA5, 0x33, 0x00]; // LDA $33; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xb5_lda_zeropage_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB5, 0x33, 0x00]; // LDA $33, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb5_lda_zeropage_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x33, expected_value);
  let program = vec![0xB5, 0x33, 0x00]; // LDA $33, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xad_lda_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xAD, 0x33, 0xC4, 0x00]; // LDA $C433; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xad_lda_absolute_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xAD, 0x33, 0xC4, 0x00]; // LDA $C433; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xbd_lda_absolute_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xBD, 0x33, 0xC4, 0x00]; // LDA $C433, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xbd_lda_absolute_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xBD, 0x33, 0xC4, 0x00]; // LDA $C433, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xb9_lda_absolute_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB9, 0x33, 0xC4, 0x00]; // LDA $C433, Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb9_lda_absolute_y_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xB9, 0x33, 0xC4, 0x00]; // LDA $C433, Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xa1_lda_indirect_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA1, 0x33, 0x00]; // LDA ($33, X); BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xa1_lda_indirect_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x00, expected_value);
  let program = vec![0xA1, 0x33, 0x00]; // LDA ($33, X); BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xb1_lda_indirect_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB1, 0x33, 0x00]; // LDA ($33), Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb1_lda_indirect_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x00, expected_value);
  let program = vec![0xB1, 0x33, 0x00]; // LDA ($33), Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}