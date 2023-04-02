use super::*;

#[derive(Clone)]
pub struct Enemy {
    pub uid: i32,

    pub hp: i8,
    pub mhp: i8,

    // move
    pub last: i32,
    pub cd: i32,

    pub slowed: i32,
    pub immobilized: bool,

    pub ty: EnemyType,
}

#[derive(Clone)]
pub enum EnemyType {
    // might add some Ghosts or Werewolves later lol
    Zombie,
    ZombieBoss(ZombieBoss),
}

#[derive(Clone)]
pub struct ZombieBoss {
    // summoning
    pub last: i32,
    pub cd: i32,
}

const FOV: f32 = 24.0;
pub fn spawn_enemy(map: &mut Map, player: &Player) {
    let mut rng = thread_rng();

    loop {
        let x = rng.gen_range(0..map.w);
        let y = rng.gen_range(0..map.h);

        if !map.pass[(x, y)] || dist(x, y, player.x, player.y) < FOV as i32 + 4 {
            continue;
        }

        let hp = 1 + (player[Score] / 1000) as i8;

        map.pass[(x, y)] = false;
        map.enemies[(x, y)] = Some(Enemy {
            uid: map.spawned,
            hp,
            mhp: hp,
            last: 0,
            slowed: 0,
            immobilized: false,
            cd: 19,
            ty: EnemyType::Zombie,
        });
        map.spawned += 1;
        break;
    }
}

pub fn spawn_boss(map: &mut Map, player: &Player, state: &mut State, frame: i32, hp: i8) -> (i32, i32) {
    let mut rng = thread_rng();

    loop {
        let x = rng.gen_range(1..map.w - 1);
        let y = rng.gen_range(1..map.h - 1);

        if !map.pass[(x, y)] || dist(x, y, player.x, player.y) <= FOV as i32 {
            continue;
        }

        state.play(Sound::BossAppear, 0.5);

        let boss = Enemy {
            uid: map.spawned,
            hp,
            mhp: hp,
            last: 0,
            cd: 47,
            slowed: 0,
            immobilized: false,
            ty: EnemyType::ZombieBoss(ZombieBoss {
                last: frame,
                cd: 6 * 60,
            }),
        };

        map.pass[(x, y)] = false;
        map.enemies[(x, y)] = Some(boss);
        map.spawned += 1;
        return (x, y);
    }
}

pub fn update_enemies(map: &mut Map, player: &mut Player, state: &mut State, frame: i32) {
    for x in max(0.0, player.x as f32 - MOB_FOV as f32)
        ..min(player.x as f32 + MOB_FOV as f32, map.w as f32)
    {
        for y in max(0.0, player.y as f32 - MOB_FOV as f32)
            ..min(player.y as f32 + MOB_FOV as f32, map.h as f32)
        {
            if let Some(ref mut enemy) = map.enemies[(x, y)] {
                if let EnemyType::ZombieBoss(ref mut boss) = enemy.ty {
                    if frame >= boss.last + boss.cd && (!enemy.immobilized || enemy.slowed <= frame)
                    {
                        boss.last = frame;
                        let mut rng = thread_rng();

                        for _ in 0..4 {
                            let dx = rng.gen_range(-1..=1);
                            let dy = rng.gen_range(-1..=1);

                            if map.pass[(x + dx, y + dy)] {
                                map.pass[(x + dx, y + dy)] = false;

                                let enemy = Enemy {
                                    uid: map.spawned,
                                    hp: 1,
                                    mhp: 1,
                                    last: frame,
                                    cd: 19,
                                    slowed: 0,
                                    immobilized: false,
                                    ty: EnemyType::Zombie,
                                };
                                map.enemies[(x + dx, y + dy)] = Some(enemy);
                                map.spawned += 1;

                                break;
                            }
                        }
                    }
                }

                let enemy = map.enemies[(x, y)].as_mut().unwrap();
                if (enemy.immobilized && enemy.slowed > frame)
                    || enemy.last
                        + if enemy.slowed > frame {
                            enemy.cd * 3
                        } else {
                            enemy.cd
                        }
                        > frame
                {
                    continue;
                }
                enemy.last = frame;
                let is_boss = matches!(enemy.ty, EnemyType::ZombieBoss(_));

                let mut rng = thread_rng();
                let (nx, ny) = if rng.gen_range(0..100) < 70 {
                    pathfind(map, (x, y), (player.x, player.y))
                } else {
                    let dx = rng.gen_range(-1..=1);
                    let dy = if dx == 0 { rng.gen_range(-1..=1) } else { 0 };

                    (x + dx, y + dy)
                };
                if nx != 0 && ny != 0 && map.pass[(nx, ny)] {
                    map.pass[(x, y)] = true;
                    map.pass[(nx, ny)] = false;
                    map.enemies[(nx, ny)] = map.enemies[(x, y)].take();

                    let d = dist(player.x, player.y, x, y) as f32;
                    let fade = 10.0 / (10.0 + d * d);
                    state.play(Sound::Walking, 0.5 * fade);

                    if is_boss {
                        map.boss.pos = Some(Position { x: nx, y: ny });
                    }
                }
            }
        }
    }
}
