use nes_emulator_rust::cpu::CPU;

// --------------- ADC --------------------
#[test]
fn test_0x69_adc_immediate_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let program = vec![0x69, value, 0x69, value, 0x00]; // ADC #$11; ADC #$11;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x69_adc_immediate_mode_should_add_value_to_accumulator_and_set_overflow_and_carry_flags() {
  // arrange
  let expected_status_flags = 0b1100_0001;
  let value = 0xFE;
  let expected_value = 0xFF;
  let program = vec![0x69, value, 0x69, 0x01, 0x00]; // ADC #$FF; ADC #$01;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program); // 1100_0001

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x65_adc_zeropage_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let zeropage_addr = 0x33u8;
  let program = vec![0x65, zeropage_addr, 0x65, zeropage_addr, 0x00]; // ADC $33; ADC $33;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(zeropage_addr as u16, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x75_adc_zeropage_x_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let zeropage_addr = 0x33u8;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA2,
    x_register_value,
    0x75,
    zeropage_addr,
    0x75,
    zeropage_addr,
    0x00,
  ]; // LDX #$03; ADC $33, X; ADC $33, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((zeropage_addr.wrapping_add(x_register_value)) as u16, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x6d_adc_absolute_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let program = vec![
    0x6D,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x6D,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // ADC $4433; ADC $4433;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(absolute_addr, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x7d_adc_absolute_x_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA2,
    x_register_value,
    0x7D,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x7D,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDX #$03; ADC $4433, X; ADC $4433, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + x_register_value as u16, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x79_adc_absolute_y_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let lsb_absolute_addr = 0x33u8;
  let msb_absolute_addr = 0x44u8;
  let absolute_addr = 0x4433u16;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA0,
    y_register_value,
    0x79,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x79,
    lsb_absolute_addr,
    msb_absolute_addr,
    0x00,
  ]; // LDY #$03; ADC $4433, Y; ADC $4433, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_addr + y_register_value as u16, value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x61_adc_indirect_x_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let zeropage_addr = 0x33u8;
  let x_register_value = 0x03u8;
  let program = vec![
    0xA2,
    x_register_value,
    0x61,
    zeropage_addr,
    0x61,
    zeropage_addr,
    0x00,
  ]; // LDX #$03; ADC ($33, X); ADC ($33, X);  BRK
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
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0x71_adc_indirect_x_mode_should_add_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value = 0x11;
  let expected_value = 0x22;
  let zeropage_addr = 0x33u8;
  let y_register_value = 0x03u8;
  let program = vec![
    0xA0,
    y_register_value,
    0x71,
    zeropage_addr,
    0x71,
    zeropage_addr,
    0x00,
  ]; // LDY #$03; ADC ($33), Y; ADC ($33), Y;  BRK
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
  assert_eq!(cpu.accumulator, expected_value);
}

// --------------- AND --------------------
#[test]
fn test_0x29_and_immediate_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x29, value_to_and, 0x00]; // LDA #$22; AND #$03;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x29_and_immediate_mode_should_and_compare_value_to_accumulator_and_set_zero_flag() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let initial_accumulator_value = 0x10u8;
  let value_to_and = 0x04u8;
  let program = vec![0xA9, initial_accumulator_value, 0x29, value_to_and, 0x00]; // LDA #$10; AND #$04;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, 0);
}

#[test]
fn test_0x25_and_zeropage_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x25, address_to_and, 0x00]; // LDA #$22; AND $44;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(address_to_and as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x35_and_zeropage_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x35,
    address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND $44, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((address_to_and + x_register_value) as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x2d_and_absolute_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let program = vec![0xA9, initial_accumulator_value, 0x2D, lsb_address_to_and, msb_address_to_and, 0x00]; // LDA #$22; AND $2244;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x3d_and_absolute_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x3D,
    lsb_address_to_and,
    msb_address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND $2244, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and + x_register_value as u16, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x39_and_absolute_y_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let lsb_address_to_and = 0x44u8;
  let msb_address_to_and = 0x22u8;
  let address_to_and = 0x2244u16;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let y_register_value = 0x05u8;
  let program = vec![
    0xA0,
    y_register_value,
    0xA9,
    initial_accumulator_value,
    0x39,
    lsb_address_to_and,
    msb_address_to_and,
    0x00,
  ]; // LDY #$05; LDA #$22; AND $2244, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(address_to_and + y_register_value as u16, value_to_and as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x21_and_indirect_x_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let x_register_value = 0x05u8;
  let addr_at_indirect_addr = 0x55u8;
  let program = vec![
    0xA2,
    x_register_value,
    0xA9,
    initial_accumulator_value,
    0x21,
    address_to_and,
    0x00,
  ]; // LDX #$05; LDA #$22; AND ($44, X);  BRK
  let mut cpu = CPU::new();
  // $0044 + x_register (#$05) = $0049
  cpu.mem_write((address_to_and + x_register_value) as u16, addr_at_indirect_addr);
  // addr to read value = $00 addr_at_indirect_addr = $0005
  cpu.mem_write(addr_at_indirect_addr as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

#[test]
fn test_0x31_and_indirect_y_mode_should_and_compare_value_to_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8;
  let address_to_and = 0x44u8;
  let value_at_address_to_and = 0x77u8;
  let value_to_and = 0x03u8;
  let expected_final_value = 0x02u8;
  let y_register_value = 0x05u8;
  let addr_at_indirect_addr = 0x55u8;
  let program = vec![
    0xA2,
    y_register_value,
    0xA9,
    initial_accumulator_value,
    0x31,
    address_to_and,
    0x00,
  ]; // LDY #$05; LDA #$22; AND ($44), Y;  BRK
  let mut cpu = CPU::new();
  // get the value at the "address_to_and" ($0044)
  cpu.mem_write(address_to_and as u16, value_at_address_to_and);
  cpu.mem_write(value_at_address_to_and as u16, addr_at_indirect_addr + y_register_value);
  cpu.mem_write((addr_at_indirect_addr + y_register_value) as u16, value_to_and);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_final_value);
}

// --------------- ASL --------------------
#[test]
fn test_0x0a_asl_accumulator_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let initial_accumulator_value = 0x22u8; // 34(10)
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA9, initial_accumulator_value, 0x0A, 0x00]; // LDA #$22; ASL;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x0a_asl_accumulator_mode_should_shift_left_contents_of_accumulator_and_set_carry_flag() {
  // arrange
  let expected_status_flags = 0b0000_0001;
  let initial_accumulator_value = 0x81u8; // 129(10)
  let expected_accumulator_value = 0x02u8; // 258(10) - 256(10) = 2(10)
  let program = vec![0xA9, initial_accumulator_value, 0x0A, 0x00]; // LDA #$81; ASL;  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x06_asl_zeropage_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let some_memory_location = 0x45u8;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0x06, some_memory_location, 0x00]; // ASL $45;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(some_memory_location as u16, value_to_be_written_at_some_memory_location);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x16_asl_zeropage_x_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let x_register_value = 0x02u8;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let some_memory_location = 0x45u8;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA2, x_register_value, 0x16, some_memory_location, 0x00]; // LDX #$02; ASL $45, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write((some_memory_location + x_register_value) as u16, value_to_be_written_at_some_memory_location);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x0e_asl_absolute_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let lsb_some_memory_location = 0x33u8;
  let msb_some_memory_location = 0x55u8;
  let absolute_some_memory_location = 0x5533u16;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0x0E, lsb_some_memory_location, msb_some_memory_location, 0x00]; // ASL $5533;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_some_memory_location, value_to_be_written_at_some_memory_location as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

#[test]
fn test_0x1e_asl_zeropage_x_mode_should_shift_left_contents_of_accumulator() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let x_register_value = 0x02u8;
  let value_to_be_written_at_some_memory_location = 0x22u8; // 34(10)
  let lsb_some_memory_location = 0x33u8;
  let msb_some_memory_location = 0x55u8;
  let absolute_some_memory_location = 0x5533u16;
  let expected_accumulator_value = 0x44u8; // 68(10)
  let program = vec![0xA2, x_register_value, 0x1E, lsb_some_memory_location, msb_some_memory_location, 0x00]; // LDX #$02; ASL $5544, X;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write_u16(absolute_some_memory_location + x_register_value as u16, value_to_be_written_at_some_memory_location as u16);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.accumulator, expected_accumulator_value);
}

// --------------- LDA --------------------

#[test]
fn test_0xa9_lda_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let program = vec![0xA9, 0x10, 0x00]; // LDA #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xa9_lda_immediate_mode_should_get_instruction_and_brk_and_set_zero_status_flag() {
  // arrange
  let expected_status_flags = 0b0000_0010;
  let program = vec![0xA9, 0x00, 0x00]; // LDA #$00 ; BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
}

#[test]
fn test_0xa5_lda_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA5, 0x33, 0x00]; // LDA $33; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xa5_lda_zeropage_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x22;
  cpu.mem_write(0x33, expected_value);
  let program = vec![0xA5, 0x33, 0x00]; // LDA $33; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xb5_lda_zeropage_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB5, 0x33, 0x00]; // LDA $33, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb5_lda_zeropage_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x33, expected_value);
  let program = vec![0xB5, 0x33, 0x00]; // LDA $33, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xad_lda_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xAD, 0x33, 0xC4, 0x00]; // LDA $C433; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xad_lda_absolute_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xAD, 0x33, 0xC4, 0x00]; // LDA $C433; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xbd_lda_absolute_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xBD, 0x33, 0xC4, 0x00]; // LDA $C433, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xbd_lda_absolute_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xBD, 0x33, 0xC4, 0x00]; // LDA $C433, X; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xb9_lda_absolute_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB9, 0x33, 0xC4, 0x00]; // LDA $C433, Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb9_lda_absolute_y_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write_u16(0xC433, expected_value);
  let program = vec![0xB9, 0x33, 0xC4, 0x00]; // LDA $C433, Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value as u8);
}

#[test]
fn test_0xa1_lda_indirect_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA1, 0x33, 0x00]; // LDA ($33, X); BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xa1_lda_indirect_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x00, expected_value);
  let program = vec![0xA1, 0x33, 0x00]; // LDA ($33, X); BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

#[test]
fn test_0xb1_lda_indirect_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xB1, 0x33, 0x00]; // LDA ($33), Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0010); // zero status flag set
  assert_eq!(cpu.accumulator, 0x00);
}

#[test]
fn test_0xb1_lda_indirect_x_mode_should_get_status_and_value_properly() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x44;
  cpu.mem_write(0x00, expected_value);
  let program = vec![0xB1, 0x33, 0x00]; // LDA ($33), Y; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.accumulator, expected_value);
}

// --------------- LDX --------------------
#[test]
fn test_0xa2_ldx_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x10;
  let program = vec![0xA2, expected_value, 0x00]; // LDX #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xa6_ldx_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xA6, addr, 0x00]; // LDX $10  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xb6_ldx_zeropage_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xB6, addr, 0x00]; // LDX $10, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xae_ldx_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAE, addr_lsb, addr_msb, 0x00]; // LDX $0307;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xae_ldx_absolute_y_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAE, addr_lsb, addr_msb, 0x00]; // LDX $0307, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_x, expected_value);
}

// --------------- LDY --------------------
#[test]
fn test_0xa0_ldy_immediate_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let expected_value = 0x10;
  let program = vec![0xA0, expected_value, 0x00]; // LDY #$10  BRK
  let mut cpu = CPU::new();

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xa4_ldy_zeropage_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xA4, addr, 0x00]; // LDY $10  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xb4_ldy_zeropage_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr = 0x10u8;
  let expected_value = 0x10;
  let program = vec![0xB4, addr, 0x00]; // LDY $10, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(addr as u16, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xac_ldy_absolute_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xAC, addr_lsb, addr_msb, 0x00]; // LDY $0307;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

#[test]
fn test_0xbc_ldy_absolute_x_mode_should_get_instruction_and_set_status_flags_properly() {
  // arrange
  let expected_status_flags = 0b0000_0000;
  let addr_lsb = 0x07u8;
  let addr_msb = 0x03u8;
  let full_addr = 0x0307u16;
  let expected_value = 0x10;
  let program = vec![0xBC, addr_lsb, addr_msb, 0x00]; // LDY $0307, Y;  BRK
  let mut cpu = CPU::new();
  cpu.mem_write(full_addr, expected_value);

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, expected_status_flags);
  assert_eq!(cpu.register_y, expected_value);
}

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

// ------------------- TAX -------------------
#[test]
fn test_0xaa_tax_should_move_acc_to_x() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x01;
  let program = vec![0xA9, expected_value, 0xAA, 0x00]; // LDA $%01; TAX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.register_x, expected_value);
}

#[test]
fn test_0xe8_inx_should_increment_x_register() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x02;
  let program = vec![0xA9, expected_value, 0xAA, 0xE8, 0x00]; // LDA $%02; TAX; INX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0x00);
  assert_eq!(cpu.register_x, 0x03);
}

#[test]
fn test_0xe8_inx_should_overflow_x_register() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xA9, 0xFF, 0xAA, 0xE8, 0xE8, 0x00]; // LDA $%FF; TAX; INX; INX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0x00);
  assert_eq!(cpu.register_x, 0x01);
}

// Should fail
#[test]
#[should_panic(expected = "OP code ff not found")]
fn test_run_should_panic_unknown_opcode() {
  // arrange
  let mut cpu = CPU::new();
  let program = vec![0xFF]; // wrong opcode

  // act
  cpu.load_and_run(program);

  // assert
}
