use nes_emulator_rust::cpu::CPU;

#[test]
fn test_0x8a_txa_should_move_acc_to_x() {
  // arrange
  let mut cpu = CPU::new();
  let expected_value = 0x01;
  let program = vec![0xA2, expected_value, 0x8A, 0x00]; // LDX #$01; TXA; BRK

  // act
  cpu.load_and_run(program);

  // assert
  assert_eq!(cpu.status, 0);
  assert_eq!(cpu.accumulator, expected_value);
  assert_eq!(cpu.register_x, expected_value);
}
