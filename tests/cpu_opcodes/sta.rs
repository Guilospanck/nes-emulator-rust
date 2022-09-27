use nes_emulator_rust::cpu::CPU;

// --------------- STA ----------------------

#[test]
fn test_0x85_sta_zeropage_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x33;
  let program = vec![0xA9, expected_value, 0x85, expected_addr, 0x00]; // LDA #$11; STA $33;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr as u16), expected_value);
}

#[test]
fn test_0x95_sta_zeropage_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x33;
  let program = vec![0xA9, expected_value, 0x95, expected_addr, 0x00]; // LDA #$11; STA $33, X;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(
    cpu.mem_read(expected_addr as u16 + cpu.register_x as u16),
    expected_value
  );
}

#[test]
fn test_0x8d_sta_absolute_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x8D, 0x44, 0x33, 0x00]; // LDA #$11; STA $3344;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read_u16(expected_addr), expected_value as u16);
}

#[test]
fn test_0x9d_sta_absolute_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x9D, 0x44, 0x33, 0x00]; // LDA #$11; STA $3344, X;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(
    cpu.mem_read_u16(expected_addr + cpu.register_x as u16),
    expected_value as u16
  );
}

#[test]
fn test_0x99_sta_absolute_y_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let expected_addr = 0x3344_u16;
  let program = vec![0xA9, expected_value, 0x99, 0x44, 0x33, 0x00]; // LDA #$11; STA $3344, Y;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(
    cpu.mem_read_u16(expected_addr + cpu.register_y as u16),
    expected_value as u16
  );
}

#[test]
fn test_0x81_sta_indirect_x_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA9, expected_value, 0x81, addr, 0x00]; // LDA #$11; STA ($33, X);  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, 0x05);
  cpu.mem_write(addr.wrapping_add(1) as u16, 0x06);
  let expected_addr = 0x0605_u16; // Indirect Lookup will be solved to this address (acc = $33, x = $00. Then => $33 + $00 = $33)

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr), expected_value);
}

#[test]
fn test_0x91_sta_indirect_y_mode_should_store_acc_at_the_right_addr() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x11;
  let addr = 0x33_u8;
  let program = vec![0xA9, expected_value, 0x91, addr, 0x00]; // LDA #$11; STA ($33), Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, 0x05);
  cpu.mem_write(addr.wrapping_add(1) as u16, 0x06);
  let expected_addr = 0x0605_u16;

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.mem_read(expected_addr), expected_value);
}
