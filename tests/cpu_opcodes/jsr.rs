use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0x20_jsr_add_to_acc_and_then_break() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let lsb_absolute_address = 0x06u8;
  let msb_absolute_address = 0x80u8;
  let acc_value = 0x99;  
  let sec_acc_value = 0x01;
  let expected_acc_value = 0x9A;
  let program = vec![
    0xA9,
    acc_value,
    0x20,
    lsb_absolute_address,
    msb_absolute_address,
    0x00,
    0x69,
    sec_acc_value,
    0x60,
  ]; // LDA #$99; JSR $8007; BRK; ADC #$99; RTS
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_acc_value);
}


#[test]
fn test_0x20_jsr_will_break_without_adding_to_acc() {
  // arrange
  let expected_status_flags = 0b1000_0000;
  let lsb_absolute_address = 0x08u8;
  let msb_absolute_address = 0x80u8;
  let acc_value = 0x99;  
  let expected_acc_value = 0x99;
  let program = vec![
    0xA9,
    acc_value,
    0x20,
    lsb_absolute_address,
    msb_absolute_address,
    0x69,
    acc_value,
    0x60,
    0x00,
  ]; // LDA #$99; JSR $8007; ADC #$99; RTS; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_acc_value);
}
