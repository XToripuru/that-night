use super::*;

impl Index<Stat> for Player {
    type Output = i32;
    fn index(&self, index: Stat) -> &Self::Output {
        &self.stats[index as usize]
    }
}

impl IndexMut<Stat> for Player {
    fn index_mut(&mut self, index: Stat) -> &mut Self::Output {
        &mut self.stats[index as usize]
    }
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub stats: Vec<i32>,
    pub progress: Progress,
    pub running: bool,
    pub dead: bool,
    pub paused: bool,
    pub upgrading: Option<Upgrading>,
    pub upgrades: Vec<i32>,
}

#[repr(u8)]
#[derive(Clone)]
pub enum Stat {
    Score,
    ScoreAmmo,
    ScoreBomb,
    ScoreTurret,
    ScoreEmp,

    Food,
    Ammo,
    Bomb,
    Turret,
    Emp,

    MaxFood,
    MaxAmmo,
    MaxBomb,
    MaxTurret,
    MaxEmp,

    LastShot,
    CdShot,
    DmgAmmo,
    NotConsumeAmmo,
    PierceAmmo,
    ForkAmmo,

    LastBomb,
    CdBomb,
    RadBomb,
    DmgBomb,
    FuseTimeBomb,

    LastTurret,
    CdTurret,
    CdTurretShot,
    DmgTurret,
    DurTurret,

    LastEmp,
    CdEmp,
    RadEmp,
    DmgEmp,
    DurEmp,
    StunEmp,
    SlowEmp,

    LastMove,
    CdMove,

    Luck,

    Character,
}
pub use Stat::*;

impl Player {
    pub fn new(ch: Character, x: i32, y: i32) -> Self {
        let mut p = Player {
            x,
            y,
            stats: vec![0; 64],
            progress: Progress::new(),
            running: false,
            dead: false,
            paused: false,
            upgrading: None,
            upgrades: vec![0; Upgrade::COUNT],
        };

        p[Score] = 0;
        p[ScoreAmmo] = 3;
        p[ScoreBomb] = 3;
        p[ScoreTurret] = 3;
        p[ScoreEmp] = 3;

        p[Food] = 1_000_000;
        p[MaxFood] = 1_000_000;

        p[Ammo] = 10;
        p[MaxAmmo] = 10;
        p[DmgAmmo] = 1;
        p[CdShot] = 10;
        p[NotConsumeAmmo] = 0;
        p[PierceAmmo] = 0;
        p[ForkAmmo] = 0;

        p[CdBomb] = 60;
        p[Bomb] = 3;
        p[MaxBomb] = 3;
        p[RadBomb] = 5;
        p[DmgBomb] = 3;
        p[FuseTimeBomb] = 200;

        p[Turret] = 3;
        p[MaxTurret] = 3;
        p[CdTurret] = 60;
        p[CdTurretShot] = 300;
        p[DmgTurret] = 1;
        p[DurTurret] = 900;

        p[CdEmp] = 60;
        p[Emp] = 3;
        p[MaxEmp] = 3;
        p[RadEmp] = 8;
        p[DmgEmp] = 0;
        p[DurEmp] = 60;
        p[StunEmp] = 0;
        p[SlowEmp] = 300;

        p[CdMove] = 9;

        p[Luck] = 0;

        p[Character] = ch as i32;

        match ch {
            Character::Joshua => {
                p[CdMove] -= 1;
            }
            Character::Anne => {
                p[Ammo] = 1;
                p[MaxAmmo] = 1;
                p[Bomb] += 3;
                p[MaxBomb] += 3;
                p[RadBomb] += 1;
                p[FuseTimeBomb] -= 60;
            }
            Character::Andrew => {
                p[DmgAmmo] += 1;
                p[NotConsumeAmmo] += 20;
            }
            Character::Matthew => {
                p[DurTurret] *= 2;
            }
            Character::Megan => {
                p[RadEmp] += 3;
                p[StunEmp] = 1;
            }
            Character::LiShen => {
                p[Food] += 500_000;
                p[MaxFood] += 500_000;
                p[Luck] += 5;
            }
        }

        p
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Character {
    Joshua,
    Anne,
    Andrew,
    Matthew,
    Megan,
    LiShen,
}

pub fn make_moves(player: &mut Player, keys: &Vec<Key>, map: &mut Map, state: &State, frame: i32) {
    let Some(&last_key) = keys.last() else { return };

    let (dx, dy) = match last_key {
        _ if last_key as u32 == state.storage.hotkeys[Hotkey::Up as usize] => (0, -1),
        _ if last_key as u32 == state.storage.hotkeys[Hotkey::Right as usize] => (1, 0),
        _ if last_key as u32 == state.storage.hotkeys[Hotkey::Down as usize] => (0, 1),
        _ if last_key as u32 == state.storage.hotkeys[Hotkey::Left as usize] => (-1, 0),
        _ => {
            return;
        }
    };

    let (mut px, mut py) = (player.x, player.y);

    if frame
        < player[LastMove]
            + (if matches!(map.tiles[(px + dx, py + dy)], Tile::MovableWall(_)) {
                2 * player[CdMove]
            } else {
                player[CdMove]
            }) / if player.running { 2 } else { 1 }
    {
        return;
    }

    match map.tiles[(px + dx, py + dy)] {
        Tile::None => {
            if map.pass[(px + dx, py + dy)] {
                px += dx;
                py += dy;
                if player.running {
                    player[Food] -= 2000;
                }
            }
        }
        Tile::Wall(_) => {}
        Tile::MovableWall(MovableWall { bullets }) => {
            if map.pass[(px + 2 * dx, py + 2 * dy)] {
                px += dx;
                py += dy;
                if player.running {
                    player[Food] -= 2000;
                }

                map.tiles[(px, py)] = Tile::None;
                map.tiles[(px + dx, py + dy)] = Tile::MovableWall(MovableWall { bullets });

                map.pass[(px, py)] = true;
                map.pass[(px + dx, py + dy)] = false;
            }
        }
        Tile::Chest(ref chest) => {
            px += dx;
            py += dy;
            if player.running {
                player[Food] -= 2000;
            }

            state.play(Sound::PickChest, 0.25);

            match chest.ty {
                ChestType::Ammo => player[Ammo] = std::cmp::min(player[MaxAmmo], player[Ammo] + 3),
                ChestType::Bomb => player[Bomb] = std::cmp::min(player[MaxBomb], player[Bomb] + 1),
                ChestType::Turret => {
                    player[Turret] = std::cmp::min(player[MaxTurret], player[Turret] + 1)
                }
                ChestType::Emp => player[Emp] = std::cmp::min(player[MaxEmp], player[Emp] + 1),
                ChestType::Food => {
                    player[Food] = std::cmp::min(player[MaxFood], player[Food] + 500_000)
                }
                ChestType::Rainbow => {
                    player[Ammo] = std::cmp::min(player[MaxAmmo], player[Ammo] + 3);
                    player[Bomb] = std::cmp::min(player[MaxBomb], player[Bomb] + 1);
                    player[Turret] = std::cmp::min(player[MaxTurret], player[Turret] + 1);
                    player[Emp] = std::cmp::min(player[MaxEmp], player[Emp] + 1);
                    player[Food] = std::cmp::min(player[MaxFood], player[Food] + 500_000);
                }
            }

            map.tiles[(px, py)] = Tile::None;
            map.pass[(px, py)] = true;
            let (idx, _) = map
                .chests
                .iter()
                .enumerate()
                .find(|&(_, &(x, y))| x == px && y == py)
                .unwrap();
            map.chests.remove(idx);
        }
    }

    player[LastMove] = frame;
    player.x = px;
    player.y = py;
}
