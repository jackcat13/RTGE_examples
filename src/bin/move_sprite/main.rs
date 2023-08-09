use std::{thread::sleep, time::Duration};

use rtge::{
    controls::direction::Direction,
    rendering::{
        entity::{print_sprites_centered_on, Entity},
        position::Position,
        sprite::load_sprite,
    },
};

const TERM_SIZE_X: u16 = 1000;
const TERM_SIZE_Y: u16 = 1000;

fn main() {
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
    let others = vec![Entity {
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

    for i in 1..100 {
        let bob_async = bob.clone();
        let others_async = others.clone();
        print_sprites_centered_on(&bob_async, &others_async);

        bob.position.x += 1;

        sleep(Duration::from_millis(100));
    }
}
