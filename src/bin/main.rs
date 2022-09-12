use nes_emulator_rust::cpu::CPU;

fn main() {
  let mut cpu = CPU::new();
  cpu.interpret(vec![0x09]);
}
