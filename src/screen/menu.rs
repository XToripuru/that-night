use super::*;

static NAMES: &[&str] = &["Joshua", "Anne", "Andrew", "Matthew", "Megan", "Li-Shen"];

static DESC: &[&str] = &[
    "Olympic Runner",
    "Devil Pyromaniac",
    "Arms Dealer",
    "Hobbyist Engineer",
    "Scientist at NASA",
    "Monk of the Jade Temple",
];

static BONUSES: &[&[&str]] = &[
    &["increased movement speed"],
    &[
        "start with 1 max ammo",
        "+3 max bombs",
        "+1 bomb radius",
        "-1s bomb fuse time",
    ],
    &["+1 bullet damage", "+20% chance to not consume ammo"],
    &["doubled turret duration"],
    &["+3 EMP range", "EMP immobilizes for half the duration"],
    &["+50% max food", "increased luck"],
];

static ACHIEVEMENTS: &[&str] = &[
    "Do nothing",
    "Achieve 1000 score using only bombs",
    "Kill 150 enemies in one game",
    "Kill 50 enemies using only turrets in one game",
    "Achieve 3000 score",
    "Achieve 1000 score without killing any monsters",
];

pub struct Menu {
    frame: i32,
    chars: Vec<Character>,
    unlocked: Vec<bool>,
    achs: f32,
    chs: usize,
    intro: Option<Sink>,
    next: Option<Box<Screen>>,
}

impl Menu {
    pub fn new(state: &State) -> Self {
        let intro = state.play_get(Sound::Intro, 0.2);
        Self {
            frame: 0,
            chars: vec![
                Character::Joshua,
                Character::Anne,
                Character::Andrew,
                Character::Matthew,
                Character::Megan,
                Character::LiShen,
            ],
            unlocked: vec![
                true,
                state.storage.achievements[&Achievement::UnlockAnne].state,
                state.storage.achievements[&Achievement::UnlockAndrew].state,
                state.storage.achievements[&Achievement::UnlockMatthew].state,
                state.storage.achievements[&Achievement::UnlockMegan].state,
                state.storage.achievements[&Achievement::UnlockLiShen].state,
            ],
            achs: 0.0,
            chs: 0,
            intro: Some(intro),
            next: None,
        }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        let m = app.mouse.position();
        let alpha = 255 - ((self.frame as f32 / 40.0).sin().abs() * 200.0) as u8;
        draw.pixtext(
            "press SPACE to begin",
            [state.w * 0.5 - 132.0, -state.h * 0.5 + 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, alpha);

        draw.pixtext(
            "press S to open settings",
            [-state.w * 0.5 + 156.0, -state.h * 0.5 + 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, alpha);

        draw.pixtext(
            VERSION,
            [-state.w * 0.5 + 48.0, state.h * 0.5 - 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, 255);

        for k in 0..self.chars.len() {
            let d = (k as f32 - self.achs).abs();
            if d > 2.5 {
                continue;
            }

            let br = max(255.0 - d * 128.0, 128.0) as u8;

            draw.pixtext(
                NAMES[k],
                [0.0, 0.0 + (self.achs - k as f32) * state.h * 0.3],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(br, br, br, 255);
        }

        let a = self.achs.floor();
        let b = self.achs.ceil();
        let aidx = a as usize;
        let bidx = b as usize;
        let da = (self.achs - a).abs();
        let db = (self.achs - b).abs();
        let abr = (255.0 - da * 555.0).clamp(0.0, 255.0) as u8;
        let bbr = (255.0 - db * 555.0).clamp(0.0, 255.0) as u8;

        if !self.unlocked[aidx] {
            draw.pixtext(
                "LOCKED",
                [state.w * 0.25, 64.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, abr);

            draw.pixtext(
                "To unlock you must",
                [-state.w * 0.25, 0.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, abr);

            draw.pixtext(
                ACHIEVEMENTS[aidx],
                [-state.w * 0.25, -48.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, abr);
        } else {
            draw.pixtext(
                format!("{}", state.storage.highscore),
                [-state.w * 0.25, 0.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, abr);

            draw.pixtext(
                "HIGHSCORE",
                [-state.w * 0.25, -32.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, abr);
        }

        {
            let d = (-1 as f32 - self.achs).abs();

            let br = max(255.0 - d * 128.0, 128.0) as u8;

            draw.pixtext(
                "Select your identity",
                [0.0, 0.0 + (self.achs + 1 as f32) * state.h * 0.3],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(br, br, br, 255);
        }

        draw.pixtext(
            DESC[aidx],
            [state.w * 0.25, 0.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, abr);

        for k in 0..BONUSES[aidx].len() {
            draw.pixtext(
                BONUSES[aidx][k],
                [state.w * 0.25, -48.0 - k as f32 * 32.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, abr);
        }

        //

        if !self.unlocked[bidx] {
            draw.pixtext(
                "LOCKED",
                [state.w * 0.25, 64.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, bbr);

            draw.pixtext(
                "To unlock you must",
                [-state.w * 0.25, 0.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, bbr);

            draw.pixtext(
                ACHIEVEMENTS[bidx],
                [-state.w * 0.25, -48.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 0, 0, bbr);
        } else {
            draw.pixtext(
                format!("{}", state.storage.highscore),
                [-state.w * 0.25, 0.0],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, bbr);

            draw.pixtext(
                "HIGHSCORE",
                [-state.w * 0.25, -32.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, bbr);
        }

        draw.pixtext(
            DESC[bidx],
            [state.w * 0.25, 0.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, bbr);

        for k in 0..BONUSES[bidx].len() {
            draw.pixtext(
                BONUSES[bidx][k],
                [state.w * 0.25, -48.0 - k as f32 * 32.0],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(255, 255, 255, bbr);
        }

    }
    pub fn update(&mut self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        self.achs += (self.chs as f32 - self.achs) * 0.1;
        self.frame += 1;

        self.next.take()
    }
    pub fn pressed(&mut self, app: &App, state: &mut State, k: Key) {
        match k {
            Key::Up if self.chs > 0 => {
                state.play(Sound::UiSwitch, 0.75);
                self.chs -= 1;
            }
            Key::Down if self.chs < self.chars.len() - 1 => {
                state.play(Sound::UiSwitch, 0.75);
                self.chs += 1;
            }
            Key::Space if self.unlocked[self.chs] => {
                self.intro.take().unwrap().fade(2000);
                //state.play(Sound::UiSwitch, 1.0);
                let playing = Some(Box::new(Screen::Playing(Playing::new(
                    self.chars[self.chs],
                    state
                ))));
                if !state.storage.achievements[&Achievement::Tutorial].state {
                    self.next = Some(Box::new(Screen::Tutorial(Tutorial::new(playing))));
                } else {
                    self.next = playing;
                }
            }
            Key::S => {
                state.play(Sound::UiSwitch, 0.75);
                self.next = Some(Box::new(Screen::Settings(Settings::new())));
            }
            Key::Escape => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
    pub fn released(&mut self, app: &App, state: &mut State, k: Key) {}
    pub fn mpressed(&mut self, app: &App, state: &mut State, mb: MouseButton) {
        let m = app.mouse.position();

        if m.x < -state.w * 0.5 + 164.0 && m.y < -state.h * 0.5 + 32.0 {
            self.next = Some(Box::new(Screen::Settings(Settings::new())));
        }
    }
}
