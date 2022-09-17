use nes_emulator_rust::cpu::CPU;

fn main() {
  let mut cpu = CPU::new();
  cpu.load_and_run(vec![0x09]);
}
