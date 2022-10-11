# The journey of writing a NES emulator in Rust

## Summary
Since I started programming I always wanted to know how emulator works.
> How can a computer let me play the same game that otherwise would require a console like Game Boy, NES, Playstation and so on?

Whenever I would look into the realms of how to write an emulator, I've always found it pretty difficulty to even start (and, to be totally honest, it does require some many SSH - Sitting Studying Hours), so I never did it.

But some time ago, I decided to get into it and understand at least the principles of how it works and how I can write it. This is my journey.

## Development
To start learning about how an emulator works is necessary to learn first about numbering systems representations.

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
