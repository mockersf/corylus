use std::io::{self, BufRead};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct CssSprite {
    width: String,
    height: String,
    background: String,
}

#[derive(Serialize, Debug)]
struct Sprite {
    width: u16,
    height: u16,
    x: u16,
    y: u16,
}

#[derive(Serialize, Debug)]
struct SpriteSheet {
    texture_width: u16,
    texture_height: u16,
    sprites: Vec<Sprite>,
}

fn px_to_u16(px: &str) -> Result<u16, std::num::ParseIntError> {
    let ln = px.len();
    let nb = &px[..(ln - 2)];
    nb.parse::<u16>()
}

fn background_to_pos(background: &str) -> Result<(u16, u16), std::num::ParseIntError> {
    let items = background.split(' ').collect::<Vec<_>>();
    Ok((px_to_u16(&items[1][1..])?, px_to_u16(&items[2][1..])?))
}

fn main() -> Result<(), std::io::Error> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        let sprites = serde_json::from_str::<std::collections::HashMap<String, CssSprite>>(&line)?
            .iter()
            .map(|sprite| {
                let pos = background_to_pos(&sprite.1.background).unwrap();
                Sprite {
                    width: px_to_u16(&sprite.1.width).unwrap(),
                    height: px_to_u16(&sprite.1.height).unwrap(),
                    x: pos.0,
                    y: pos.1,
                }
            })
            .collect::<Vec<_>>();
        let texture_width = sprites
            .iter()
            .map(|sprite| sprite.x + sprite.width)
            .max()
            .unwrap_or(0);
        let texture_height = sprites
            .iter()
            .map(|sprite| sprite.y + sprite.height)
            .max()
            .unwrap_or(0);
        let sprite_sheet = SpriteSheet {
            texture_width,
            texture_height,
            sprites,
        };
        println!("List({})", ron::to_string(&sprite_sheet).unwrap());
    }

    Ok(())
}
