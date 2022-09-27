use nes_emulator_rust::cpu::CPU;

// ------------------- TAX -------------------
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
fn test_run_should_panic_unknown_opcode() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xFF]; // wrong opcode

  // act
  cpu.load_and_run(program);

  // assert
}
