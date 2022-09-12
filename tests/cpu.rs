use nes_emulator_rust::cpu::CPU;

#[test]
fn test_should_get_lda_immediate_instruction_and_brk_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let program = vec![0xA9, 0x10, 0x00]; // LDA #$0x10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.interpret(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_should_get_lda_immediate_instruction_and_brk_and_set_zero_status_flag() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let program = vec![0xA9, 0x00, 0x00]; // LDA #$0x00  BRK
  let mut cpu = CPU::new();

  // act
  cpu.interpret(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}