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
  <div>Decimal representation</div>
</div>

As it's possible to realize from the image above, starting from left as 0, every number is at an index which is an exponent of 10. Then, after solving the exponent, it is multiplied by the number at the current index.

The same happens for `binary (base2)` and `hexadecimal (base16)` systems, but on the place of 10 we use 2 and 16 respectively.

<div align="center">
  <image src="../img/binary_representation.png" width="550" height="250">
  <div>Binary representation</div>
</div>

<div align="center">
  <image src="../img/base_hex.jpg" width="550" height="300">
  <div>Hexadecimal representation</div>
</div>

> One thing to notice is that on the hexadecimal numbering system, the algorisms are numbered as follows: 0123456789ABCDEF.

Knowing that, one can see why different number representation systems are important. They can reduce the amount of data to represent some information.

At the end of the day, everything that the computer can understand are zeros (0) and ones (1). A long stream of them. In a simplistic way all the computer is made of are of transistors. Thousands, millions, billions of them.

Just imagine what would it be like to understand what a stream of 100 characters containing only 0's and 1's mean. It would be awful. Because of that, everything related to computer science and the description of them is usually represented using the hexadecimal number system.

### Endianness
> In computing, endianness is the order or sequence of bytes of a word of digital data in computer memory. Endianness is primarily expressed as big-endian or little-endian. A big-endian system stores the most significant byte of a word at the smallest memory address and the least significant byte at the largest. --Wikipedia

From the definition above you can have a grasp of what it is.
Binary numbers have the definition of MSB and LSB: <i>Most Significant Bit</i> and <i>Least Significant Bit</i> respectively.

In the <i>Big Endian</i> notation we have the MSB at the right position and the LSB at the left. In the <i>Little Endian</i> is the contrary. Here's an image to exemplify it better.

<div align="center">
  <image src="../img/endianness.jpg" width="550" height="250">
  <div>Little and Big-Endian notation</div>
</div>

So, for example, if we have the number `0x0A0B0C0D` (in hexadecimal representation), we will store it like:

- this `0x0A0B0C0D`, in Big-Endian; and
- this `0x0D0C0B0A`, in Little-Endian.

At the end of the day, the number is the same, only the way we store (and read) it that differs.

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
  <div>NES</div>
</div>

For its CPU it uses a version of the MOS Technology 6502.

### CPU 6502
A CPU (<i>Central Processing Unit</i>) works by executing machine instructions. Each instruction starts with an operation code (`opcode`), defining the particular operation that must be executed.
The speed with which the CPU processes the instructions is ruled by the clock.

The 6502 CPU version used by the NES it's fairly simple.
It is an 8-bit CPU, which means that it has an 8-bit data bus. Therefore, this CPU can only process 8-bit (1 byte) instruction per clock and, if you want to process more than one, you'll have to combine them.

Its architecture consists of:
- Three registers: `Accumulator`, `X` and `Y`, each one with 1 byte, used to perform mathematical operations and what-not;
- One `status` register to store our operation [flags](https://github.com/Guilospanck/nes-emulator-rust#flags);
- A [stack pointer](https://github.com/Guilospanck/nes-emulator-rust#stack-pointer) that shows what is the next free address to use the stack;
- A 16-bit bus to address memory.

### Stack Pointer
A stack is an abstract data type widely used in mostly everything. It makes use of the concept of LIFO (<i><b>L</b>ast <b>I</b>n <b>F</b>irst <b>O</b>ut</i>).

<div align="center">
  <image src="../img/stack.jpg" width="550" height="300">
  <div>A stack</div>
</div>

The stack pointer is just an 1-byte address that tells us what is the next free address in the stack.

The addresses reserved for the stack are the ones from `$0100` to `$01FF`.

### Flags
The flags are nothing more than 1-bit values that tells us the current state of the operations happening in the CPU. These are them:

<div align="center">
  <image src="../img/flags.png" width="550" height="300">
  <div>CPU Flags</div>
</div>

Each CPU instruction will update (or not) those values depending on the result of the operation.

### Instructions and Opcodes
The 6502 CPU has 56 instructions and 151 official opcodes.

<div align="center">
  <a href="https://www.nesdev.org/obelisk-6502-guide/reference.html">
    <image src="../img/opcodes.jpeg" width="550" height="130">
    <div>6502 official instructions</div>
  </a>
</div>

As you can see from the image above, there aren't 151 opcodes. So, from where they come?

Each instruction has a different way of being processed (or, we should say, different `addressing modes`).

### Addressing Modes
The addressing modes 6502 have are:

```rs
pub enum AddressingMode {
  Accumulator,
  Relative,
  Immediate,
  ZeroPage,
  ZeroPageX,
  ZeroPageY,
  Absolute,
  AbsoluteX, // Absolute, X
  AbsoluteY, // Absolute, Y
  IndirectX, // (Indirect, X)
  IndirectY, // (Indirect), Y
  NoneAddressing,
}
```

For each one of them we'll have a different `opcode` for a different instruction.

Basically (and in a simplified way) the addressing modes work as follows:

- `Accumulator`: acts directly on the Accumulator;
- `Relative`: commonly used for branching instructions. It'll branch to another place on the <i>Program Counter</i> relative to where it is at the moment;
- `Immediate`: using this addressing mode will not load the contents of some address, but the content itself. Example: `LDA #$04` will load the value `0x04` into the Accumulator register;
- `ZeroPage`: gives us the second byte of the address formed by `$00{}`. In other words, if the address is `$E9` then the absolute address will be `$00E9`;
- `ZeroPageX`: works as `ZeroPage`, but before getting the contents of the ZeroPage address, it'll add to it the value contained in the `X register`. Example: `LDX #$03; LDA $06, X` will get the contents from the address `$0009` (0x06 + 0x03 = 0x09);
- `ZeroPageY`: the same as above, but with the `Y Register`;
- `Absolute`: loads the contents of some absolute address. Example: `LDA $03ED` will load the contents of the address `$03ED` to the Accumulator register;
- `AbsoluteX`: the same as `ZeroPageX` but with absolute address;
- `AbsoluteY`: the same as `ZeroPageY` but with absolute address;
- `IndirectX`: this and the next one are a bit tricky, but bear with me. Let's understand this one using an example.
  Let's say that we have:

    - at the address `$0003` the value `0x05`; (a)
    - at the address `$0004` the value `0x07`; (b)
    - the X Register has the value `0x01`;     (c)
    - at the address `$0705` the value `0xAD`. (d)

  Then we want to process the following instruction: `LDA ($02, X)` which uses the IndirectX addressing mode.

  This is what will happen:
    - First we get the `ZeroPageX` out of it: `$02 + x => $02 + $01 = $03`;
    - Now we get the absolute address that begins at the address `$0003`, which from `(a)` we know it's `0x05` and from `(b)` we know it's `0x07`. Then, knowing that the CPU uses Little-Endian notation, we have the absolute address `$0705`;
    - We will now load into the Accumulator the contents that exists at the address `$0705`, which from `(d)` we know it's `0xAD`.

- `IndirectY`: this one follows a line that it looks like the previous one, but with some minor differences. Let's us understand it using an example:
  Let's say that we have:

    - at the address `$0003` the value `0x05`; (a)
    - at the address `$0004` the value `0x07`; (b)
    - the Y Register has the value `0x01`;     (c)
    - at the address `$0706` the value `0xAD`. (d)

  Then we want to process the following instruction: `LDA ($03), Y` which uses the IndirectY addressing mode.

  This is what will happen:
    - First we get the `ZeroPage` out of it: `$0003`;
    - Now we get the absolute address that begins at the address `$0003`, which from `(a)` we know it's `0x05` and from `(b)` we know it's `0x07`. Then, knowing that the CPU uses Little-Endian notation, we have the absolute address `$0705`;
    - Add to that absolute address the value of the Y Register: `$0705 + $0001 = $0706`;
    - We will now load into the Accumulator the contents that exists at the address `$0706`, which from `(d)` we know it's `0xAD`.

- `NoneAddressing`: this is used for the instructions that don\`t require any other option, like `BRK`, `CLC`, `DEX`, `INX` and so on.