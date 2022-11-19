use std::{collections::HashMap, fs::File, error::Error, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum Block {
    Leaf,
    Dirt,
    Stone,
    Lava,
    Orchid,
    Puddle,
    Wall,
    Banner(String),
    Item(char),
}

pub fn main() {
    try_main().unwrap();
}

pub fn try_main() -> Result<(), Box<dyn Error>> {
    // let mut entities = HashMap::new();
    // entities.insert(Coord {x: 0, y: 0}, Block::Grass);
    // entities.insert(Coord {x: 2, y: 0}, Block::Sand);
    // entities.insert(Coord {x: 4, y: 0}, Block::Rock);
    // entities.insert(Coord {x: 6, y: 0}, Block::Cinderblock);
    // entities.insert(Coord {x: 8, y: 0}, Block::Flowers);

    let mut file = File::open("maps/primordial.ron")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let map: HashMap<(i32, i32), Block> = ron::from_str(&contents).unwrap();
    dbg!(map);
    Ok(())

}