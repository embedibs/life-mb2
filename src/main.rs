//! Life-mb2
//! Ethan Dibble
//!
//! - The program runs the game at 10 frames per second (updates once per 100ms).
//! - The program starts with a random board.
//! - While the A button is held, the board is re-randomized every frame.
//! - Otherwise, when the B button is not ignored and is pressed, the board is
//!   “complemented”: every “on” cell is turned “off” and every “off” cell is
//!   turned “on”. The B button is then ignored for 5 frames (0.5s).
//! - Otherwise, if the program reaches a state where all cells on the board are
//!   off, the program waits 5 frames (0.5s). If it has not received a button
//!   press, it then starts with a new random board.
//! - Otherwise, normal Life steps are taken.

#![no_main]
#![no_std]

mod life;
use life::*;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{Rng as HwRng, timer::Timer},
};
use nanorand::{Pcg64, Rng, SeedableRng};
use panic_halt as _;

type Buf = [[u8; 5]; 5];

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let fb: &mut Buf = &mut Default::default();
    let rng = &mut nanorand::Pcg64::new_seed(4);

    randomize(fb, rng);

    loop {
        life(fb);
        display.show(&mut timer, *fb, 1000);
        display.clear();
        timer.delay_ms(100);
    }
}

fn randomize(fb: &mut Buf, rng: &mut Pcg64) {
    fb.iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|px| *px = rng.generate::<bool>() as u8);
}
