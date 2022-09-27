use nes_emulator_rust::cpu::CPU;

// ------------------- STX -------------------
#[test]
fn test_0x86_stx_zeropage_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA2, expected_value, 0x86, addr, 0x00]; // LDX #$11; STX $33;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
  assert_eq!(cpu.mem_read(addr as u16), expected_value);
}

#[test]
fn test_0x96_stx_zeropage_y_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let y_value = 0x03;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA2, expected_value, 0xA0, y_value, 0x96, addr, 0x00]; // LDX #$11; LDY #$03; STX $33, Y;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
  assert_eq!(cpu.mem_read((addr + y_value) as u16), expected_value);
}

#[test]
fn test_0x8e_stx_absolute_mode_should_store_x_register_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr_lsb = 0x33u8;
  let addr_msb = 0x22u8;
  let absolute_addr = 0x2233_u16;
  let program = vec![0xA2, expected_value, 0x8E, addr_lsb, addr_msb, 0x00]; // LDX #$11; STX $2233;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
  assert_eq!(cpu.mem_read_u16(absolute_addr), expected_value as u16);
}