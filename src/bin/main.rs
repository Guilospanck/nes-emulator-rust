use nes_emulator_rust::cpu::CPU;
use nes_emulator_rust::games;

fn main() {
  let mut cpu = CPU::new();
  let snake_game_code: &Vec<u8> = &(*games::snake::SNAKE_GAME_CODE);
  cpu.load(snake_game_code.to_vec());
  cpu.reset();
  cpu.run_with_callback(move |_cpu| {
    println!("potato");
  });
}
