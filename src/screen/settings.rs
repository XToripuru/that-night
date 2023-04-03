use super::*;

static DESC: &[&str] = &[
    "Up", "Left", "Down", "Right", "Shoot", "Bomb", "Turret", "Emp", "Run",
];

pub struct Settings {
    n: usize,
    choosing: bool,
    next: Option<Box<Screen>>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            n: 0,
            choosing: false,
            next: None,
        }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        // let p = self.frame as f32 / 60.0;

        let m = app.mouse.position();

        for k in 0..state.storage.hotkeys.len() {
            let [r, g, b, a] = if k == self.n {
                if self.choosing {
                    [255, 0, 0, 255]
                } else {
                    [255, 255, 255, 255]
                }
            } else {
                [128, 128, 128, 255]
            };

            draw.pixtext(
                DESC[k],
                [-128.0, state.h * 0.25 - 64.0 * k as f32],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(r, g, b, a);

            draw.pixtext(
                format!("{:?}", KEYS[state.storage.hotkeys[k] as usize]),
                [128.0, state.h * 0.25 - 64.0 * k as f32],
                19 * 2,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(r, g, b, a);
        }

        draw.pixtext(
            "Settings",
            [0.0, state.h * 0.5 - 96.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        draw.pixtext(
            "press SPACE to select",
            [0.0, -state.h * 0.5 + 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, 255);

        draw.pixtext(
            "press S to close settings",
            [-state.w * 0.5 + 156.0, -state.h * 0.5 + 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, 255);
    }
    pub fn update(&mut self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        self.next.take()
    }
    pub fn pressed(&mut self, app: &App, state: &mut State, k: Key) {
        match k {
            k @ _ if self.choosing => {
                state.play(Sound::UiSwitch, 0.75);
                state.storage.hotkeys[self.n] = k as u32;
                self.choosing = false;
            }
            Key::Up if self.n > 0 => {
                state.play(Sound::UiSwitch, 0.75);
                self.n -= 1;
            }
            Key::Down if self.n < state.storage.hotkeys.len() - 1 => {
                state.play(Sound::UiSwitch, 0.75);
                self.n += 1;
            }
            Key::Space => {
                state.play(Sound::UiSwitch, 0.75);
                self.choosing = true;
            }
            Key::Escape | Key::S => {
                state.play(Sound::UiSwitch, 0.75);
                state.storage.save();
                self.next = Some(Box::new(Screen::Menu(Menu::new(state))));
            }
            _ => {}
        }
    }
    pub fn mpressed(&mut self, app: &App, state: &mut State, mb: MouseButton) {
        let m = app.mouse.position();

        if m.x < -state.w * 0.5 + 164.0 && m.y < -state.h * 0.5 + 32.0 {
            state.storage.save();
            self.next = Some(Box::new(Screen::Menu(Menu::new(state))));
        }
    }
}

#[repr(u8)]
pub enum Hotkey {
    Up,
    Left,
    Down,
    Right,

    Shoot,
    Bomb,
    Turret,
    Emp,

    Run,
}

pub const KEYS: [Key; 163] = [
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
    Key::Key9,
    Key::Key0,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::Escape,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::F11,
    Key::F12,
    Key::F13,
    Key::F14,
    Key::F15,
    Key::F16,
    Key::F17,
    Key::F18,
    Key::F19,
    Key::F20,
    Key::F21,
    Key::F22,
    Key::F23,
    Key::F24,
    Key::Snapshot,
    Key::Scroll,
    Key::Pause,
    Key::Insert,
    Key::Home,
    Key::Delete,
    Key::End,
    Key::PageDown,
    Key::PageUp,
    Key::Left,
    Key::Up,
    Key::Right,
    Key::Down,
    Key::Back,
    Key::Return,
    Key::Space,
    Key::Compose,
    Key::Caret,
    Key::Numlock,
    Key::Numpad0,
    Key::Numpad1,
    Key::Numpad2,
    Key::Numpad3,
    Key::Numpad4,
    Key::Numpad5,
    Key::Numpad6,
    Key::Numpad7,
    Key::Numpad8,
    Key::Numpad9,
    Key::NumpadAdd,
    Key::NumpadDivide,
    Key::NumpadDecimal,
    Key::NumpadComma,
    Key::NumpadEnter,
    Key::NumpadEquals,
    Key::NumpadMultiply,
    Key::NumpadSubtract,
    Key::AbntC1,
    Key::AbntC2,
    Key::Apostrophe,
    Key::Apps,
    Key::Asterisk,
    Key::At,
    Key::Ax,
    Key::Backslash,
    Key::Calculator,
    Key::Capital,
    Key::Colon,
    Key::Comma,
    Key::Convert,
    Key::Equals,
    Key::Grave,
    Key::Kana,
    Key::Kanji,
    Key::LAlt,
    Key::LBracket,
    Key::LControl,
    Key::LShift,
    Key::LWin,
    Key::Mail,
    Key::MediaSelect,
    Key::MediaStop,
    Key::Minus,
    Key::Mute,
    Key::MyComputer,
    Key::NavigateForward,
    Key::NavigateBackward,
    Key::NextTrack,
    Key::NoConvert,
    Key::OEM102,
    Key::Period,
    Key::PlayPause,
    Key::Plus,
    Key::Power,
    Key::PrevTrack,
    Key::RAlt,
    Key::RBracket,
    Key::RControl,
    Key::RShift,
    Key::RWin,
    Key::Semicolon,
    Key::Slash,
    Key::Sleep,
    Key::Stop,
    Key::Sysrq,
    Key::Tab,
    Key::Underline,
    Key::Unlabeled,
    Key::VolumeDown,
    Key::VolumeUp,
    Key::Wake,
    Key::WebBack,
    Key::WebFavorites,
    Key::WebForward,
    Key::WebHome,
    Key::WebRefresh,
    Key::WebSearch,
    Key::WebStop,
    Key::Yen,
    Key::Copy,
    Key::Paste,
    Key::Cut,
];
