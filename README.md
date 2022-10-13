# NES Emulator Rust
A simple NES Emulator written in Rust.

[![codecov](https://codecov.io/gh/Guilospanck/nes-emulator-rust/branch/main/graph/badge.svg?token=7VJXXU7UFJ)](https://codecov.io/gh/Guilospanck/nes-emulator-rust)

## Goal
Have a simple, but working, NES emulator written in rust.

## Current project
As writing an emulator (when you've never done it) takes a lot of time, currently this one only emulates some `opcodes` necessary to run a basic `Snake Game`. In other words, it's only focused on the CPU part of it.

<div align="center">
  <image src="./docs/img/snake.jpeg" width="400" height="400">
</div>

You can play it by using the WASD keys.

## Installation

### Rust
Be sure to have `Rust` installed in your system. You can do that by following the instructions of [this link](https://www.rust-lang.org/tools/install).

### SDL2
This emulator is using the Rust crate for `SDL2` ([rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2)). SDL (<i>Simple DirectMedia Layer</i>) is a library that provides low level access to many resources, like graphics hardware, keyboard, mouse, joystick and so on.

To install it on MacOS, do the following:
```bash
brew install sdl2
```
```bash
brew link sdl2 
```
Then, edit `.zshenv` (and not `.zshrc`, if you are using ZSH, of course) and add:
```bash
export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/lib"
```

### Project
Git clone this repository:
```bash
git clone https://github.com/Guilospanck/nes-emulator-rust.git
```

## How to run
After the process of installation above, change directory into the project folder:

```bash
cd nes-emulator-rust/
```

And then run:

```bash
cargo run .
```

## Useful Links
Here are some of the links that were (are being) used to get to this point of the project. Be sure to check them out the have more context.
- [Introduction to 6502](https://skilldrick.github.io/easy6502/index.html#intro)

- [NES Emulator in Rust](https://bugzmanov.github.io/nes_ebook/chapter_3_1.html)

- [6502 Reference Guide](https://www.nesdev.org/obelisk-6502-guide/)