use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0xe0_cpx_immediate_mode_should_compare_equal_value_to_acc_and_set_zero_and_carry_flag() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let program = vec![0xA2, value, 0xE0, value, 0x00]; // LDX #$11; CPX #$11;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xe0_cpx_immediate_mode_should_compare_different_value_to_acc_and_only_set_negative_and_carry_flag_if_appropriated() {
  // arrange
  let expected_status_flags = 0b1000_0001;
  let value = 0xFF;
  let program = vec![0xA2, value, 0xE0, 0x02, 0x00]; // LDX #$FF; CPX #$02;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xe4_cpx_zeropage_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let acc_value = 0x17;
  let zeropage_addr = 0x33u8;
  let program = vec![0xA2, acc_value, 0xE4, zeropage_addr, 0x00]; // LDX #$17; CPX $33;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(zeropage_addr as u16, acc_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xec_cpx_absolute_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let program = vec![
    0xA2,
    value,
    0xEC,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDX #$11; CPX $4433;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(absolute_addr, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}
