use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0x10_bpl_will_not_branch_because_negative_flag_is_set() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let first_value = 0xF0;
  let sec_value = 0x01;
  let expected_final_acc_value = 0xF3;
  let bpl_relative_step = 0x04;
  let program = vec![
    0xA9,
    first_value,
    0x69,
    sec_value,
    0x10,
    bpl_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // LDA #$F0; ADC #$01; BPL #$04; ADC #$01; ADC #$01; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_acc_value);
}

#[test]
fn test_0x10_bpl_will_branch_because_negative_flag_is_not_set() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let first_value = 0x02;
  let sec_value = 0x01;
  let expected_final_acc_value = 0x03;
  let bpl_relative_step = 0x04;
  let program = vec![
    0xA9,
    first_value,
    0x69,
    sec_value,
    0x10,
    bpl_relative_step,
    0x69,
    sec_value,
    0x69,
    sec_value,
    0x00,
  ]; // LDA #$F0; ADC #$01; BPL #$04; ADC #$01; ADC #$01; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_acc_value);
}
