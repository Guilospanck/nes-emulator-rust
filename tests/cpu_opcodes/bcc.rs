use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0x90_bcc_will_not_branch_because_carry_is_not_clear() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let first_value = 0xF0;
  let sec_value = 0x1F;
  let expected_final_acc_value = 0x4D; // 77(10)
  let bcc_relative_step = 0x04;
  let program = vec![
    0x69,
    first_value,
    0x69,
    sec_value,
    0x90,
    bcc_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // ADC #$F0; ADC #$1F; BCC #$04; ADC #$1F; ADC #$1F; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_acc_value);
}

#[test]
fn test_0x90_bcc_will_branch_because_carry_is_clear() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let first_value = 0xF0;
  let sec_value = 0x01;
  let expected_final_acc_value = 0xF1; // 241(10)
  let bcc_relative_step = 0x04;
  let program = vec![
    0x69,
    first_value,
    0x69,
    sec_value,
    0x90,
    bcc_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // ADC #$F0; ADC #$1F; BCC #$04; ADC #$1F; ADC #$1F; BRK
  let mut cpu = CPU::new();

  // act
  // it will just ADC 0xF0 and 0x1F and then branch out to BRK
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_acc_value);
}