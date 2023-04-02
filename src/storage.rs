use super::*;

const KEY: u8 = 0b11001010;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub highscore: i32,
    pub hotkeys: Vec<u32>,
    pub achievements: HashMap<Achievement, AchievementState>,
}

impl Storage {
    pub fn new() -> Self {
        Storage::load().unwrap_or_default()
    }
    pub fn load() -> Option<Self> {
        let mut raw = std::fs::read("storage").ok()?;

        for k in 0..raw.len() {
            raw[k] ^= KEY;
        }

        let decrypted = String::from_utf8(raw).ok()?;

        let parsed = json::from_str(&*decrypted).ok()?;
        Some(parsed)
    }
    #[must_use]
    pub fn save(&self) -> std::io::Result<()> {
        let data = json::to_string(self)?;

        let encrypted = data
        .as_bytes()
        .iter()
        .map(|&b| b ^ KEY)
        .collect::<Vec<u8>>();

        std::fs::write("storage", encrypted)?;
        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        use Achievement::*;
        let achievements = HashMap::from([
            (Tutorial, AchievementState::default()),
            (UnlockAnne, AchievementState::default()),
            (UnlockAndrew, AchievementState::default()),
            (UnlockMatthew, AchievementState::default()),
            (UnlockMegan, AchievementState::default()),
            (UnlockLiShen, AchievementState::default()),
        ]);
        Storage {
            highscore: 0,
            hotkeys: vec![
                Key::Up as u32,
                Key::Left as u32,
                Key::Down as u32,
                Key::Right as u32,
                Key::LControl as u32,
                Key::Q as u32,
                Key::E as u32,
                Key::R as u32,
                Key::W as u32,
            ],
            achievements,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Achievement {
    Tutorial,
    UnlockAnne,
    UnlockAndrew,
    UnlockMatthew,
    UnlockMegan,
    UnlockLiShen,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AchievementState {
    pub state: bool,
}
