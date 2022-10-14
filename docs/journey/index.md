# The journey of writing a NES emulator in Rust - Part I: The CPU

## Introduction
Since I started programming I always wanted to know how an emulator works.
> How can a computer let me play the same game that otherwise would require a console like Game Boy, NES, Playstation and so on?

Whenever I would look into the realms of how to write an emulator, I'd always find it pretty difficulty to even start (and, to be totally honest, it does require many HSS - <i>Hours Sitting Studying</i>), so I'd never do it.

But some time ago, I decided to get into it and understand at least the principles of how it works and how I can write one. This is my journey.

## Methodology
The process of initiating anything requires some bullet points, some planning. We must know what we are aiming for and what we're going to use and how to use it.

The first and foremost is to know the theory behind the emulator and all fields related, such as the ones described in the section below.

Then do some POC-like code that helps us understand the theory behind it better.

Finally, the next step is to implement it in the desired space/language/framework. The key in this part is to divide the work in small chunks and then work on them separately one after another in order to not lose traction and motivation of the project as a whole.

## Theory fundamentals
To start learning about emulators is necessary to learn first about some core principles of Computer Science, as they are inside every computer-like type of system.

### Numbering systems representations
Usually in our day to day work/life we use the `base10 representation` (also known as `decimal system`), which consists in multiplying some number by 10. Here's an example:

<div align="center">
  <image src="../img/base_decimal.jpg" width="550" height="300">
  <div>Decimal representation</div>
</div>

As it's possible to realize from the image above, starting from right as 0, every number is at an index which is an exponent of 10. Then, after solving the exponent, it is multiplied by the number at the current index.

The same happens for `binary (base2)` and `hexadecimal (base16)` systems, but on the place of `10` we use `2` and `16` respectively.

<div align="center">
  <image src="../img/binary_representation.png" width="550" height="250">
  <div>Binary representation</div>
</div>

<div align="center">
  <image src="../img/base_hex.jpg" width="550" height="300">
  <div>Hexadecimal representation</div>
</div>

> One thing to notice is that on the hexadecimal numbering system, the algorisms are numbered as follows: 0123456789ABCDEF, where A=10, B=11, C=12 and so on.

Knowing that, one can see why different number representation systems are important. They can reduce the amount of data required to represent some information.

At the end of the day, everything that the computer can understand are zeros (0) and ones (1). A long stream of them. In a simplistic way all the computer is made of are transistors. Thousands, millions, billions of them.

Just imagine how it would be like to understand what a stream of 100 characters containing only 0's and 1's mean. It would be awful. Because of that, everything related to computer science and the description of them is <b>usually</b> represented using the hexadecimal number system.

### Endianness
> In computing, endianness is the order or sequence of bytes of a word of digital data in computer memory. Endianness is primarily expressed as big-endian or little-endian. A big-endian system stores the most significant byte of a word at the smallest memory address and the least significant byte at the largest. --Wikipedia

From the definition above you can have a grasp of what it is.
Binary numbers have the definition of MSB and LSB: <i>Most Significant Bit</i> and <i>Least Significant Bit</i> respectively.

In the <i>Big Endian</i> notation we have the MSB at the left position and the LSB at the right. In the <i>Little Endian</i> is the contrary. Here's an image to exemplify it better.

<div align="center">
  <image src="../img/endianness.jpg" width="550" height="250">
  <div>Little and Big-Endian notation</div>
</div>

So, for example, if we have the number `0x0A0B0C0D` (in hexadecimal representation), we will store it like:

- this `0x0A0B0C0D`, in Big-Endian; and
- this `0x0D0C0B0A`, in Little-Endian.

At the end of the day, the number is the same, only the way we store (and read) it that differs.

### Bitwise operations
Bitwise operations are called like that because they are operations we perform on bits.

Some of the operations are:
- `AND`: will be 1 only if the number at the same position in two different binaries is 1.
```bash
    0101 (decimal 5)
AND 0011 (decimal 3)
  = 0001 (decimal 1)
```
- `OR`: will be 0 only if the number at the same position in two different binaries is 0.
```bash
   0101 (decimal 5)
OR 0011 (decimal 3)
 = 0111 (decimal 7)
```
- `XOR` (exclusive OR): will be 1 only if the just one number at the same position in two different binaries is 1.
```bash
    0101 (decimal 5)
XOR 0011 (decimal 3)
  = 0110 (decimal 6)
```
- `Left-Shift`: will shift every bit in a number to the left. If you shift left a number for just one position you're actually multiplying it by 2 (binary representation).
```bash
   00010111 (decimal +23) LEFT-SHIFT
=  00101110 (decimal +46)
```
- `Right-Shift`: will shift every bit in a number to the right. If you shift right a number for just one position you're actually dividing it by 2 (binary representation).
```bash
=  00101110 (decimal +46) RIGHT-SHIFT
   00010111 (decimal +23) LEFT-SHIFT
```

### Choosing the system to emulate
An emulator is unique to a console/computer/system. Therefore, if you know how to emulate, for instance, a Playstation console, that does not mean that you know how to emulate a NES one. Every system will be different.

The first thing to do when creating an emulator is to understand how the physical console works.
- What are the components (CPU, PPU, APU, Memory...)?
- How are they distributed?
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
- A 16-bit bus to [address memory](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#address-space).

### Stack Pointer
A stack is an abstract data type widely used in mostly everything. It makes use of the concept of LIFO (<i><b>L</b>ast <b>I</b>n <b>F</b>irst <b>O</b>ut</i>).

<div align="center">
  <image src="../img/stack.jpg" width="550" height="300">
  <div>A stack</div>
</div>

The stack pointer is just an 1-byte address that tells us what is the next free address in the stack.

The addresses reserved for the stack are the ones from `$0100` to `$01FF`.

> One thing to notice is that the stack in the 6502 starts from top addresses to down ones. For example, if we have two values to be inserted into the stack, the first one will be inserted at the address `$01FF` and the second one at `$01FE`. After these instructions, the Stack Pointer will have the value `$FD` indicating that the next free address in the stack is the `$01FD`.

### Flags
The flags are nothing more than 1-bit values that tells us the current state of the operations happening in the CPU. These are them:

<div align="center">
  <image src="../img/flags.png" width="550" height="300">
  <div>CPU Flags</div>
</div>

Each CPU instruction will update (or not) those values depending on the result of the operation.

### Address space
The 6502 has a 16-bit address space going from `$0000` to `$FFFF`. Therefore, it can address 2^16 (65536) memory locations.

This address space can be used to read from cartridges, graphics, audio components and other things.

The addressing in this case means that whenever the CPU reads a value from a part of the address space, it's actually reading from the component which was addressed to that space.

### Instructions and Opcodes
The 6502 CPU has 56 instructions and 151 official opcodes.

<div align="center">
  <a href="https://www.nesdev.org/obelisk-6502-guide/reference.html">
    <image src="../img/opcodes.jpeg" width="550" height="130">
    <div>6502 official instructions</div>
  </a>
</div>

As you can see from the image above, there aren't 151 opcodes. So, from where they come?

Each instruction has a different way of being processed (or, we should say, different `addressing modes`), and each one of those modes for each instruction will have a different opcode.

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


## Development
Following the guidelines from the [Methodology section](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#methodology), the first thing to do is to learn more about how the NES (to be more incisive, the 6502 CPU) works.

When you first start reading it, it looks like a beast (even though it's one of the simplest CPUs out there). Do not be afraid. You can do it. 

> By the way, those motivational words are for myself.

In this part you should be reading about `registers`, `accumulator`, operation codes (AKA `opcodes`), `instructions`, `stack`, `program counter` and everything related to that. It's super important. Do not skip it.

Good information for this part can be found at [NESDev website](https://www.nesdev.org/obelisk-6502-guide/).

As just reading is not enough, one must also act on it. Put it into practice. Therefore, coding the instructions of the 6502 in 6502 assembly language is a good start. You can take a look at [Easy 6502](https://skilldrick.github.io/easy6502/). There you can find some examples and also a nice working playground that will show you the current status of the CPU flags, registers, stack and program counter.
You can also use the `debug` function to go through your program step by step and see how everything is working.

<div align="center">
  <a href="https://skilldrick.github.io/easy6502/">
    <image src="../img/easy6502.png" width="500" height="400">
    <div>Easy 6502 by Nick Morgan</div>
  </a>
</div>

The next step is to actually implement those CPU instructions using the language/framework you want. In this case we're using the Rust language. The reason for that is mostly because <s><i><b>I want</b></i></s> Rust is a fast language, very close to the machine code and also because of practice purposes.

At this point, if you've followed the guidelines, it shouldn't be hard for you to implement it, because the operations will not be that complex (but it will require a lot of coding).

The [address space](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#address-space), for example, will be emulated as an array of size `0xFFFF` using an `u16` type. The `Program Counter`, the [Stack Pointer](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#stack-pointer), and the [Registers](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#cpu-6502) will be `u8` types that will hold the value of the current address in our address space (the value we are reading and evaluating).

Using the same idea, the [flags](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#flags) will also be an `u8` type that will allow us to read its value in binary form to verify which ones are set or clear.

<div align="center">
  <a href="https://github.com/Guilospanck/nes-emulator-rust/blob/main/src/cpu.rs#L34">
    <image src="../img/snap.png" width="550" height="400">
    <div>CPU struct</div>
  </a>
</div>

Here's an example of implementation of the `LDA` (loads some value into the Accumulator) instruction. 
<div align="center">
  <a href="https://github.com/Guilospanck/nes-emulator-rust/blob/main/src/cpu.rs#L416">
    <image src="../img/snap2.png" width="550" height="250">
    <div>Example of implementation of LDA instruction</div>
  </a>
</div>

As you can see, we receive a parameter called `addressing_mode` that tells us which mode we are operating (Absolute, Immediate, ZeroPage, and so on).

Then, when we get the address at which the value exists and read it from the [address space](https://github.com/Guilospanck/nes-emulator-rust/blob/main/docs/journey/index.md#address-space).

Finally, we load it into the accumulator and update the `negative` and `carry` flags appropriately.

We know what to do because we're following the reference for that instruction.

<div align="center">
  <a href="https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA">
    <image src="../img/lda.png" width="550" height="450">
    <div>Reference of LDA instruction</div>
  </a>
</div>

## Conclusion
By now you may know what I meant in the beginning when I told you that writing an emulator requires a lot of studying and knowledge of different related concepts.

The good thing is that when you have a grasp of it, you can really be astonished by how people before us thought about things. How they created many things from dust, out of thin air.

This first part is focused on the CPU component of an emulator. There exists also the ppu, audio, rom, bus and gamepad components, but that is for another time.

I really hope that this basic, unrevised and chaotic article could help you all spark the fire of wanting to get into the development of emulators or low-level concepts in general. 

See you next time!

👉🏻 All the code is available at the [GitHub repository](https://github.com/Guilospanck/nes-emulator-rust).