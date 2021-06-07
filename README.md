# Rusty Chip-8 emulator (WIP)

Chip-8 emulator written in Rust

![Screenshot](screenshots/space_invaders_intro_screen.png)

## Keyboard Mapping

<table>
  <tr><th>Chip-8 Keyboard </th><th></th><th>Standard Keyboard</th></tr>
<tr><td>
  
<table>
  <tr> <td> 1 </td><td> 2 </td><td> 3 </td><td> C </td> </tr>
  <tr> <td> 4 </td><td> 5 </td><td> 6 </td><td> D </td> </tr>
  <tr> <td> 7 </td><td> 8 </td><td> 9 </td><td> E </td> </tr>
  <tr> <td> A </td><td> 0 </td><td> B </td><td> F </td> </tr>
</table>
    
</td>
  <td> => </td>
<td>
  
<table>
  <tr> <td> 1 </td><td> 2 </td><td> 3 </td><td> 4 </td> </tr>
  <tr> <td> Q </td><td> W </td><td> E </td><td> R </td> </tr>
  <tr> <td> A </td><td> S </td><td> D </td><td> F </td> </tr>
  <tr> <td> Y </td><td> X </td><td> C </td><td> V </td> </tr>
</table>
    
</td>
  
</tr>   
</table>

## Run chip-8 emulator

... Work in Progress

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
