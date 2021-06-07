# Rusty Chip-8 emulator

Chip-8 emulator written in Rust

![Screenshot](screenshots/space_invaders_intro_screen.png)

## Keyboard Mapping

<table>
<tr><th>Chip-8 Keyboard </th><th>Standard Keyboard</th></tr>
<tr><td>
|    |    |    |    |
|:--:|:--:|:--:|:--:|
|1   |2   |3   |C   |
|4   |5   |6   |D   |
|7   |8   |9   |E   |
|A   |0   |B   |F   |

</td><td>
|    |    |    |    |
|:--:|:--:|:--:|:--:|
|1   |2   |3   |4   |
|Q   |W   |E   |R   |
|A   |S   |D   |F   |
|Y   |X   |C   |V   |

</td></tr> 
</table>
## Test Roms

Two roms can be used to check if graphics are rendered correctly.

- test_rendering.ch8: Test conditional jumps, mathematical and logical operations of the chip8 ([more detailed explanation](https://slack-files.com/T3CH37TNX-F3RKEUKL4-b05ab4930d)).
- test_opcode.ch8: Test operations (read, write, compare etc.) on chip8 registers ([more detailed explanation](https://github.com/corax89/chip8-test-rom))

<b> Sreenshot when test_rendering is successful <b>

![Screenshot](screenshots/test_chip8_rendering_screen.png)

<b> Screenshot when test_opcode is successful <b>

![Screenshot](screenshots/test_opcode_correct_screen.png)

## Resources

- [Cowgod's Chip-8 Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [How to write an emulator](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Mastering Chip-8](https://github.com/mattmikolay/chip-8/wiki/Mastering-CHIP-8)
- [Chip8 emulator with code snippets in C++](https://austinmorlan.com/posts/chip8_emulator/)
