# CHIP-8 RUST EMULATOR 
Yet another CHIP-8 emulator written in RUST programming language.

[![License: WTFPL](https://img.shields.io/badge/License-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)


## Installation
For now you can only compile it yourself using [Rust compiler](https://www.rust-lang.org/learn/get-started).

Clone this repository.
```git
git clone https://github.com/Padrition/chip8
```

Cd into it.
```bash
cd chip8
```

Run cargo run command.
```bash
cargo run
```

Emulator should compile and start.

## Adding games to play
I added some games and a programm to play around with into /assets directory.

If you want to add downloaded or self-made game or programm simply add a game file to /assets directory.
Be shure tho the file has .ch8 extension, otherwise the emulator will ignore the file.

## Contribution
Feel free to contribute and to point out issuse with the emulator or my implementation of things.

Simply open an issue or submit a PR with explanation why you belive it should be done this way.

## License
[WTFPL](COPYING.WTFPL)