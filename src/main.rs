use std::error::Error;
use std::{boxed::Box, time::Duration};

use blinkrs::{Blinkers, Color, LedNum, Message};

fn main() -> Result<(), Box<dyn Error>> {
    let blinkers = Blinkers::new()?;

    println!("Device Count: {}", blinkers.device_count()?);
    // blinkers.send(Message::from("red"))?;
    // blinkers.send(Message::Fade(Color::Red, Duration::new(0, 0), LedNum::All))?;
    // blinkers.send(Message::Fade(
    //     Color::Green,
    //     Duration::new(2, 0),
    //     LedNum::Led1,
    // ))?;
    // blinkers.send(Message::Fade(
    //     Color::Blue,
    //     Duration::new(2, 0),
    //     LedNum::Led2,
    // ))?;
    // blinkers.send(Message::from("off"))?;
    blinkers.send(Message::SetLedNum(LedNum::Led1))?;
    blinkers.send(Message::SetLinePattern(Color::Red, Duration::new(2, 0), 0))?;
    blinkers.send(Message::SetLedNum(LedNum::Led2))?;
    blinkers.send(Message::SetLinePattern(Color::Blue, Duration::new(2, 0), 1))?;
    blinkers.send(Message::SetLedNum(LedNum::All))?;
    blinkers.send(Message::SetLinePattern(
        Color::Green,
        Duration::new(2, 0),
        2,
    ))?;
    blinkers.send(Message::PlayLoop {
        on: true,
        start_pos: 0,
        end_pos: 2,
        loop_count: 0,
    })?;
    Ok(())
}
