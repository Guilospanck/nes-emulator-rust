use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0x18_clc_will_not_branch_because_carry_is_not_clear() {
  // arrange
  let expected_status_flags = 0b0100_0000;
  let first_value = 0xF0;
  let sec_value = 0x1F;
  let expected_final_acc_value = 0x0F; // 15(10)
  let program = vec![
    0xA9,
    first_value,
    0x69,
    sec_value,
    0x18,
    0x00,
  ]; // LDA #$F0; ADC #$1F; CLC; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_acc_value);
}
