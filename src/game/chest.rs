use super::*;

#[derive(Clone)]
pub struct Chest {
    pub start: i32,
    pub duration: i32,
    pub ty: ChestType,
}

#[derive(Clone)]
pub enum ChestType {
    Ammo,
    Bomb,
    Turret,
    Emp,
    Food,
    Rainbow,
}

impl From<i32> for ChestType {
    fn from(value: i32) -> Self {
        match value {
            0 => ChestType::Ammo,
            1 => ChestType::Bomb,
            2 => ChestType::Turret,
            3 => ChestType::Emp,
            4 => ChestType::Food,
            5 => ChestType::Rainbow,
            _ => {
                panic!()
            }
        }
    }
}

impl ChestType {
    pub fn random() -> Self {
        pub const COUNT: i32 = 5;

        let mut rng = thread_rng();
        ChestType::from(rng.gen_range(0..COUNT))
    }
}

const FOV: i32 = 12;
pub fn spawn_random_chest(map: &mut Map, player: &Player, frame: i32) {
    let mut rng = thread_rng();

    loop {
        let x = rng.gen_range(0..map.w);
        let y = rng.gen_range(0..map.h);

        if !map.pass[(x, y)] || dist(x, y, player.x, player.y) < FOV + 4 {
            continue;
        }

        let ty = ChestType::from(rng.gen_range(0..5));
        spawn_chest(map, frame, x, y, ty);

        break;
    }
}

pub fn spawn_chest(map: &mut Map, frame: i32, x: i32, y: i32, ty: ChestType) {
    let chest = Chest {
        start: frame,
        duration: 14_400,
        ty,
    };

    map.tiles[(x, y)] = Tile::Chest(chest);
    map.chests.push_back((x, y));
}
