use nes_emulator_rust::cpu::CPU;

/// TODO
/// Addressing Modes that use X and Y (ZeroPage, Absolute and Indirect)
/// Must be improved when LDX (or STX) and LDY (or STY) are created.

// --------------- LDA --------------------

#[test]
fn test_0xa9_lda_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let program = vec![0xA9, 0x10, 0x00]; // LDA #$0x10  BRK
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
  let program = vec![0xA9, 0x00, 0x00]; // LDA #$0x00 ; BRK
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

// --------------- STA ----------------------

#[test]
fn test_0x85_sta_zeropage_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x33;
  let program = vec![0xA9, expected_value, 0x85, expected_addr, 0x00]; // LDA #$0x11; STA $33;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr as u16), expected_value);
}

#[test]
fn test_0x95_sta_zeropage_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x33;
  let program = vec![0xA9, expected_value, 0x95, expected_addr, 0x00]; // LDA #$0x11; STA $33, X;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr as u16 + cpu.register_x as u16), expected_value);
}

#[test]
fn test_0x8d_sta_absolute_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x8D, 0x44, 0x33, 0x00]; // LDA #$0x11; STA $3344;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read_u16(expected_addr), expected_value as u16);
}

#[test]
fn test_0x9d_sta_absolute_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x9D, 0x44, 0x33, 0x00]; // LDA #$0x11; STA $3344, X;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read_u16(expected_addr + cpu.register_x as u16), expected_value as u16);
}

#[test]
fn test_0x99_sta_absolute_y_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x99, 0x44, 0x33, 0x00]; // LDA #$0x11; STA $3344, Y;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read_u16(expected_addr + cpu.register_y as u16), expected_value as u16);
}

#[test]
fn test_0x81_sta_indirect_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA9, expected_value, 0x81, addr, 0x00]; // LDA #$0x11; STA ($33, X);  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, 0x05);
  cpu.mem_write(addr.wrapping_add(1) as u16, 0x06);
  let expected_addr = 0x0605_u16; // Indirect Lookup will be solved to this address (acc = $33, x = $00. Then => $33 + $00 = $33)

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr), expected_value);
}

#[test]
fn test_0x91_sta_indirect_y_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA9, expected_value, 0x91, addr, 0x00]; // LDA #$0x11; STA ($33), Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, 0x05);
  cpu.mem_write(addr.wrapping_add(1) as u16, 0x06);
  let expected_addr = 0x0605_u16;

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr), expected_value);
}

// ---------------------------------------------

#[test]
fn test_0xaa_tax_should_move_acc_to_x() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x01;
  let program = vec![0xA9, expected_value, 0xAA, 0x00]; // LDA $%01; TAX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xe8_inx_should_increment_x_register() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x02;
  let program = vec![0xA9, expected_value, 0xAA, 0xE8, 0x00]; // LDA $%02; TAX; INX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0x00);
  assert_eq!(cpu.register_x, 0x03);
}

#[test]
fn test_0xe8_inx_should_overflow_x_register() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]; // LDA $%FF; TAX; INX; INX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0x00);
  assert_eq!(cpu.register_x, 0x01);
}

// Should fail
#[test]
#[should_panic(expected = "OP code ff not found")]
fn test_run_should_panic_unknown_opcode(){
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xFF]; // wrong opcode
  
  // act
  cpu.load_and_run(program);

  // assert
}