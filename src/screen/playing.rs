use nannou::color::white_point::E;

use super::*;

pub struct Playing {
    frame: i32,
    map: Map,
    cam: Camera,
    player: Player,
    movement: Vec<Key>,
    ambient: Option<Sink>,
    next: Option<Box<Screen>>
}

impl Playing {
    pub fn new(ch: Character, state: &State) -> Self {
        let ambient = state.play_get(Sound::Ambient, 0.2);
        let map = Map::new();
        let (x, y) = (map.w / 2, map.h / 2);
        Playing {
            frame: 0,
            player: Player::new(ch, x, y),
            map,
            cam: Camera::new(x as f32, y as f32),
            movement: Vec::with_capacity(4),
            ambient: Some(ambient),
            next: None,
        }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        const HFOV: f32 = 24.0;
        const VFOV: f32 = 12.0;
        const TILE_WIDTH: f32 = 48.0;

        for x in max(self.cam.x - HFOV, 0.0)..=min(self.cam.x + HFOV, (self.map.w - 1) as f32) {
            for y in max(self.cam.y - VFOV, 0.0)..=min(self.cam.y + VFOV, (self.map.h - 1) as f32) {
                let (vx, vy) = (self.cam.x - x as f32, self.cam.y - y as f32);

                // (my)TODO: optimize later for light table for quick access
                let alpha = {
                    let mut dsq = (vx * vx) + (vy * vy);
                    if self.frame < 60 {
                        dsq *= ((self.frame - 30) as f32 / 30.0).clamp(0.001, 1.0).inv();
                    }
                    let mut sum = (255 - 2 * dsq as i32).clamp(0, 255);
                    for bomb in &self.map.bombs {
                        if self.frame < bomb.start + bomb.duration - 20 {
                            let dsq = (bomb.x - x) * (bomb.x - x) + (bomb.y - y) * (bomb.y - y);
                            sum += (255 - 16 * dsq).clamp(0, 255);
                        } else {
                            let dsq = (bomb.x - x) * (bomb.x - x) + (bomb.y - y) * (bomb.y - y);
                            let v = dsq as f32
                                * (1.0
                                    - 0.5
                                        * (3.1415
                                            * (self.frame - (bomb.start + bomb.duration - 20))
                                                as f32
                                            / 20.0)
                                            .sin());
                            sum += (255 - 16 * v as i32).clamp(0, 255);
                        }
                    }
                    sum.clamp(0, 255) as u8
                };

                if alpha <= 8 {
                    continue;
                }

                match self.map.tiles[(x, y)] {
                    Tile::None => {
                        draw.rect()
                            .rgba8(48, 48, 48, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h(TILE_WIDTH, TILE_WIDTH);
                    }
                    Tile::Wall(_) => {
                        draw.rect()
                            .rgba8(255, 165, 0, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h(TILE_WIDTH, TILE_WIDTH);
                    }
                    Tile::MovableWall(_) => {
                        draw.rect()
                            .rgba8(200, 150, 100, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h(TILE_WIDTH, TILE_WIDTH);
                    }
                    Tile::Chest(ref chest) => {
                        let c = match chest.ty {
                            ChestType::Ammo => ORANGE,
                            ChestType::Bomb => RED,
                            ChestType::Turret => BLUE,
                            ChestType::Emp => PURPLE,
                            ChestType::Food => TURQUOISE,
                            ChestType::Rainbow => [ORANGE, RED, BLUE, PURPLE, TURQUOISE]
                                [((self.frame / 30) % 5) as usize],
                        };

                        let (r, g, b) = (c.red, c.green, c.blue);

                        draw.rect()
                            .rgba8(48, 48, 48, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h(TILE_WIDTH, TILE_WIDTH);

                        // draw.rect()
                        //     .rgba8(r/2, g/2, b/2, alpha)
                        //     .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                        //     .w_h((TILE_WIDTH * 0.5 + 8.0).round(), (TILE_WIDTH * 0.5 + 8.0).round());

                        draw.rect()
                            .stroke_weight(4.0)
                            .stroke(Rgba8::new(r, g, b, alpha))
                            .rgba8(r / 2, g / 2, b / 2, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h((TILE_WIDTH * 0.5).round(), (TILE_WIDTH * 0.5).round());
                    }
                    _ => {}
                }
                match self.map.enemies[(x, y)] {
                    Some(ref enemy) => {
                        let (r, g, b) = match enemy.ty {
                            EnemyType::Zombie => (
                                0,
                                100 + (155.0 * enemy.hp as f32 / enemy.mhp as f32) as u8,
                                0,
                            ),
                            EnemyType::ZombieBoss(_) => (
                                100,
                                0,
                                (100.0 + (self.frame as f32 / 30.0).sin() * 25.0) as u8,
                            ),
                        };

                        draw.rect()
                            .rgba8(r, g, b, alpha)
                            .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                            .w_h(TILE_WIDTH, TILE_WIDTH);
                    }
                    None => {}
                }
            }
        }

        for bomb in &self.map.bombs {
            let (x, y) = (bomb.x, bomb.y);

            match bomb.duration - (self.frame - bomb.start) {
                n @ 0..=20 => {
                    // let radius = (n - 20).abs() * self.player[RadBomb] / 20;
                    let radius =
                        ((3.1415 * n as f32 / 20.0).sin() * bomb.radius as f32).round() as i32;

                    // TODO: optimize this so it's just loops and no `if` inside (with math)
                    // for z in max(0.0, (x - radius) as f32)
                    //     ..=min((self.map.w - 1) as f32, (x + radius) as f32)
                    // {
                    //     for t in max(0.0, (y - radius) as f32)
                    //         ..=min((self.map.h - 1) as f32, (y + radius) as f32)
                    //     {
                    //         if dist(x, y, z, t) <= radius {
                    //             let (vx, vy) = (self.cam.x - z as f32, self.cam.y - t as f32);

                    //             draw.rect()
                    //                 .rgba8(255, 0, 0, 255)
                    //                 .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                    //                 .w_h(TILE_WIDTH, TILE_WIDTH);
                    //         }
                    //     }
                    // }

                    for x in max(0.0, (bomb.x - radius) as f32)
                        ..=min((self.map.w - 1) as f32, (bomb.x + radius) as f32)
                    {
                        for y in max(0.0, (bomb.y - ((bomb.x - x).abs() - radius).abs()) as f32)
                            ..=min(
                                (self.map.h - 1) as f32,
                                (bomb.y + ((bomb.x - x).abs() - radius).abs()) as f32,
                            )
                        {
                            let (vx, vy) = (self.cam.x - x as f32, self.cam.y - y as f32);

                            draw.rect()
                                .rgba8(255, 0, 0, 255)
                                .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                                .w_h(TILE_WIDTH, TILE_WIDTH);
                        }
                    }
                }
                n @ 20.. => {
                    let (vx, vy) = (self.cam.x - x as f32, self.cam.y - y as f32);

                    let elapsed = self.frame - bomb.start;

                    let v = (3.1415 * elapsed.pow(2) as f32 / 3600.0).cos().abs();

                    draw.rect()
                        .rgba8(255, (v * 255.0) as u8, (v * 255.0) as u8, 255)
                        .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                        .w_h(TILE_WIDTH, TILE_WIDTH);
                }
                _ => {}
            }
        }

        for turret in &self.map.turrets {
            let (vx, vy) = (self.cam.x - turret.x as f32, self.cam.y - turret.y as f32);

            let (cx, cy) = (-(vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round());

            let p = (self.frame - turret.last) as f32 / turret.cd as f32;
            //let v = (3.1415 * p).cos().abs();
            //let c = (v * 128.0) as u8;

            // TODO: optimize so that it draws one rect instead of 2 with alpha
            draw.rect()
                .rgba8(0, 0, 255, 255)
                .x_y(cx, cy)
                .w_h(TILE_WIDTH, TILE_WIDTH);

            draw.rect()
                .rgba8(255, 0, 0, ((p - 0.5) * 255.0 * 2.0).clamp(0.0, 255.0) as u8)
                .x_y(cx, cy)
                .w_h(TILE_WIDTH, TILE_WIDTH);

            // let pp = (3.0 * p * 255.0).clamp(0.0, 255.0) as u8;
            // draw.rect()
            // .rgba8(255, 0, 0, pp)
            // .x_y(cx, cy)
            // .w_h(TILE_WIDTH * 0.2, TILE_WIDTH * 0.5);

            // let pp = (3.0 * (p - 0.33) * 255.0).clamp(0.0, 255.0) as u8;
            // draw.rect()
            // .rgba8(255, 0, 0, pp)
            // .x_y(cx - TILE_WIDTH * 0.2, cy)
            // .w_h(TILE_WIDTH * v, TILE_WIDTH * v);

            // let pp = (3.0 * (p - 0.66) * 255.0).clamp(0.0, 255.0) as u8;
            // draw.rect()
            // .rgba8(255, 0, 0, pp)
            // .x_y(cx - TILE_WIDTH * 0.4, cy)
            // .w_h(TILE_WIDTH * v, TILE_WIDTH * v);

            // draw.rect()
            //     .rgba8(255, 255, 255, 255)
            //     .x_y(cx, cy)
            //     .w_h((TILE_WIDTH * 0.5).round(), (TILE_WIDTH * 0.5).round());

            // let (dx, dy) = [(-1.0, 0.0), (0.0, 1.0), (1.0, 0.0), (0.0, -1.0)][turret.direction];

            // draw.rect()
            //     .rgba8(255, 255, 255, 255)
            //     .x_y(
            //         (cx + dx * TILE_WIDTH * 0.375).round(),
            //         (cy + dy * TILE_WIDTH * 0.375).round(),
            //     )
            //     .w_h((TILE_WIDTH * 0.25).round(), (TILE_WIDTH * 0.25).round());

            // draw
            // .translate(Vec3::new(cx, cy + 4.0, 0.0))
            // //.rotate(3.14159 * 0.5)
            // .text("<")
            // .font(state.font.clone())
            // .font_size(19 * 2)
            // .rgba8(255, 255, 255, 255)
            // .align_text_middle_y()
            // .center_justify()
            // .no_line_wrap();

            // draw.polyline()
            // .weight(3.0)
            // .points([
            //     (cx + TILE_WIDTH * 0.15, cy + TILE_WIDTH * 0.3),
            //     (cx - TILE_WIDTH * 0.15, cy),
            //     (cx + TILE_WIDTH * 0.15, cy - TILE_WIDTH * 0.3)
            // ])
            // .rgba8(255, 255, 255, 255);

            // for k in 0..3 {
            //     let off = 0.3 * TILE_WIDTH - k as f32 * TILE_WIDTH * 0.3;
            //     let size = 0.2 + k as f32 * 0.1;
            //     draw.polyline()
            //     .weight(3.0)
            //     .points([
            //         (cx + TILE_WIDTH * size + off, cy + TILE_WIDTH * size),
            //         (cx + off, cy),
            //         (cx + TILE_WIDTH * size + off, cy - TILE_WIDTH * size)
            //     ])
            //     .rgba8(255, 255, 255, 255);
            // }
        }

        for bullet in &self.map.bullets {
            let (vx, vy) = (self.cam.x - bullet.x as f32, self.cam.y - bullet.y as f32);

            let g = std::cmp::min((self.frame - bullet.start) * 2, 255) as u8;

            draw.rect()
                .rgba8(255, g, 0, 255)
                .x_y(-(vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                .w_h(TILE_WIDTH, TILE_WIDTH);
        }

        for emp in &self.map.emps {
            //let (x, y) = (emp.x, emp.y);
            // let radius = (emp.duration - (emp.start + emp.duration - self.frame)) * emp.radius
            //     / emp.duration;

            let q = (3.1415 * (self.frame - emp.start) as f32 / emp.duration as f32).sin();

            // TODO: optimize this shit ngl
            for k in 0..(7 * emp.radius) {
                let angle = 2.0 * 3.1415 * k as f32 / (7 * emp.radius) as f32
                    + (self.frame - emp.start) as f32 / 10.0;

                let (x, y) = (
                    emp.x as f32 + emp.radius as f32 * angle.cos(),
                    emp.y as f32 + emp.radius as f32 * angle.sin(),
                );
                let (vx, vy) = (self.cam.x - x.round(), self.cam.y - y.round());

                draw.rect()
                    .rgba8(
                        0x89,
                        0xD1,
                        0xFE,
                        ((8.0 * 3.1415 * k as f32 / (7 * emp.radius) as f32).sin() * 255.0 * q)
                            as u8,
                    )
                    .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
                    .w_h(TILE_WIDTH, TILE_WIDTH);
            }

            // for z in
            //     max(0.0, (x - radius) as f32)..=min((self.map.w - 1) as f32, (x + radius) as f32)
            // {
            //     for t in max(0.0, (y - radius) as f32)
            //         ..=min((self.map.h - 1) as f32, (y + radius) as f32)
            //     {
            //         if dist(x, y, z, t) == radius {
            //             let (vx, vy) = (self.cam.x - z as f32, self.cam.y - t as f32);

            //             draw.rect()
            //                 .rgba8(0x89, 0xD1, 0xFE, 255)
            //                 .x_y((-vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
            //                 .w_h(TILE_WIDTH, TILE_WIDTH);
            //         }
            //     }
            // }
        }

        let (vx, vy) = (
            self.cam.x - self.player.x as f32,
            self.cam.y - self.player.y as f32,
        );

        draw.rect()
            .rgba8(255, 255, 255, 255)
            .x_y(-(vx * TILE_WIDTH).round(), (vy * TILE_WIDTH).round())
            .w_h(TILE_WIDTH, TILE_WIDTH);

        if let Some(ref pos) = self.map.boss.pos {
            let mut dx = (pos.x - self.player.x) as f32;
            let mut dy = (pos.y - self.player.y) as f32;
            let angle = (-dy).atan2(dx);

            let square = |angle: f32| {
                let len = if (angle > -PI * 0.25 && angle <= PI * 0.25)
                    || (angle > PI * 0.75 || angle < -PI * 0.75)
                {
                    angle.cos().abs().inv()
                } else {
                    angle.sin().abs().inv()
                };

                draw.rect()
                    .rgba8(0, 255, 0, 255)
                    .x_y(
                        (-vx * TILE_WIDTH
                            + ((TILE_WIDTH - 8.0) * 0.5 * angle.cos() * len)
                                .div(8.0)
                                .floor()
                                * 8.0
                            + 4.0)
                            .round(),
                        (vy * TILE_WIDTH
                            + ((TILE_WIDTH - 8.0) * 0.5 * angle.sin() * len)
                                .div(8.0)
                                .floor()
                                * 8.0
                            + 4.0)
                            .round(),
                    )
                    .w_h(8.0, 8.0);
            };

            square(angle - 0.2);
            square(angle);
            square(angle + 0.2);
        }

        self.draw_stats(app, draw, state);

        if self.player.dead {
            draw.pixtext(
                "You are DEAD",
                [0.0, state.h * 0.5 - 182.0],
                19 * 4,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(128, 0, 0, 255);

            // draw.pixtext(
            //     "press SPACE to continue",
            //     [0.0, -state.h * 0.5 + 96.0],
            //     19 * 2,
            //     (0, 0),
            //     state.font.clone(),
            // )
            // .rgba8(255, 255, 255, 255);
        } else if let Some(ref upgrading) = self.player.upgrading {
            upgrading.render(app, draw, state);
        }
    }
    pub fn update(&mut self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        let score = self.player[Score];
        let food = self.player[Food];

        if let Some(next) = self.next.take() {
            return Some(next);
        }

        match (
            self.player.paused,
            self.player.dead,
            &mut self.player.upgrading,
        ) {
            (true, true, _) => {
                return None;
            }
            (true, false, Some(ref mut upgrading)) => {
                upgrading.update(app, state);
                if let Some(ret) = upgrading.ret {
                    ret.apply(&mut self.player);
                    self.player.paused = false;
                    self.player.upgrading = None;
                }

                return None;
            }
            _ => {}
        }

        if state.keys[state.storage.hotkeys[Hotkey::Shoot as usize] as usize] {
            check_shot(
                &self.movement,
                &mut self.player,
                &mut self.map.bullets,
                &state,
                self.frame,
            );
        }

        let can_move = !state.keys[state.storage.hotkeys[Hotkey::Shoot as usize] as usize]
            && !state.keys[state.storage.hotkeys[Hotkey::Turret as usize] as usize];
        if can_move {
            make_moves(
                &mut self.player,
                &self.movement,
                &mut self.map,
                &state,
                self.frame,
            );
        }

        update_turrets(&mut self.map, &mut self.player, state, self.frame);

        update_bullets(&mut self.map, self.frame);

        check_bullets(&mut self.map, &mut self.player, state, self.frame);

        update_bombs(&mut self.map, &mut self.player, state, self.frame);

        //update_emps(&mut self.map, self.frame);
        if self.frame % 30 == 0 {
            spawn_random_chest(&mut self.map, &self.player, self.frame);
        }
        if self.frame % 60 == 0 {
            self.player[Score] += 1;
            spawn_enemy(&mut self.map, &self.player);
        }
        if self.player[Score] >= 200 + 400 * self.map.boss.nth && self.map.boss.pos.is_none() {
            let hp = 3 + self.map.boss.nth as i8;
            let (x, y) = spawn_boss(&mut self.map, &self.player, state, self.frame, hp);
            self.map.boss.nth += 1;
            self.map.boss.pos = Some(Position { x, y });
            self.map.boss.resp = Some(self.frame);
        }
        update_enemies(&mut self.map, &mut self.player, state, self.frame);

        check_bullets(&mut self.map, &mut self.player, state, self.frame);

        // remove chest if too old :)
        if let Some(&(x, y)) = self.map.chests.get(0) {
            if let Tile::Chest(ref chest) = self.map.tiles[(x, y)] {
                if self.frame >= chest.start + chest.duration {
                    self.map.pass[(x, y)] = true;
                    self.map.tiles[(x, y)] = Tile::None;
                    self.map.chests.remove(0);
                }
            }
        }
        // remove emps
        let mut i = 0;
        while i < self.map.emps.len() {
            let emp = &mut self.map.emps[i];

            if self.frame >= emp.start + emp.duration {
                self.map.emps.swap_remove(i);
                continue;
            }
            i += 1;
        }

        match self.ambient {
            Some(ref s) if s.len() == 0 => {
                self.ambient = Some(state.play_get(Sound::Ambient, 0.2));
            }
            _ => {}
        }

        // update enemies max hp
        if score / 1000 != self.player[Score] / 1000 {
            let mut mhp = 1 + (self.player[Score] + 1) / 1000;

            for x in 1..self.map.w - 1 {
                for y in 1..self.map.h - 1 {
                    if let Some(ref mut enemy) = self.map.enemies[(x, y)] {
                        enemy.mhp = mhp as i8;
                    }
                }
            }
        }

        let (px, py) = (self.player.x, self.player.y);

        // check if there is enemy next to player
        if self.map.enemies[(px - 1, py)].is_some()
            || self.map.enemies[(px + 1, py)].is_some()
            || self.map.enemies[(px, py - 1)].is_some()
            || self.map.enemies[(px, py + 1)].is_some()
        {
            self.player.dead = true;
        }

        self.cam.x += (px as f32 - self.cam.x) * 0.05;
        self.cam.y += (py as f32 - self.cam.y) * 0.05;

        self.frame += 1;
        self.player[Food] -= 85;

        if self.player[Food] <= 0 {
            self.player.dead = true;
        }

        if self.player.dead {

            state.play(Sound::Defeat, 0.5);

            self.player.paused = true;

            //check achievements
            let (score, killed, used, mut save) = (
                self.player[Score],
                self.player.progress.killed,
                self.player.progress.used,
                false,
            );

            if self.player[Score] > state.storage.highscore {
                state.storage.highscore = self.player[Score];
                save = true;
            }

            if score >= 1000
                && matches!(used, [0, 1..=i32::MAX, 0, 0])
                && state.storage.achievements[&Achievement::UnlockAnne].state == false
            {
                // annie
                state
                .storage
                .achievements
                .insert(Achievement::UnlockAnne, AchievementState { state: true });

                save = true;
            }
            if killed >= 150
                && state.storage.achievements[&Achievement::UnlockAndrew].state == false
            {
                // andrew
                state
                .storage
                .achievements
                .insert(Achievement::UnlockAndrew, AchievementState { state: true });

                save = true;
            }
            if killed >= 50
                && matches!(used, [0, 0, 1..=i32::MAX, 0])
                && state.storage.achievements[&Achievement::UnlockMatthew].state == false
            {
                // matthew
                state
                .storage
                .achievements
                .insert(Achievement::UnlockMatthew, AchievementState { state: true });

                save = true;
            }
            if score >= 3000 && state.storage.achievements[&Achievement::UnlockMegan].state == false
            {
                // megan
                state
                .storage
                .achievements
                .insert(Achievement::UnlockMegan, AchievementState { state: true });

                save = true;
            }
            if score >= 1000
                && killed == 0
                && state.storage.achievements[&Achievement::UnlockLiShen].state == false
            {
                // li shen
                state
                .storage
                .achievements
                .insert(Achievement::UnlockLiShen, AchievementState { state: true });

                save = true;
            }

            if save {
                state.storage.save();
            }
        }

        None
    }
    pub fn pressed(&mut self, app: &App, state: &mut State, key: Key) {

        if key == Key::Escape {
            self.next = Some(Box::new(Screen::Menu(Menu::new(state))));
            return;
        }

        match (self.player.paused, &mut self.player.upgrading) {
            (_, Some(ref mut upgrading)) => {
                upgrading.pressed(app, state, key);
                return;
            }
            (true, _) => return,
            _ => {}
        }

        if (key as u32 == state.storage.hotkeys[Hotkey::Up as usize]
            || key as u32 == state.storage.hotkeys[Hotkey::Right as usize]
            || key as u32 == state.storage.hotkeys[Hotkey::Down as usize]
            || key as u32 == state.storage.hotkeys[Hotkey::Left as usize])
            && !self.movement.contains(&key)
        {
            self.movement.push(key);

            if state.keys[state.storage.hotkeys[Hotkey::Turret as usize] as usize] {
                check_turret(
                    &self.movement,
                    &mut self.player,
                    &mut self.map,
                    state,
                    self.frame,
                );
            }

            return;
        }

        match key {
            _ if key as u32 == state.storage.hotkeys[Hotkey::Run as usize] => {
                self.player.running = true
            }
            _ if key as u32 == state.storage.hotkeys[Hotkey::Bomb as usize] => {
                check_bomb(&mut self.player, &mut self.map, state, self.frame)
            }
            _ if key as u32 == state.storage.hotkeys[Hotkey::Emp as usize] => {
                check_emp(&mut self.player, &mut self.map, state, self.frame)
            }
            _ => {}
        }
    }
    pub fn released(&mut self, app: &App, state: &mut State, key: Key) {
        if let Some((idx, _)) = self.movement.iter().enumerate().find(|&(_i, &k)| k == key) {
            self.movement.remove(idx);
        }

        if self.player.paused {
            return;
        }

        match key {
            _ if key as u32 == state.storage.hotkeys[Hotkey::Run as usize] => {
                self.player.running = false
            }
            _ if key as u32 == state.storage.hotkeys[Hotkey::Turret as usize] => check_turret(
                &self.movement,
                &mut self.player,
                &mut self.map,
                state,
                self.frame,
            ),
            _ => {}
        }
    }

    pub fn draw_stats(&self, app: &App, draw: &Draw, state: &State) {
        let SIZE: u32 = 38;
        let BAR_WIDTH: f32 = 200.0;
        let BAR_HEIGHT: f32 = 24.0;
        let OFFSET: f32 = 12.0;

        let bar = |draw: &Draw, [x, y, w, h]: [f32; 4], p: f32, color: Rgba8| {
            draw.rect().x_y(x, y).w_h(w, h).rgba8(39, 39, 39, 255);

            draw.rect()
                .x_y((x - w * (0.5 - p * 0.5)).round(), y)
                .w_h((w * p).round(), h)
                .color(color);
        };

        let sbar =
            |draw: &Draw, [x, y, w, h]: [f32; 4], p: f32, color: Rgba8, bt: &str, ut: &str| {
                bar(&draw, [x, y, w, h], p, color);

                draw.pixtext(
                    bt,
                    [x, y + h * 0.5 + 24.0],
                    19 * 2,
                    (0, 0),
                    state.font.clone(),
                );
                draw.pixtext(
                    ut,
                    [x, y + h * 0.5 + 24.0 + 36.0],
                    19,
                    (0, 0),
                    state.font.clone(),
                );
            };

        sbar(
            &draw,
            [
                -state.w * 0.5 + BAR_WIDTH * 0.5 + OFFSET,
                -state.h * 0.5 + BAR_HEIGHT * 0.5 + OFFSET,
                BAR_WIDTH,
                BAR_HEIGHT,
            ],
            self.player[Ammo] as f32 / self.player[MaxAmmo] as f32,
            Rgba8::new(255, 165, 0, 255),
            &*format!("{}/{}", self.player[Ammo], self.player[MaxAmmo]),
            "AMMO",
        );

        sbar(
            &draw,
            [
                -state.w * 0.5 + BAR_WIDTH * 1.5 + OFFSET * 2.0,
                -state.h * 0.5 + BAR_HEIGHT * 0.5 + OFFSET,
                BAR_WIDTH,
                BAR_HEIGHT,
            ],
            self.player[Bomb] as f32 / self.player[MaxBomb] as f32,
            Rgba8::new(255, 0, 0, 255),
            &*format!("{}/{}", self.player[Bomb], self.player[MaxBomb]),
            "BOMB",
        );

        sbar(
            &draw,
            [
                state.w * 0.5 - BAR_WIDTH * 1.5 - OFFSET * 2.0,
                -state.h * 0.5 + BAR_HEIGHT * 0.5 + OFFSET,
                BAR_WIDTH,
                BAR_HEIGHT,
            ],
            self.player[Turret] as f32 / self.player[MaxTurret] as f32,
            Rgba8::new(0, 0, 255, 255),
            &*format!("{}/{}", self.player[Turret], self.player[MaxTurret]),
            "TURRET",
        );

        sbar(
            &draw,
            [
                state.w * 0.5 - BAR_WIDTH * 0.5 - OFFSET,
                -state.h * 0.5 + BAR_HEIGHT * 0.5 + OFFSET,
                BAR_WIDTH,
                BAR_HEIGHT,
            ],
            self.player[Emp] as f32 / self.player[MaxEmp] as f32,
            Rgba8::new(128, 0, 128, 255),
            &*format!("{}/{}", self.player[Emp], self.player[MaxEmp]),
            "EMP",
        );

        bar(
            &draw,
            [
                state.w * 0.5 - BAR_WIDTH * 1.0 - OFFSET * 1.5,
                state.h * 0.5 - BAR_HEIGHT * 0.5 - OFFSET,
                BAR_WIDTH * 2.0 + OFFSET,
                BAR_HEIGHT,
            ],
            self.player[Food] as f32 / self.player[MaxFood] as f32,
            Rgba8::new(64, 224, 208, 255),
        );

        draw.pixtext(
            "FOOD",
            [
                state.w * 0.5 - BAR_WIDTH * 1.0 - OFFSET * 1.5,
                state.h * 0.5 - BAR_HEIGHT * 1.0 - OFFSET - 24.0,
            ],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, 255);

        {
            draw.pixtext(
                format!("{}", self.player[Score]),
                [
                    -state.w * 0.5 + BAR_WIDTH * 0.5 + OFFSET,
                    state.h * 0.5 - (BAR_WIDTH * 0.5 + OFFSET) + 32.0,
                ],
                38,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, 255);

            draw.pixtext(
                "SCORE",
                [
                    -state.w * 0.5 + BAR_WIDTH * 0.5 + OFFSET,
                    state.h * 0.5 - (BAR_WIDTH * 0.5 + OFFSET),
                ],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, 255);
        }
    }
}
//let lc = (-1920.0 / 2.0, 1080.0 / 2.0); // left upper corner

// draw.text(&format!("Score: {}", player[Score]))
//     .font(state.font.clone())
//     .font_size(FONT_SIZE)
//     .x_y(lc.0 + 120.0, lc.1 - 100.0);

// let ratio = player[Food] as f32 / player[MaxFood] as f32;
// draw.text(&format!("Food: {}%", (ratio * 100.0) as i32))
//     .font(state.font.clone())
//     .font_size(FONT_SIZE)
//     .x_y(lc.0 + 120.0, lc.1 - START);

// let width = ratio * 300.0;
// draw.rect()
//     .color(TURQUOISE)
//     .x_y(lc.0 + width / 2.0, lc.1 - START - 50.0)
//     .w_h(width, 30.0);

// let stats_info = [
//     ("Ammo", Ammo, MaxAmmo, ORANGE),
//     ("Bombs", Bomb, MaxBomb, RED),
//     ("Turrets", Turret, MaxTurret, BLUE),
//     ("Emps", Emp, MaxEmp, PURPLE),
// ];
// for (i, (str_stat, stat, stat_max, color)) in stats_info.into_iter().enumerate() {
//     let ratio = player[stat.clone()] as f32 / player[stat_max.clone()] as f32;

//     draw.text(&format!(
//         "{str_stat}: {}/{}",
//         player[stat], player[stat_max]
//     ))
//     .font(state.font.clone())
//     .font_size(FONT_SIZE)
//     .x_y(lc.0 + 120.0, lc.1 - START - 150.0 - 150.0 * i as f32);

//     let width = ratio * 300.0;
//     draw.rect()
//         .color(color)
//         .x_y(
//             lc.0 + width / 2.0,
//             lc.1 - START - 150.0 - 50.0 - 150.0 * i as f32,
//         )
//         .w_h(width, 30.0);
//}
