use nes_emulator_rust::cpu::CPU;
use nes_emulator_rust::games;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas};
use sdl2::video::Window;
use sdl2::{EventPump};

fn color(byte: u8) -> Color {
  match byte {
    0 => Color::BLACK,
    1 => Color::WHITE,
    2 | 9 => Color::GREY,
    3 | 10 => Color::RED,
    4 | 11 => Color::GREEN,
    5 | 12 => Color::BLUE,
    6 | 13 => Color::MAGENTA,
    7 | 14 => Color::YELLOW,
    _ => Color::CYAN,
  }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
  let mut frame_idx = 0;
  let mut update = false;
  for addr in 0x0200..0x0600 {
    // get the byte of color at the address
    let color_idx = cpu.mem_read_u16(addr);
    // and translates it into a Color
    let (b1, b2, b3) = color(color_idx as u8).rgb();

    if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3 {
      frame[frame_idx] = b1;
      frame[frame_idx + 1] = b2;
      frame[frame_idx + 2] = b3;
      update = true;
    }

    frame_idx += 3;
  }

  update
}

/// This function will run at every loop routine (after every instruction) and will expect some key press event
/// from the user.
/// It writes to the 0xFF memory (which is where we're gathering our user inputs) the
/// ASCII code of the key (WASD) pressed.
fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit { .. }
      | Event::KeyDown {
        keycode: Some(Keycode::Escape),
        ..
      } => std::process::exit(0),
      Event::KeyDown {
        keycode: Some(Keycode::W),
        ..
      } => {
        cpu.mem_write(0xFF, 0x77);
      }
      Event::KeyDown {
        keycode: Some(Keycode::S),
        ..
      } => {
        cpu.mem_write(0xFF, 0x73);
      }
      Event::KeyDown {
        keycode: Some(Keycode::A),
        ..
      } => {
        cpu.mem_write(0xFF, 0x61);
      }
      Event::KeyDown {
        keycode: Some(Keycode::D),
        ..
      } => {
        cpu.mem_write(0xFF, 0x64);
      }
      _ => { /* do nothing */ }
    }
  }
}

fn init_sdl2() -> (Canvas<Window>, EventPump) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window("Snake Game", (32.0 * 10.0) as u32, (32.0 * 10.0) as u32)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
  let event_pump = sdl_context.event_pump().unwrap();
  canvas.set_scale(10.0, 10.0).unwrap();

  (canvas, event_pump)
}

fn main() {
  let (mut canvas, mut event_pump) = init_sdl2();

  let creator = canvas.texture_creator();
  let mut texture = creator
    .create_texture_target(PixelFormatEnum::RGB24, 32, 32)
    .unwrap();

  let mut screen_state = [0u8; 32 * 3 * 32];
  let mut rng = rand::thread_rng();

  let mut cpu = CPU::new();
  let snake_game_code: &Vec<u8> = &(*games::snake::SNAKE_GAME_CODE);
  // let snake_game_code: &Vec<u8> = &(*games::example::SNAKE_GAME_CODE); // example
  cpu.load(snake_game_code.to_vec());
  cpu.reset();
  cpu.run_with_callback(move |cpu| {
    handle_user_input(cpu, &mut event_pump);
    cpu.mem_write(0xFE, rng.gen_range(1..16)); // writes random value to the byte responsible for randomic things

    if read_screen_state(cpu, &mut screen_state) { // if something changed since last iteration
      texture.update(None, &screen_state, 32 * 3).unwrap();
      canvas.copy(&texture, None, None).unwrap();
      canvas.present();
    }

    ::std::thread::sleep(std::time::Duration::new(0, 70_000));

  });
}
