use super::*;

#[derive(Clone)]
pub struct Bomb {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub dmg: i32,
    pub start: i32,
    pub duration: i32,
}

pub fn check_bomb(player: &mut Player, map: &mut Map, state: &mut State, frame: i32) {
    if frame - player[CdBomb] < player[LastBomb]
        || player[Bomb] == 0
        || map
            .bombs
            .iter()
            .find(|b| b.x == player.x && b.y == player.y)
            .is_some()
        || map
            .turrets
            .iter()
            .find(|t| t.x == player.x && t.y == player.y)
            .is_some()
    {
        return;
    }

    state.play(Sound::UseBomb, 0.5);

    let bomb = Bomb {
        x: player.x,
        y: player.y,
        radius: player[RadBomb],
        dmg: player[DmgBomb],
        start: frame,
        duration: player[FuseTimeBomb],
    };

    player[LastBomb] = frame;
    player[Bomb] -= 1;
    player.progress.used[Weapon::Bomb as usize] += 1;

    map.bombs.push(bomb);
}

pub fn update_bombs(map: &mut Map, player: &mut Player, state: &mut State, frame: i32) {
    let mut rng = thread_rng();
    for i in 0..map.bombs.len() {
        let bomb = &map.bombs[i];
        let (x, y) = (bomb.x, bomb.y);
        let mut rng = thread_rng();

        if frame >= bomb.start + bomb.duration {
            map.bombs.swap_remove(i);
            break;
        }

        if (frame - bomb.start + 30) % 60 == 0 {
            let d = dist(player.x, player.y, x, y) as f32;
            let fade = 100.0 / (100.0 + d * d);
            state.play(Sound::BombTick, 0.75 * fade);
        }

        if frame == bomb.start + bomb.duration - 20 {
            let d = dist(player.x, player.y, x, y) as f32;
            let fade = 100.0 / (100.0 + d * d);
            state.play(Sound::BombExplosion, 0.8 * fade);
        }

        if frame == bomb.start + bomb.duration - 10 {

            // destroy blocks around bomb (plus sign)
            for (dx, dy) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                if bomb.x + dx > 0
                    && bomb.x + dx < map.w - 1
                    && bomb.y + dy > 0
                    && bomb.y + dy < map.h - 1
                {
                    let (x, y) = (bomb.x + dx, bomb.y + dy);

                    match map.tiles[(x, y)] {
                        Tile::Wall(Wall { bullets })
                        | Tile::MovableWall(MovableWall { bullets }) => {
                            for _ in 0..bullets {
                                let (dx, dy) = DIRECTIONS[rng.gen_range(0..DIRECTIONS.len())];

                                map.bullets.push(Bullet {
                                    x,
                                    y,
                                    dx,
                                    dy,
                                    dmg: player[DmgBomb] as i8,
                                    pierce: 0,
                                    fork: 0,
                                    last: 0,
                                    cd: rng.gen_range(2..=4),
                                    start: frame,
                                    hit: None,
                                });
                            }
                        }
                        Tile::Chest(_) => {
                            let (idx, _) = map
                                .chests
                                .iter()
                                .enumerate()
                                .find(|&(_i, &(cx, cy))| cx == x && cy == y)
                                .unwrap();
                            map.chests.remove(idx);
                        }
                        _ => {}
                    }

                    map.tiles[(x, y)] = Tile::None;
                    map.pass[(x, y)] = true;
                }
            }
            // change walls to movable walls
            for (dx, dy) in [(-1, -1), (-1, 1), (1, 1), (1, -1)] {
                let (x, y) = (bomb.x + dx, bomb.y + dy);
                if bomb.x + dx > 0
                    && bomb.x + dx < map.w - 1
                    && bomb.y + dy > 0
                    && bomb.y + dy < map.h - 1
                    && matches!(map.tiles[(x, y)], Tile::Wall(_))
                {
                    let Tile::Wall(Wall { bullets }) = map.tiles[(x, y)] else { unreachable!() };

                    map.tiles[(x, y)] = Tile::MovableWall(MovableWall { bullets });
                }
            }
            for x in max(1.0, (bomb.x - bomb.radius) as f32)
                ..=min((map.w - 2) as f32, (bomb.x + bomb.radius) as f32)
            {
                for y in max(
                    1 as f32,
                    (bomb.y - ((bomb.x - x).abs() - bomb.radius).abs()) as f32,
                )
                    ..=min(
                        (map.h - 2) as f32,
                        (bomb.y + ((bomb.x - x).abs() - bomb.radius).abs()) as f32,
                    )
                {
                    if let Some(ref mut enemy) = map.enemies[(x, y)] {
                        enemy.hp -= bomb.dmg as i8;

                        if enemy.hp <= 0 {
                            player.progress.killed += 1;

                            map.pass[(x, y)] = true;

                            if let EnemyType::ZombieBoss(_) = enemy.ty {
                                map.boss.pos = None;
                                player[Score] += 100;
                                player.paused = true;
                                player.upgrading = Some(Upgrading::new(player, state));
                            } else {
                                player[Score] += player[ScoreBomb];
                            }

                            map.enemies[(x, y)].take();
                        }
                    }
                }
            }
            let radius = bomb.radius;
            if dist(x, y, player.x, player.y) <= radius {
                player.dead = true;
            }

            for idx in 0..map.bombs.len() {
                let bomb = &mut map.bombs[idx];
                if idx != i
                    && dist(x, y, bomb.x, bomb.y) <= radius
                    && frame <= bomb.start + bomb.duration - 20
                {
                    //bomb.start -= (bomb.start + bomb.duration) - frame - 20;
                    bomb.start = frame + 20 - bomb.duration;
                }
            }
        }
    }
}
