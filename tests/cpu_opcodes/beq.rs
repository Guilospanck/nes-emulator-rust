use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0xf0_beq_will_not_branch_because_zero_flag_is_clear() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let acc_value = 0xEF;
  let sec_value = 0x01;
  let value = 0xAA;
  let beq_relative_step = 0x04;
  let expected_acc_value = 0xF1;
  let program = vec![
    0xA9,
    acc_value,
    0xC9,
    value,
    0xF0,
    beq_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // LDA #$EF; CMP #$AA; BEQ #$04; ADC #$01; ADC #$01; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_acc_value);
}

#[test]
fn test_0xf0_beq_will_branch_because_zero_flag_is_set() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let acc_value = 0xEF;
  let sec_value = 0x01;
  let value = 0xA;
  let beq_relative_step = 0xEF;
  let expected_acc_value = 0xF1;
  let program = vec![
    0xA9,
    acc_value,
    0xC9,
    value,
    0xF0,
    beq_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // LDA #$EF; CMP #$EF; BEQ #$04; ADC #$01; ADC #$01; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_acc_value);
}