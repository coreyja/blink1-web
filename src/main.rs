use std::error::Error;
use std::{boxed::Box, time::Duration};

use blinkrs::{Blinkers, Color, LedNum, Pattern, PatternLine};

fn main() -> Result<(), Box<dyn Error>> {
    let blinkers = Blinkers::new()?;

    println!("Device Count: {}", blinkers.device_count()?);

    blinkers.play_pattern(
        Pattern {
            lines: vec![
                PatternLine {
                    color: Color::Red,
                    duration: Duration::new(1, 0),
                    ledn: LedNum::Led1,
                },
                PatternLine {
                    color: Color::Blue,
                    duration: Duration::new(1, 0),
                    ledn: LedNum::Led2,
                },
                PatternLine {
                    color: Color::Green,
                    duration: Duration::new(2, 0),
                    ledn: LedNum::All,
                },
            ],
        },
        1,
    )?;

    Ok(())
}
