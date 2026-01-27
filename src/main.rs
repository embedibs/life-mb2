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
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use nanorand::{Pcg64, Rng};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

type Buf = [[u8; 5]; 5];

struct Counter(u8);

impl Counter {
    fn set(&mut self, n: u8) {
        self.0 = n;
    }

    fn dec(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    fn is_zero(&self) -> bool {
        match self.0 {
            0 => true,
            _ => false,
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let fb: &mut Buf = &mut Default::default();
    let rng = &mut nanorand::Pcg64::new_seed(5);

    let mut c_button_b = Counter(5);
    let mut c_reset = Counter(5);

    randomize(fb, rng);

    loop {
        // Need to push pretty hard on the buttons.
        let a_pressed = button_a.is_low().unwrap();
        let b_pressed = button_b.is_low().unwrap();

        match (a_pressed, b_pressed) {
            (true, _) => {
                rprintln!("Button A pressed");
                randomize(fb, rng);

                c_reset.set(5);
            }
            (_, true) if c_button_b.is_zero() => {
                rprintln!("Button B pressed");
                invert(fb);

                c_button_b.set(5);
                c_reset.set(5);
            }
            _ if done(fb) && c_reset.is_zero() => {
                rprintln!("Resetting board");
                randomize(fb, rng);
            }
            _ => (),
        }

        life(fb);
        display.show(&mut timer, *fb, 1000);

        c_button_b.dec();
        c_reset.dec();

        timer.delay_ms(100);
        display.clear();
    }
}

fn randomize(fb: &mut Buf, rng: &mut Pcg64) {
    fb.iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|px| *px = rng.generate::<bool>() as u8);
}

fn invert(fb: &mut Buf) {
    fb.iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|px| *px ^= 1)
}
