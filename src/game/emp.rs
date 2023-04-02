use super::*;

#[derive(Clone)]
pub struct Emp {
    pub x: i32,
    pub y: i32,
    pub start: i32,
    pub duration: i32,
    pub slow: i32,
    pub radius: i32,
    pub damage: i32,
    pub immobilize: bool,
}

pub fn check_emp(player: &mut Player, map: &mut Map, state: &mut State, frame: i32) {
    if frame - player[CdEmp] < player[LastEmp] || player[Emp] == 0 {
        return;
    }

    state.play(Sound::UseEmp, 1.0);

    let emp = Emp {
        x: player.x,
        y: player.y,
        start: frame,
        duration: player[DurEmp],
        slow: player[SlowEmp],
        radius: player[RadEmp],
        damage: player[DmgEmp],
        immobilize: player[StunEmp] == 1,
    };

    for x in max(0.0, emp.x as f32 - emp.radius as f32)
        ..=min((map.w - 1) as f32, emp.x as f32 + emp.radius as f32)
    {
        for y in max(0.0, emp.y as f32 - emp.radius as f32)
            ..=min((map.h - 1) as f32, emp.y as f32 + emp.radius as f32)
        {
            let dist = ((emp.x - x).pow(2) as f32 + (emp.y - y).pow(2) as f32)
                .sqrt()
                .round() as i32;
            if dist > emp.radius {
                continue;
            }

            if let Some(ref mut enemy) = map.enemies[(x, y)] {
                if enemy.slowed < frame {
                    enemy.slowed = frame;
                }

                enemy.slowed += if emp.immobilize {
                    emp.slow / 2
                } else {
                    emp.slow
                };
                enemy.immobilized = emp.immobilize;
                enemy.hp -= emp.damage as i8;

                if enemy.hp <= 0 {
                    player.progress.killed += 1;

                    map.pass[(x, y)] = true;

                    let mut rng = thread_rng();
                    match enemy.ty {
                        EnemyType::Zombie => {
                            player[Score] += player[ScoreEmp];

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
            }
        }
    }

    player[LastEmp] = frame;
    player[Emp] -= 1;
    player.progress.used[Weapon::Emp as usize] += 1;

    map.emps.push(emp);
}

// pub fn update_emps(map: &mut Map, frame: i32) {
//     let mut i = 0;

//     while i < map.emps.len() {
//         let emp = &mut map.emps[i];

//         if frame >= emp.start + emp.duration {
//             map.emps.swap_remove(i);
//             continue;
//         }

//         let (x, y) = (emp.x, emp.y);
//         let radius =
//             (emp.duration - (emp.start + emp.duration - frame)) * emp.radius / emp.duration;

//         for x in
//             max(0.0, x as f32 - radius as f32)..=min((map.w - 1) as f32, x as f32 + radius as f32)
//         {
//             for y in max(0.0, y as f32 - radius as f32)
//                 ..=min((map.h - 1) as f32, y as f32 + radius as f32)
//             {
//                 if dist(emp.x, emp.y, x, y) > radius {
//                     continue;
//                 }

//                 if let Some(ref mut enemy) = map.enemies[(x, y)] {
//                     if !emp.slowed.contains(&enemy.uid) {
//                         if enemy.slowed < frame {
//                             enemy.slowed = frame;
//                         }
//                         enemy.slowed += emp.slow;
//                         enemy.immobilized = emp.immobilize;
//                         enemy.hp -= emp.damage as i8;

//                         emp.slowed.push(enemy.uid);
//                     }
//                 }
//             }
//         }

//         i += 1;
//     }
// }
