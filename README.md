# rust-nes

This is an NES emulator written in Rust.

![Super Mario Brothers](https://media.giphy.com/media/h40wc0r5KJvP8YDjhy/giphy.gif)
![Castlevania](https://media.giphy.com/media/SqZ2IYPCeEXZM3GUqk/giphy.gif)

The goal of this project is purely educational to both learn Rust and also learn more about systems programming/writing an emulator. This emulator uses NTSC timing and as of right now only supports mappers 0 and 2.

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