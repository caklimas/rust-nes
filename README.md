# rust-nes

This is an NES emulator written in Rust.

![Super Mario Brothers](demo/Super%20Mario%20Brothers.gif)
![Castlevania](demo/Castlevania.gif)

The goal of this project is purely educational to both learn Rust and also learn more about systems programming/writing an emulator. This emulator uses NTSC timing.

The emulator supports the following mappers:
- [Mapper_000](https://wiki.nesdev.com/w/index.php/NROM)
- [Mapper_002](https://wiki.nesdev.com/w/index.php/UxROM)
- [Mapper_003](https://wiki.nesdev.com/w/index.php/INES_Mapper_003)
- [Mapper_066](https://wiki.nesdev.com/w/index.php/GxROM)

## Getting Started
### Install Rust
To run this project, Rust needs to be installed:
[Download Rust](https://www.rust-lang.org/tools/install)

### Clone repository:
```
git clone https://github.com/caklimas/rust-nes.git
```

### Build project(if you don't build under release the program will run very slowly):
```
cargo build --release
```

### Run executable
```
Navigate to /target/release folder then execute the following:
rust-nes.exe {path-to-rom}

Ex: rust-nes.exe "C:\ROMS\Super Mario Bros. (World).nes"
```

### Controls
This is the keyboard mapping from the NES Controller:
| NES    | Keyboard
| -------| --------
|   A    |    Z   |
|   B    |    X   |
| Start  |  Enter |
| Select | R-Shift|
|   Up   |   Up   |
|  Down  |  Down  |
|  Left  |  Left  |
|  Right |  Right |