use super::*;

pub struct Context {
    pub state: State,
    pub frames: usize,
    pub last: Instant,
    pub screen: Screen,
}

impl Context {
    pub fn run() {
        nannou::app(Context::new)
            //.loop_mode(LoopMode::rate_fps(60.0))
            .view(Context::render)
            .update(Context::update)
            .run();
    }
    pub fn new(app: &App) -> Self {
        app.new_window()
            .size(0, 0)
            .decorations(false)
            .title("That Night")
            .key_pressed(Context::pressed)
            .key_released(Context::released)
            .mouse_pressed(Context::mpressed)
            .build()
            .expect("Couldn't create window");

        app.set_exit_on_escape(false);

        let (w, h) = app.main_window().inner_size_pixels();

        let (stream, sout) = OutputStream::try_default().unwrap();

        app.main_window().set_fullscreen(true);

        Self {
            state: State {
                frame: 0,
                storage: Storage::new(),
                keys: [false; 256],
                font: font(),
                w: w as f32,
                h: h as f32,
                stream,
                sout,
                sounds: sounds(),
            },
            frames: 0,
            last: Instant::now(),
            screen: Screen::Loading(Loading::new()),
        }
    }
    pub fn render(app: &App, ctx: &Self, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);

        match ctx.screen {
            Screen::Loading(ref screen) => {
                screen.render(app, &draw, &ctx.state);
            }
            Screen::Menu(ref screen) => {
                screen.render(app, &draw, &ctx.state);
            }
            Screen::Tutorial(ref screen) => {
                screen.render(app, &draw, &ctx.state);
            }
            Screen::Playing(ref screen) => {
                screen.render(app, &draw, &ctx.state);
            }
            Screen::Settings(ref screen) => {
                screen.render(app, &draw, &ctx.state);
            }
            _ => {}
        }

        draw.to_frame(app, &frame).unwrap();
    }
    pub fn update(app: &App, ctx: &mut Self, _update: Update) {
        ctx.frames += 1;
        //let mut dt = ctx.last.elapsed().as_millis();
        // if dt >= 1000 {
        //     println!("{}", ctx.frames);
        //     ctx.frames = 0;
        //     ctx.last = Instant::now();
        // }
        let mut dt = ctx.last.elapsed().as_millis();
        while dt < 16 {
            dt = ctx.last.elapsed().as_millis();
        }

        let (w, h) = app.main_window().inner_size_pixels();

        ctx.state.w = w as f32;
        ctx.state.h = h as f32;

        if let Some(new) = match ctx.screen {
            Screen::Loading(ref mut screen) => screen.update(app, &mut ctx.state),
            Screen::Menu(ref mut screen) => screen.update(app, &mut ctx.state),
            Screen::Tutorial(ref mut screen) => screen.update(app, &mut ctx.state),
            Screen::Playing(ref mut screen) => screen.update(app, &mut ctx.state),
            Screen::Settings(ref mut screen) => screen.update(app, &mut ctx.state),
            _ => None,
        } {
            ctx.screen = *new;
        }

        ctx.last = Instant::now();

        ctx.state.frame += 1;
    }
    pub fn pressed(app: &App, ctx: &mut Self, k: Key) {
        ctx.state.keys[k as usize] = true;

        match ctx.screen {
            Screen::Menu(ref mut screen) => {
                screen.pressed(app, &mut ctx.state, k);
            }
            Screen::Tutorial(ref mut screen) => {
                screen.pressed(app, &mut ctx.state, k);
            }
            Screen::Playing(ref mut screen) => {
                screen.pressed(app, &mut ctx.state, k);
            }
            Screen::Settings(ref mut screen) => {
                screen.pressed(app, &mut ctx.state, k);
            }
            _ => {}
        }
    }
    pub fn released(app: &App, ctx: &mut Self, k: Key) {
        ctx.state.keys[k as usize] = false;

        match ctx.screen {
            Screen::Menu(ref mut screen) => {
                screen.released(app, &mut ctx.state, k);
            }
            Screen::Playing(ref mut screen) => {
                screen.released(app, &mut ctx.state, k);
            }
            _ => {}
        }
    }
    pub fn mpressed(app: &App, ctx: &mut Self, mb: MouseButton) {
        match ctx.screen {
            Screen::Menu(ref mut screen) => {
                screen.mpressed(app, &mut ctx.state, mb);
            }
            Screen::Settings(ref mut screen) => {
                screen.mpressed(app, &mut ctx.state, mb);
            }
            _ => {}
        }
    }
}

pub struct State {
    frame: i32,
    pub storage: Storage,
    pub keys: [bool; 256],
    pub font: Font,
    pub w: f32,
    pub h: f32,
    stream: OutputStream,
    sout: OutputStreamHandle,
    sounds: Vec<Cursor<Vec<u8>>>,
}

impl State {
    pub fn play(&self, sound: Sound, v: f32) {
        let s = self
            .sout
            .play_once(self.sounds[sound as usize].clone())
            .expect("Can't play sound");
        s.set_volume(v);
        s.detach();
    }
    pub fn play_get(&self, sound: Sound, v: f32) -> Sink {
        let s = self
            .sout
            .play_once(self.sounds[sound as usize].clone())
            .expect("Can't play sound");
        s.set_volume(v);
        s
    }
}

#[repr(u8)]
pub enum Screen {
    Loading(Loading),
    Menu(Menu),
    Tutorial(Tutorial),
    Playing(Playing),
    Settings(Settings),
    Defeat(Defeat),
}

pub const SOUNDS: [Sound; 19] = [
    Sound::Intro,
    Sound::Ambient,
    Sound::UiSwitch,
    Sound::LowFood,
    Sound::UseAmmo,
    Sound::UseBomb,
    Sound::UseTurret,
    Sound::UseEmp,
    Sound::PickChest,
    Sound::BombExplosion,
    Sound::BombTick,
    Sound::TurretShoot,
    Sound::ZombieHit,
    Sound::ZombieDeath,
    Sound::BossAppear,
    Sound::Upgrade,
    Sound::Defeat,
    Sound::Walking,
    Sound::Running,
];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Sound {
    Intro,
    Ambient,

    UiSwitch,

    LowFood,

    UseAmmo,
    UseBomb,
    UseTurret,
    UseEmp,

    PickChest,

    BombExplosion,
    BombTick,
    TurretShoot,

    ZombieHit,
    ZombieDeath,

    BossAppear,

    Upgrade,
    Defeat,

    Walking,
    Running,
}

pub trait SinkExt {
    fn fade(self, dur: u64);
}

impl SinkExt for Sink {
    fn fade(self, dur: u64) {
        spawn(move || {
            let current = self.volume();
            let inv = 1.0 / (dur / 20) as f32;
            for k in 0..dur / 20 {
                self.set_volume(current - k as f32 * inv * current);
                sleep(Duration::from_millis(20));
            }
        });
    }
}

fn font() -> Font {
    Font::from_bytes(include_bytes!("../assets/m5x7.ttf")).expect("Invalid font format")
}

fn sounds() -> Vec<Cursor<Vec<u8>>> {
    let mut r = vec![];
    for bytes in [
        &include_bytes!("../assets/Intro.mp3")[..],
        &include_bytes!("../assets/Ambient.mp3")[..],
        &include_bytes!("../assets/UiSwitch.wav")[..],
        &include_bytes!("../assets/LowFood.mp3")[..],
        &include_bytes!("../assets/UseAmmo.mp3")[..],
        &include_bytes!("../assets/UseBomb.wav")[..],
        &include_bytes!("../assets/UseTurret.wav")[..],
        &include_bytes!("../assets/UseEmp.wav")[..],
        &include_bytes!("../assets/PickChest.wav")[..],
        &include_bytes!("../assets/BombExplosion.wav")[..],
        &include_bytes!("../assets/BombTick.mp3")[..],
        &include_bytes!("../assets/TurretShoot.mp3")[..],
        &include_bytes!("../assets/ZombieHit.wav")[..],
        &include_bytes!("../assets/ZombieDeath.wav")[..],
        &include_bytes!("../assets/BossAppear.wav")[..],
        &include_bytes!("../assets/Upgrade.wav")[..],
        &include_bytes!("../assets/Defeat.wav")[..],
        &include_bytes!("../assets/Walking.mp3")[..],
        &include_bytes!("../assets/Running.mp3")[..],
    ] {
        r.push(Cursor::new(Vec::from(&bytes[..])));
    }
    r
}
