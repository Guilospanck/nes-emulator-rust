use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0xc9_cmp_immediate_mode_should_compare_equal_value_to_acc_and_set_zero_and_carry_flag() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let program = vec![0xA9, value, 0xC9, value, 0x00]; // LDA #$11; CMP #$11;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xc9_cmp_immediate_mode_should_compare_different_value_to_acc_and_only_set_negative_and_carry_flag_if_appropriated() {
  // arrange
  let expected_status_flags = 0b1000_0001;
  let value = 0xFF;
  let program = vec![0xA9, value, 0xC9, 0x02, 0x00]; // LDA #$FF; CMP #$02;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xc5_cmp_zeropage_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let acc_value = 0x17;
  let zeropage_addr = 0x33u8;
  let program = vec![0xA9, acc_value, 0xC5, zeropage_addr, 0x00]; // LDA #$17; CMP $33;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(zeropage_addr as u16, acc_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xd5_cmp_zeropage_x_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let acc_value = 0x17;
  let x_register_value = 0x03u8;
  let zeropage_addr = 0x33u8;
  let program = vec![
    0xA9,
    acc_value,
    0xA2,
    x_register_value,
    0xD5,
    zeropage_addr,
    0x00,
  ]; // LDA #$17; LDX #$03, X; CMP $33, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((zeropage_addr.wrapping_add(x_register_value)) as u16, acc_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xcd_cmp_absolute_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let program = vec![
    0xA9,
    value,
    0xCD,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDA #$11; CMP $4433;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(absolute_addr, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xdd_cmp_absolute_x_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA2,
    x_register_value,
    0xDD,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDA #$11; LDX #$03; CMP $4433, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + x_register_value as u16, value as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xd9_cmp_absolute_y_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0011;
  let value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA0,
    y_register_value,
    0xD9,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDA #$11; LDY #$03; CMP $4433, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + y_register_value as u16, value as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xc1_cmp_indirect_x_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let zeropage_addr = 0x33u8;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xC1,
    zeropage_addr,
    0x00,
  ]; // LDX #$03; CMP ($33, X); CMP ($33, X);  BRK
  let mut cpu = CPU::new();
  // the lookup will de indirected to (zeropage + x = $33 + #$03 = $0036). Then, it
  // will see that there there is the values: $0036 = #$44; $0037 = #$00 => then the address
  // of the final value will be located at $0044
  cpu.mem_write_u16(zeropage_addr as u16 + x_register_value as u16, 0x44);
  cpu.mem_write_u16(0x0044, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xd1_cmp_indirect_y_mode_should_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let zeropage_addr = 0x33u8;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA0,
    y_register_value,
    0xD1,
    zeropage_addr,
    0x00,
  ]; // LDY #$03; CMP ($33), Y; CMP ($33), Y;  BRK
  let mut cpu = CPU::new();
  // Gets the address from the zeropage ($0033)
  cpu.mem_write(zeropage_addr as u16, 0x43); // Indirect lookup: $0033 = 0x43, $0034 = 0x00 => $0043
                                             // now, with this new address ($0043), we're going to add the y_register and then
                                             // look for the value that will exists at this new addr ($0043 + Y = $0046)
  cpu.mem_write(0x0046u16, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}