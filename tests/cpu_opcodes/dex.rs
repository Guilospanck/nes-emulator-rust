use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0xca_dex_should_decrement_x_register_by_one() {
  // arrange
  let mut cpu = CPU::new();
  let x_register_value = 0x13;
  let expected_value = 0x10;
  let program = vec![0xA2, x_register_value, 0xCA, 0xCA, 0xCA, 0x00]; // LDX #$10; DEX; DEX; DEX; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0b0000_0000);
  assert_eq!(cpu.register_x, expected_value);
}
