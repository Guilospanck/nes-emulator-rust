use nes_emulator_rust::cpu::CPU;

// ------------------- STY -------------------
#[test]
fn test_0x84_sty_zeropage_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA0, expected_value, 0x84, addr, 0x00]; // LDY #$11; STY $33;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
  assert_eq!(cpu.mem_read(addr as u16), expected_value);
}

#[test]
fn test_0x94_sty_zeropage_y_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let x_value = 0x03;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA0, expected_value, 0xA2, x_value, 0x94, addr, 0x00]; // LDY #$11; LDX #$03; STY $33, X;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
  assert_eq!(cpu.mem_read((addr + x_value) as u16), expected_value);
}

#[test]
fn test_0x8c_sty_absolute_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr_lsb = 0x33u8;
  let addr_msb = 0x22u8;
  let absolute_addr = 0x2233_u16;
  let program = vec![0xA0, expected_value, 0x8C, addr_lsb, addr_msb, 0x00]; // LDY #$11; STY $2233;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
  assert_eq!(cpu.mem_read_u16(absolute_addr), expected_value as u16);
}