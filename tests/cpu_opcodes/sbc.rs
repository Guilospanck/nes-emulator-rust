use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0xe9_sbc_immediate_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let subtract_value = 0x11;
  let expected_value = 0x11;
  let program = vec![0xA9, value, 0xE9, subtract_value, 0x00]; // LDA #$22; SBC #$11;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xe9_sbc_immediate_mode_should_subtract_value_to_accumulator_and_set_overflow_and_carry_flags() {
  // arrange
  let expected_status_flags = 0b1100_0001;
  let value = 0x01;
  let expected_value = 0xFF;
  let program = vec![0xA9, value, 0xE9, 0x02, 0x00]; // LDA #$01; SBC #$02;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program); // 1100_0001

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xe5_sbc_zeropage_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let zeropage_addr = 0x33u8;
  let program = vec![0xA9, value, 0xE5, zeropage_addr, 0x00]; // LDA #$22; SBC $33;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(zeropage_addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xf5_sbc_zeropage_x_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let zeropage_addr = 0x33u8;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA2,
    x_register_value,
    0xF5,
    zeropage_addr,
    0x00,
  ]; // LDA #$22; LDX #$03; SBC $33, X; SBC $33, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((zeropage_addr.wrapping_add(x_register_value)) as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xed_sbc_absolute_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let program = vec![
    0xA9,
    value,
    0xED,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // SBC $4433; SBC $4433;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(absolute_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xfd_sbc_absolute_x_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA2,
    x_register_value,
    0xFD,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDX #$03; SBC $4433, X; SBC $4433, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + x_register_value as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xf9_sbc_absolute_y_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA0,
    y_register_value,
    0xF9,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDY #$03; SBC $4433, Y; SBC $4433, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + y_register_value as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xe1_sbc_indirect_x_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let value = 0x22;
  let expected_value = 0x11;
  let zeropage_addr = 0x33u8;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA2,
    x_register_value,
    0xE1,
    zeropage_addr,
    0xE1,
    zeropage_addr,
    0x00,
  ]; // LDX #$03; SBC ($33, X); SBC ($33, X);  BRK
  let mut cpu = CPU::new();
  // the lookup will de indirected to (zeropage + x = $33 + #$03 = $0036). Then, it
  // will see that there there is the values: $0036 = #$44; $0037 = #$00 => then the address
  // of the final value will be located at $0044
  cpu.mem_write_u16(zeropage_addr as u16 + x_register_value as u16, 0x44);
  cpu.mem_write_u16(0x0044, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, 0u8);
}

#[test]
fn test_0xf1_sbc_indirect_x_mode_should_subtract_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x22;
  let expected_value = 0x11;
  let zeropage_addr = 0x33u8;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA9,
    value,
    0xA0,
    y_register_value,
    0xF1,
    zeropage_addr,
    0x00,
  ]; // LDY #$03; SBC ($33), Y; SBC ($33), Y;  BRK
  let mut cpu = CPU::new();
  // Gets the address from the zeropage ($0033)
  cpu.mem_write(zeropage_addr as u16, 0x43); // Indirect lookup: $0033 = 0x43, $0034 = 0x00 => $0043
                                             // now, with this new address ($0043), we're going to add the y_register and then
                                             // look for the value that will exists at this new addr ($0043 + Y = $0046)
  cpu.mem_write(0x0046u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}