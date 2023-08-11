use rtge::{
    controls::direction::Direction,
    rendering::{
        entity::{print_sprites, Entity},
        position::Position,
        sprite::load_sprite,
    },
};

fn main() {
    let bob = Entity {
        name: "bob".to_string(),
        sprite: load_sprite("./src/bin/print_sprite/sprites/bob.json".to_string()),
        position: Position { x: 10, y: 10 },
        direction: Direction {
            up: false,
            down: false,
            left: false,
            right: false,
        },
        speed: 2,
        animation_name: None,
    };
    let mut another_bob = bob.clone();
    another_bob.position.x = 100;
    let mut entities = vec![bob, another_bob];
    print_sprites(&mut entities);
}
