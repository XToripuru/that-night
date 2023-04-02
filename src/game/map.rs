use super::*;

pub struct Boss {
    pub nth: i32,
    pub resp: Option<i32>,
    pub pos: Option<Position>,
}

pub struct Map {
    pub bullets: Vec<Bullet>,
    pub pass: MultiVec<bool>,
    pub enemies: MultiVec<Option<Enemy>>,
    pub tiles: MultiVec<Tile>,
    pub bombs: Vec<Bomb>,
    pub turrets: Vec<Turret>,
    pub emps: Vec<Emp>,
    pub chests: VecDeque<(i32, i32)>,
    pub boss: Boss,
    pub spawned: i32,
    pub w: i32,
    pub h: i32,
}

impl Map {
    pub fn new() -> Map {
        let (w, h) = (400, 400);

        let bullets = vec![];

        let tiles = Map::generate(w, h);

        let mut pass = MultiVec {
            inner: tiles.iter().map(|t| matches!(t, Tile::None)).collect(),
            w,
        };

        let mut tiles = MultiVec { inner: tiles, w };

        let spawn = 1024;
        let enemies = Map::spawn_enemies(spawn, &mut pass, w, h);

        let bombs = vec![];
        let turrets = vec![];
        let emps = vec![];

        let chests = Map::spawn_chests(480, &mut pass, &mut tiles, w as i32, h as i32);

        let boss = Boss {
            nth: 0,
            resp: None,
            pos: None,
        };

        Map {
            bullets,
            pass,
            enemies,
            tiles,
            bombs,
            turrets,
            emps,
            chests,
            boss,
            spawned: spawn,
            w: w as i32,
            h: h as i32,
        }
    }
    fn spawn_chests(
        n: i32,
        pass: &mut MultiVec<bool>,
        tiles: &mut MultiVec<Tile>,
        w: i32,
        h: i32,
    ) -> VecDeque<(i32, i32)> {
        let mut chests = VecDeque::with_capacity(n as usize);
        let mut rng = thread_rng();

        let mut i = 0;
        while i < n {
            let x = rng.gen_range(0..w);
            let y = rng.gen_range(0..h);

            if !pass[(x, y)] {
                continue;
            }

            let ty = ChestType::random();
            tiles[(x, y)] = Tile::Chest(Chest {
                start: 0,
                duration: 60 * 60 * 4,
                ty,
            });
            chests.push_back((x, y));

            i += 1;
        }

        chests
    }
    fn spawn_enemies(
        n: i32,
        pass: &mut MultiVec<bool>,
        w: usize,
        h: usize,
    ) -> MultiVec<Option<Enemy>> {
        let mut enemies = MultiVec {
            inner: vec![None; w * h],
            w,
        };

        let mut rng = thread_rng();
        let mut i = 0;
        while i < n {
            let x = rng.gen_range(0..w) as i32;
            let y = rng.gen_range(0..h) as i32;

            if !pass[(x, y)] || dist(x, y, w as i32 / 2, h as i32 / 2) <= 20 {
                continue;
            }

            pass[(x, y)] = false;
            enemies[(x, y)] = Some(Enemy {
                uid: i,
                hp: 1,
                mhp: 1,
                last: rng.gen_range(0..=120),
                cd: 19,
                slowed: 0,
                immobilized: false,
                ty: EnemyType::Zombie,
            });

            i += 1;
        }

        enemies
    }
    fn generate(w: usize, h: usize) -> Vec<Tile> {
        // generate random structures on map
        let mut tiles = vec![Tile::None; w * h];

        // map borders
        for i in 0..w {
            tiles[i] = Tile::Wall(Wall { bullets: 0 });
            tiles[i + w * (h - 1)] = Tile::Wall(Wall { bullets: 0 });
        }
        for i in 0..h {
            tiles[i * w] = Tile::Wall(Wall { bullets: 0 });
            tiles[i * w + (w - 1)] = Tile::Wall(Wall { bullets: 0 });
        }

        let mut rng = rand::thread_rng();
        for x in 1..w - 1 {
            for y in 1..h - 1 {
                if dist(w as i32 / 2, h as i32 / 2, x as i32, y as i32) < 10 {
                    continue;
                }
                // ##
                // #X <- X is current position, we dont want wall here
                match (
                    &tiles[x - 1 + (y - 1) * w],
                    &tiles[x - 1 + y * w],
                    &tiles[x + (y - 1) * w],
                ) {
                    (Tile::Wall(_), Tile::Wall(_), Tile::Wall(_)) => continue,
                    _ => {}
                };

                let mut walls = 0;
                for z in -1i32..=1 {
                    for t in -1i32..=1 {
                        if matches!(
                            tiles[(x as i32 + z) as usize + (y as i32 + t) as usize * w],
                            Tile::Wall(_)
                        ) {
                            walls += 1;
                        }
                    }
                }

                if walls > 0 && rng.gen_range(0..100) < 35 {
                    tiles[x + y * w] = if rng.gen_range(0..100) < 70 {
                        Tile::Wall(Wall { bullets: 0 })
                    } else {
                        Tile::MovableWall(MovableWall { bullets: 0 })
                    };
                } else if rng.gen_range(0..100) < 3 {
                    tiles[x + y * w] = Tile::Wall(Wall { bullets: 0 });
                }
            }
        }
        for x in 1..w - 1 {
            for y in 1..h - 1 {
                if matches!(tiles[x - 1 + y * w], Tile::Wall(_) | Tile::MovableWall(_))
                    && matches!(tiles[x + 1 + y * w], Tile::Wall(_) | Tile::MovableWall(_))
                    && matches!(tiles[x + (y - 1) * w], Tile::Wall(_) | Tile::MovableWall(_))
                    && matches!(tiles[x + (y + 1) * w], Tile::Wall(_) | Tile::MovableWall(_))
                {
                    tiles[x + y * w] = Tile::MovableWall(MovableWall { bullets: 0 });
                }
            }
        }

        tiles
    }
}

#[derive(Clone)]
pub enum Tile {
    None,
    Wall(Wall),
    MovableWall(MovableWall),
    Chest(Chest),
}

#[derive(Clone)]
pub struct Wall {
    pub bullets: i8,
}

#[derive(Clone)]
pub struct MovableWall {
    pub bullets: i8,
}

pub struct MultiVec<T> {
    pub inner: Vec<T>,
    pub w: usize,
}

impl<T> Index<(i32, i32)> for MultiVec<T> {
    type Output = T;
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        &self.inner[y as usize * self.w..][..self.w][x as usize]
    }
}

impl<T> IndexMut<(i32, i32)> for MultiVec<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        &mut self.inner[y as usize * self.w..][..self.w][x as usize]
    }
}

impl<T> Index<usize> for MultiVec<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.inner[idx]
    }
}

impl<T> IndexMut<usize> for MultiVec<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.inner[idx]
    }
}
