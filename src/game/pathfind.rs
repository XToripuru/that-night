use super::*;

pub const MOB_FOV: usize = 30;

pub fn mark(map: &Map, state: &mut [u8], x: usize, y: usize, offset: (i32, i32)) {
    let ax = (x as i32 - MOB_FOV as i32 / 2) + offset.0;
    let ay = (y as i32 - MOB_FOV as i32 / 2) + offset.1;

    if x > 0 && state[(x - 1) + y * MOB_FOV] != 1 && map.pass[(ax - 1, ay)] {
        state[(x - 1) + y * MOB_FOV] = 2;
    }
    if x < MOB_FOV - 1 && state[(x + 1) + y * MOB_FOV] != 1 && map.pass[(ax + 1, ay)] {
        state[(x + 1) + y * MOB_FOV] = 2;
    }
    if y > 0 && state[x + (y - 1) * MOB_FOV] != 1 && map.pass[(ax, ay - 1)] {
        state[x + (y - 1) * MOB_FOV] = 2;
    }
    if y < MOB_FOV - 1 && state[x + (y + 1) * MOB_FOV] != 1 && map.pass[(ax, ay + 1)] {
        state[x + (y + 1) * MOB_FOV] = 2;
    }

    state[x + y * MOB_FOV] = 1;
}

pub fn pathfind(map: &Map, from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    let mut state = [0u8; MOB_FOV * MOB_FOV];
    let mut poss = [(0i32, 0i32); MOB_FOV * 4 * 4];

    let q = (
        MOB_FOV as i32 / 2 + from.0 - to.0,
        MOB_FOV as i32 / 2 + from.1 - to.1,
    );

    mark(map, &mut state, MOB_FOV / 2, MOB_FOV / 2, to);

    for tries in 0..MOB_FOV * 4 {
        let mut idx = 0;

        for x in 0..MOB_FOV {
            for y in 0..MOB_FOV {
                if state[x + y * MOB_FOV] == 2 {
                    poss[idx] = (x as i32, y as i32);
                    idx += 1;
                }
            }
        }
        if idx == 0 {
            return (0, 0);
        }
        let mut best_k = 0;
        let mut best_d = 1024;

        for k in 0..idx {
            let pos = poss[k];
            let d = dist(q.0, q.1, pos.0, pos.1);

            if d <= 1 {
                return (
                    pos.0 - MOB_FOV as i32 / 2 + to.0,
                    pos.1 - MOB_FOV as i32 / 2 + to.1,
                );
            }
            if d < best_d {
                best_d = d;
                best_k = k;
            }
            let best = poss[best_k];
            mark(map, &mut state, best.0 as usize, best.1 as usize, to);
        }
    }

    (0, 0)
}
