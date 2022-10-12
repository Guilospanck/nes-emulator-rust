# The journey of writing a NES emulator in Rust

## Summary
Since I started programming I always wanted to know how emulator works.
> How can a computer let me play the same game that otherwise would require a console like Game Boy, NES, Playstation and so on?

Whenever I would look into the realms of how to write an emulator, I've always found it pretty difficulty to even start (and, to be totally honest, it does require some many SSH - Sitting Studying Hours), so I never did it.

But some time ago, I decided to get into it and understand at least the principles of how it works and how I can write it. This is my journey.

## Development
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

Knowing that, one can see why different number representation systems are important. They can reduce the amount of data to represent some information.

At the end of the day, everything that the computer can understand are zeros (0) and ones (1). A long stream of them. In a simplistic way all the computer is made of are of transistors. Thousands, millions, billions of them.

Just imagine what would it be like to understand what a stream of 100 characters containing only 0's and 1's mean. It would be awful. Because of that, everything related to computer science and the description of them is usually represented using the hexadecimal number system.

### Choosing the system to emulate
An emulator is unique to a console/computer/system. Therefore, if you know how to emulate, for instance, a Playstation console, that does not mean that you know how to emulate a NES one.

The first thing to do when creating an emulator is to understand how to the physical console works.
- What are the components (CPU, PPU, APU, Memory...)?
- How they are distributed?
- How are they interconnected?
- What is the capacity of each component?

Therefore, before even dreaming to get into code, you need to delve into the hardware. Because, ultimately, <i>an emulator is software emulating a hardware</i>.

