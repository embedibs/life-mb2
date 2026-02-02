# Life-mb2

Conway's Game of Life for the BBC Micro:Bit V2.

> Ethan Dibble  
> January 2026

## Tool Setup

```
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools
cargo install cargo-binutils
cargo install --locked probe-rs-tools
```

## Build and Run

```
cargo embed
```

## Features

Compiling with `--feature slow` will run the game at 1 frame per second
instead of the normal 10 frames per second.


## License

This project is licensed under the [MIT License][License].

[License]: ./LICENSE
