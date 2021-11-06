use std::time::Duration;

use blinkrs::{Blinkers, Color, LedNum, Pattern, PatternLine};

use serde::Deserialize;

use color_eyre::eyre::Result;

use tracing::{info, instrument};

use eyre::{eyre, WrapErr};

#[derive(Deserialize, Debug)]
struct FrameSnake {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Color")]
    color: String,
}

#[derive(Deserialize, Debug)]
struct Frame {
    #[serde(rename = "Snakes")]
    snakes: Vec<FrameSnake>,
}

#[derive(Deserialize, Debug)]
struct BattlesnakeEngineGameResponse {
    #[serde(rename = "LastFrame")]
    last_frame: Frame,
}

#[instrument]
fn color_from_string(s: &str) -> Result<Color> {
    if s.len() != 7 {
        return Err(eyre!("Wrong length for color"));
    }

    let chars: Vec<_> = s.chars().collect();

    if chars[0] != '#' {
        return Err(eyre!("Color format is wrong"));
    }

    fn u8_from_slice(chars: &[char]) -> Result<u8> {
        let s: String = chars.iter().collect();

        u8::from_str_radix(&s, 16).wrap_err("Couldn't parse a u8 from the given string")
    }

    let red = u8_from_slice(&chars[1..=2])?;
    let green = u8_from_slice(&chars[3..=4])?;
    let blue = u8_from_slice(&chars[5..=6])?;

    Ok(Color::Three(red, green, blue))
}

struct Snake {
    color: Color,
}

fn play_leds_for_game(game_id: &str) -> Result<()> {
    let json: BattlesnakeEngineGameResponse =
        ureq::get(&format!("https://engine.battlesnake.com/games/{}", game_id))
            .call()?
            .into_json()?;
    info!(?json, "Got response from engine");

    let snakes: Vec<Snake> = json
        .last_frame
        .snakes
        .into_iter()
        .map(|s| {
            Ok(Snake {
                color: color_from_string(&s.color)?,
            })
        })
        .collect::<Result<_>>()?;

    let lines = snakes
        .iter()
        .map(|s| {
            let p1 = PatternLine {
                color: s.color,
                duration: Duration::new(1, 0),
                ledn: LedNum::Led1,
            };
            let p2 = PatternLine {
                color: s.color,
                duration: Duration::new(1, 0),
                ledn: LedNum::Led2,
            };

            vec![p1, p2, p2]
        })
        .flatten()
        .collect::<Vec<_>>();
    let pattern = Pattern { lines };

    let blinkers = Blinkers::new()?;

    println!("Device Count: {}", blinkers.device_count()?);

    blinkers.play_pattern(pattern, 0)?;

    Ok(())
}

#[macro_use]
extern crate rocket;

use rocket::{
    http::Status,
    response::{status::Created, Debug},
};

#[get("/game/<game_id>/start")]
fn start_game(game_id: &str) -> Result<Status, Debug<eyre::ErrReport>> {
    play_leds_for_game(game_id).with_context(|| "Things went badly")?;

    Ok(Status::Ok)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![start_game])
}
