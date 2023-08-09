use std::io::{self};

use controls::{
    direction::Direction,
    inputs::{process_inputs, process_key_press_event},
};
use crossterm::event::{Event, KeyCode};
use rtge::{
    controls,
    rendering::{
        entity::{move_entities, move_entity, print_sprites_centered_on, Entity},
        position::Position,
        sprite::load_sprite,
    },
};

const TERM_SIZE_X: u16 = 20_000;
const TERM_SIZE_Y: u16 = 20_000;
const UP: char = 'z';
const DOWN: char = 's';
const LEFT: char = 'q';
const RIGHT: char = 'd';

//Main only useful to do some manual tests in the library
#[tokio::main]
async fn main() -> io::Result<()> {
    game().await
}

async fn game() -> io::Result<()> {
    let mut bob = Entity {
        name: "bob".to_string(),
        sprite: load_sprite("./src/bin/print_sprite/sprites/bob.json".to_string()),
        position: Position {
            x: TERM_SIZE_X / 2,
            y: TERM_SIZE_Y / 2,
        },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 2,
    };
    let mut enemies = vec![Entity {
        name: "enemy".to_string(),
        sprite: load_sprite("./src/bin/print_sprite/sprites/bob.json".to_string()),
        position: Position {
            x: (TERM_SIZE_X / 2) + 20,
            y: (TERM_SIZE_Y / 2) + 20,
        },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 1,
    }];

    loop {
        let bob_async = bob.clone();
        let enemies_async = enemies.clone();
        print_sprites_centered_on(&bob_async, &enemies_async);

        //Relies on crossterm crate event-stream feature
        let inputs_rules = |event: Event| -> Result<(), String> {
            bob.direction.up = event == process_key_press_event(UP);
            bob.direction.down = event == process_key_press_event(DOWN);
            bob.direction.left = event == process_key_press_event(LEFT);
            bob.direction.right = event == process_key_press_event(RIGHT);
            if event == Event::Key(KeyCode::Esc.into()) {
                return Err("Escape event".to_string());
            }
            Ok(())
        };

        match process_inputs(inputs_rules).await {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }

        bob = move_entity(bob);
        enemies = move_entities(enemies);
    }
    Ok(())
}
