# The journey of writing a NES emulator in Rust

## Introduction
Since I started programming I always wanted to know how emulator works.
> How can a computer let me play the same game that otherwise would require a console like Game Boy, NES, Playstation and so on?

Whenever I would look into the realms of how to write an emulator, I've always found it pretty difficulty to even start (and, to be totally honest, it does require some many SSH - Sitting Studying Hours), so I never did it.

But some time ago, I decided to get into it and understand at least the principles of how it works and how I can write it. This is my journey.

## Theoric Fundaments
To start learning about how an emulator works is necessary to learn first about numbering systems representations, as they are in the core of every computer-like type of system.

### Numbering systems representations
Usually we in our day to day work use the `base10 representation` (also known as `decimal system`), which consists in multiplying some number by 10. Here's an example:

<div align="center">
  <image src="../img/base_decimal.jpg" width="550" height="300">
  <caption>Decimal representation</caption>
</div>

As it's possible to realize from the image above, starting from left as 0, every number is at an index which is an exponent of 10. Then, after solving the exponent, it is multiplied by the number at the current index.

The same happens for `binary (base2)` and `hexadecimal (base16)` systems, but on the place of 10 we use 2 and 16 respectively.

<div align="center">
  <image src="../img/binary_representation.png" width="550" height="300">
  <caption>Binary representation</caption>
</div>

<div align="center">
  <image src="../img/base_hex.jpg" width="550" height="300">
  <caption>Hexadecimal representation</caption>
</div>

> One thing to notice is that on the hexadecimal numbering system, the algorisms are numbered as follows: 0123456789ABCDEF.

Knowing that, one can see why different number representation systems are important. They can reduce the amount of data to represent some information.

At the end of the day, everything that the computer can understand are zeros (0) and ones (1). A long stream of them. In a simplistic way all the computer is made of are of transistors. Thousands, millions, billions of them.

Just imagine what would it be like to understand what a stream of 100 characters containing only 0's and 1's mean. It would be awful. Because of that, everything related to computer science and the description of them is usually represented using the hexadecimal number system.

### Choosing the system to emulate
An emulator is unique to a console/computer/system. Therefore, if you know how to emulate, for instance, a Playstation console, that does not mean that you know how to emulate a NES one. Every system will be different.

The first thing to do when creating an emulator is to understand how to the physical console works.
- What are the components (CPU, PPU, APU, Memory...)?
- How they are distributed?
- How are they interconnected?
- What is the capacity of each component?

Therefore, before even dreaming to get into code, you need to delve into the hardware. Because, ultimately, <i>an emulator is software acting as hardware</i>.

### NES
The NES (<i>Nintendo Entertainment System</i>) was a game console produced by Nintendo in the 80's. It was a huge success and remains well known for enthusiasts.

<div align="center">
  <image src="../img/nes.jpg" width="550" height="300">
  <caption>NES</caption>
</div>

For its CPU it uses a version of the MOS Technology 6502.

### CPU 6502
A CPU (<i>Central Processing Unit</i>) works by executing machine instructions. Each instruction starts with an operation code (`opcode`), defining the particular operation that must be executed.
The speed with which the CPU processes the instructions is ruled by the clock.

The 6502 CPU version used by the NES it's fairly simple.
It is an 8-bit CPU, which means that it has an 8-bit data bus. Therefore, this CPU can only process 8-bit (1 byte) instruction per clock and, if you want to process more than one, you'll have to combine them.

### Opcodes
The 6502 CPU has 56 instructions and 151 official opcodes.

<div align="center">
  <image src="../img/opcodes.jpeg" width="550" height="150">
  <caption>6502 official instructions</caption>
</div>

As you can see from the image above, there aren't 151 opcodes. So, from where they come?

Each instruction has a different way of being processed (or, we should say, different `addressing modes`).

### Addressing Modes