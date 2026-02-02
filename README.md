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

## Game Flow

- Pressing the A button will randomize the board.

- Pressing the B button will invert the board and the B button will then be
  ignored for 5 frames.

  > This option is not very interesting as the
  > cells often die off immediately due to overpopulation.

- If the board is empty, the sim waits 5 frames, and if no button is pressed,
  the board is randomized.

## Crate Features

- Compiling with `--feature slow` will run the game at 1 frame per second
instead of the normal 10 frames per second.

## Notes

Button presses are polled every frame, which works fine since counters are
measured in frames. And the 100ms frame rate is not too fast for polling. You
have to push the buttons on this board pretty hard though.  

There are two counters: the B button and reset counters. Both are measured in
frames, but are set and decremented under different conditions. Pressing the B
button will set the associated counter and it will decrement every frame.
Pressing any button will set the reset counter and it will decrement when the
board is empty. These counters simply amount to additional match guards to check
if they are zero.  

## License

This project is licensed under the [MIT License][License].

[License]: ./LICENSE
