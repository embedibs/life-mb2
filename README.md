# Life-mb2

Ethan Dibble

## Tool Setup

```
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools
cargo install cargo-binutils
cargo install --locked probe-rs-tools
```

## Features

Compiling with `--feature slow` will run the game at 1 frame per second
instead of the normal 10 frames per second.
