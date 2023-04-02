use super::*;

#[derive(Clone)]
pub struct Bullet {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
    pub dmg: i8,
    pub pierce: i8,
    pub fork: i8,
    pub last: i32,
    pub cd: i32,
    pub start: i32,
    pub hit: Option<i32>,
}

pub fn check_shot(
    keys: &Vec<Key>,
    player: &mut Player,
    bullets: &mut Vec<Bullet>,
    state: &State,
    frame: i32,
) {
    if player[Ammo] == 0 || frame - player[LastShot] < player[CdShot] {
        return;
    }

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
    let (x, y) = (player.x, player.y);
    let mut rng = thread_rng();

    player[LastShot] = frame;
    player.progress.used[Weapon::Ammo as usize] += 1;
    if rng.gen_range(0..100) >= player[NotConsumeAmmo] {
        player[Ammo] -= 1;
    }

    state.play(Sound::UseAmmo, 0.25);

    let bullet = Bullet {
        x,
        y,
        dx,
        dy,
        pierce: player[PierceAmmo] as i8,
        fork: player[ForkAmmo] as i8,
        dmg: player[DmgAmmo] as i8,
        last: 0,
        cd: 3,
        start: frame,
        hit: None,
    };
    bullets.push(bullet);
}

pub fn check_bullets(map: &mut Map, player: &mut Player, state: &mut State, frame: i32) {
    let mut i = 0;
    while i < map.bullets.len() {
        let bullet = &mut map.bullets[i];
        let (x, y) = (bullet.x, bullet.y);

        if let Some(ref mut enemy) = map.enemies[(x, y)] {
            if bullet
                .hit
                .and_then(|uid| if uid == enemy.uid { Some(uid) } else { None })
                .is_none()
            {
                let uid = enemy.uid;
                bullet.hit = Some(uid);
                bullet.pierce -= 1;
                bullet.fork -= 1;
                enemy.hp -= bullet.dmg;

                if bullet.fork >= 0 {
                    let (dx, dy) = if bullet.dx == 0 { (1, 0) } else { (0, 1) };
                    let (dmg, fork) = (bullet.dmg, bullet.fork);

                    for k in [-1, 1] {
                        map.bullets.push(Bullet {
                            x: x + k * dx,
                            y: y + k * dy,
                            dx: dx * k,
                            dy: dy * k,
                            dmg,
                            pierce: 0,
                            fork,
                            last: frame,
                            cd: 3,
                            start: frame,
                            hit: None,
                        });
                    }
                }

                let bullet = &mut map.bullets[i];
                if bullet.pierce < 0 {
                    map.bullets.swap_remove(i);
                }

                if enemy.hp <= 0 {
                    player.progress.killed += 1;

                    map.pass[(x, y)] = true;

                    let mut rng = thread_rng();
                    match enemy.ty {
                        EnemyType::Zombie => {
                            player[Score] += player[ScoreAmmo];

                            if rng.gen_range(0..100) < 65 + player[Luck] {
                                spawn_chest(map, frame, x, y, ChestType::random());
                            }
                        }
                        EnemyType::ZombieBoss(_) => {
                            map.boss.pos = None;
                            player[Score] += 100;
                            player.paused = true;
                            player.upgrading = Some(Upgrading::new(player, state));

                            spawn_chest(map, frame, x, y, ChestType::Rainbow);
                        }
                    }
                    map.enemies[(x, y)].take();
                }

                continue;
            }
        }

        match map.tiles[(x, y)] {
            Tile::Wall(Wall { ref mut bullets }) => {
                *bullets += 1;

                map.bullets.swap_remove(i);
                continue;
            }
            Tile::MovableWall(MovableWall { ref mut bullets }) => {
                *bullets += 1;

                map.bullets.swap_remove(i);
                continue;
            }
            _ => {}
        }

        i += 1;
    }
}

pub fn update_bullets(map: &mut Map, frame: i32) {
    let mut i = 0;
    while i < map.bullets.len() {
        let bullet = &mut map.bullets[i];

        if frame < bullet.last + bullet.cd {
            i += 1;
            continue;
        }

        bullet.last = frame;
        bullet.x += bullet.dx;
        bullet.y += bullet.dy;

        if let Some(bomb) = map
            .bombs
            .iter_mut()
            .find(|bomb| bomb.x == bullet.x && bomb.y == bullet.y)
        {
            if frame < bomb.start + bomb.duration - 20 {
                //bomb.start -= (bomb.start + bomb.duration) - frame - 20;
                bomb.start = frame + 20 - bomb.duration;
                map.bullets.swap_remove(i);
                continue;
            }
        }

        i += 1;
    }
}

/*

match (
            pass[(bullet.x, bullet.y)],
            &mut enemies[(bullet.x, bullet.y)],
        ) {
            (false, None) => {
                bullets.swap_remove(i);
                continue;
            }
            (false, Some(ref mut enemy)) => {
                let uid = enemy.uid;
                enemy.hp -= 1;

                if enemy.hp <= 0 {
                    player[Score] += match enemy.ty {
                        EnemyType::Zombie => player[ScoreAmmo],
                        EnemyType::ZombieBoss(_) => 100
                    };
                    enemies[(bullet.x, bullet.y)].take();
                    pass[(bullet.x, bullet.y)] = true;
                }

                bullet.hit = Some(uid);
                bullets.swap_remove(i);
                continue;
            }
            _ => {}
        }
 */
