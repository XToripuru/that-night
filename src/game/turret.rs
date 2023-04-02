use super::*;

#[derive(Clone)]
pub struct Turret {
    pub x: i32,
    pub y: i32,
    pub dmg: i8,
    pub direction: usize,
    pub start: i32,
    pub duration: i32,
    pub cd: i32,
    pub last: i32,
}

pub fn check_turret(
    keys: &Vec<Key>,
    player: &mut Player,
    map: &mut Map,
    state: &State,
    frame: i32,
) {
    if frame - player[CdTurret] < player[LastTurret]
        || player[Turret] == 0
        || map
            .turrets
            .iter()
            .find(|t| t.x == player.x && t.y == player.y)
            .is_some()
        || map
            .bombs
            .iter()
            .find(|b| b.x == player.x && b.y == player.y)
            .is_some()
    {
        return;
    }
    let mut rng = thread_rng();
    let direction = if let Some(&last_key) = keys.last() {
        match last_key {
            _ if last_key as u32 == state.storage.hotkeys[Hotkey::Up as usize] => 1,
            _ if last_key as u32 == state.storage.hotkeys[Hotkey::Right as usize] => 2,
            _ if last_key as u32 == state.storage.hotkeys[Hotkey::Down as usize] => 3,
            _ if last_key as u32 == state.storage.hotkeys[Hotkey::Left as usize] => 0,
            _ => rng.gen_range(0..=3),
        }
    } else {
        rng.gen_range(0..=3)
    };

    state.play(Sound::UseTurret, 0.3);

    let turret = Turret {
        x: player.x,
        y: player.y,
        dmg: player[DmgTurret] as i8,
        direction,
        start: frame,
        duration: player[DurTurret],
        cd: player[CdTurretShot],
        last: frame - player[CdTurretShot],
    };

    player[LastTurret] = frame;
    player[Turret] -= 1;
    player.progress.used[Weapon::Turret as usize] += 1;

    map.pass[(player.x, player.y)] = false;
    map.turrets.push(turret);
}

pub const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
pub fn update_turrets(map: &mut Map, player: &mut Player, state: &mut State, frame: i32) {
    let mut i = 0;
    while i < map.turrets.len() {
        let turret = &mut map.turrets[i];

        if frame >= turret.last + turret.cd {
            turret.last = frame;

            let (dx, dy) = DIRECTIONS[turret.direction];

            let d = dist(player.x, player.y, turret.x, turret.y) as f32;
            let fade = 100.0 / (100.0 + d * d);
            state.play(Sound::TurretShoot, 0.3 * fade);

            let bullet = Bullet {
                x: turret.x,
                y: turret.y,
                dx,
                dy,
                pierce: 0,
                fork: 0,
                dmg: turret.dmg,
                last: 0,
                cd: 3,
                start: frame,
                hit: None,
            };
            map.bullets.push(bullet);
        }

        if frame >= turret.start + turret.duration {
            map.pass[(turret.x, turret.y)] = true;
            map.turrets.swap_remove(i);
            continue;
        }

        i += 1;
    }
}
