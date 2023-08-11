use std::io;

use crossterm::event::{Event, KeyCode};
use rtge::{
    controls::{direction::Direction, inputs::process_inputs},
    rendering::{
        entity::{print_sprites, Entity},
        position::Position,
        sprite::load_sprite,
    },
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let camel = Entity {
        name: "camel".to_string(),
        sprite: load_sprite("./src/bin/sprites/camel.json".to_string()),
        position: Position { x: 100, y: 10 },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 2,
        animation_name: Some("walking".to_string()),
    };
    let mut entities = vec![camel];
    loop {
        print_sprites(&mut entities);
        let inputs_rules = |event: Event| -> Result<(), String> {
            if event == Event::Key(KeyCode::Esc.into()) {
                return Err("Escape event".to_string());
            }
            Ok(())
        };

        if let Err(_) = process_inputs(inputs_rules).await {
            break;
        }
    }
    Ok(())
}
