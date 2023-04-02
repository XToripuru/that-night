use super::*;

pub struct Upgrading {
    frame: i32,
    ups: usize,
    upgrades: Vec<Upgrade>,
    pub ret: Option<Upgrade>,
}

impl Upgrading {
    pub fn new(player: &Player, state: &mut State) -> Self {
        state.play(Sound::Upgrade, 1.0);
        Self {
            frame: 0,
            ups: 2,
            upgrades: Upgrade::random(player),
            ret: None,
        }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        if self.frame <= 30 {
            let p = self.frame as f32 / 30.0;
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(state.w, state.h * 0.5 * p)
                .rgba8(0, 0, 0, 230);
            return;
        }

        let br = if self.frame < 60 {
            (255.0 * (self.frame - 30) as f32 / 30.0).clamp(0.0, 255.0) as u8
        } else {
            255
        };

        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(state.w, state.h * 0.5)
            .rgba8(0, 0, 0, 230);

        draw.pixtext(
            "Level up!",
            [0.0, (1 * 64 + 96) as f32],
            19 * 2,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, br);

        for k in 0..self.upgrades.len() {
            let [r, g, b, a] = [
                255,
                if k == self.ups { 0 } else { 255 },
                if k == self.ups { 0 } else { 255 },
                br,
            ];
            draw.pixtext(
                self.upgrades[k].text(),
                [0.0, ((1 - k as i32) * 64) as f32],
                19,
                (0, 0),
                state.font.clone(),
            )
            .rgba8(r, g, b, a);
        }
    }
    pub fn update(&mut self, app: &App, state: &mut State) {
        self.frame += 1;
    }
    pub fn pressed(&mut self, app: &App, state: &mut State, k: Key) {
        match k {
            Key::Up if self.ups > 0 => {
                state.play(Sound::UiSwitch, 0.75);
                self.ups -= 1;
            }
            Key::Down if self.ups < self.upgrades.len() - 1 => {
                state.play(Sound::UiSwitch, 0.75);
                self.ups += 1;
            }
            Key::Space => {
                state.play(Sound::UiSwitch, 0.75);
                self.ret = Some(self.upgrades[self.ups]);
            }
            _ => {}
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy)]
pub enum Upgrade {
    MaxAmmo,
    MaxBomb,
    MaxTurret,
    MaxEmp,

    NotConsumeAmmo,
    PierceAmmo,
    ForkAmmo,
    Sniper,

    DmgBomb,
    RadAndFuseBomb,
    ScoreBomb,

    CdTurret,
    DmgTurret,

    MovSpd,

    DurEmp,
}

impl Upgrade {
    pub const COUNT: usize = 15;
    fn pool() -> Vec<Upgrade> {
        (0..Self::COUNT).map(|n| Upgrade::from(n as i32)).collect()
    }
    fn random(player: &Player) -> Vec<Upgrade> {
        let mut rng = thread_rng();
        let mut pool = Self::pool();
        let mut res = vec![];

        while res.len() < 5 {
            let upgrade = pool.remove(rng.gen_range(0..pool.len()));

            match upgrade {
                Upgrade::NotConsumeAmmo if player.upgrades[upgrade as usize] == 3 => continue,
                Upgrade::CdTurret if player.upgrades[upgrade as usize] == 3 => continue,
                Upgrade::MovSpd if player.upgrades[upgrade as usize] == 3 => continue,
                Upgrade::DmgTurret if player.upgrades[upgrade as usize] == 3 => continue,
                _ => {}
            }

            res.push(upgrade);
        }
        res
    }
    fn text(self) -> &'static str {
        use Upgrade::*;
        match self {
            MaxAmmo => "+3 max ammo",
            MaxBomb => "+1 max bomb",
            MaxTurret => "+1 max turret",
            MaxEmp => "+1 max EMP",

            NotConsumeAmmo => "+20% chance to not consume ammo", //3
            PierceAmmo => "+1 bullet pierce",
            ForkAmmo => "+1 bullet fork",
            Sniper => "slower reload and +1 bullet damage",

            DmgBomb => "+3 bomb damage",
            RadAndFuseBomb => "+1 bomb radius, +1s bomb fuse time",
            ScoreBomb => "+3 score on kill with bomb",

            CdTurret => "faster reload for turrets", //3
            DmgTurret => "+1 turret damage",         //3

            MovSpd => "increased movement speed", //3

            DurEmp => "+5s EMP duration",
        }
    }
    pub fn apply(self, player: &mut Player) {
        match self {
            Upgrade::MaxAmmo => {
                player[MaxAmmo] += 3;
            }
            Upgrade::MaxBomb => {
                player[MaxBomb] += 1;
            }
            Upgrade::MaxTurret => {
                player[MaxTurret] += 1;
            }
            Upgrade::MaxEmp => {
                player[MaxEmp] += 1;
            }

            Upgrade::NotConsumeAmmo => {
                player[NotConsumeAmmo] += 20;
            }
            Upgrade::PierceAmmo => {
                player[PierceAmmo] += 1;
            }
            Upgrade::ForkAmmo => {
                player[ForkAmmo] += 1;
            }
            Upgrade::Sniper => {
                player[CdShot] += 10;
                player[DmgAmmo] += 1;
            }

            Upgrade::DmgBomb => {
                player[DmgBomb] += 1;
            }
            Upgrade::RadAndFuseBomb => {
                player[RadBomb] += 1;
                player[FuseTimeBomb] += 1;
            }
            Upgrade::ScoreBomb => {
                player[ScoreBomb] += 3;
            }

            Upgrade::CdTurret => {
                player[CdTurretShot] -= 75;
            }
            Upgrade::DmgTurret => {
                player[DmgTurret] += 1;
            }

            Upgrade::MovSpd => {
                player[CdMove] -= 1;
            }

            Upgrade::DurEmp => {
                player[SlowEmp] += 5 * 60;
            }
        }
        player.upgrades[self as usize] += 1;
    }
}

impl From<i32> for Upgrade {
    fn from(value: i32) -> Self {
        use Upgrade::*;
        match value {
            0 => MaxAmmo,
            1 => MaxBomb,
            2 => MaxTurret,
            3 => MaxEmp,

            4 => NotConsumeAmmo,
            5 => PierceAmmo,
            6 => ForkAmmo,
            7 => Sniper,

            8 => DmgBomb,
            9 => RadAndFuseBomb,
            10 => ScoreBomb,

            11 => CdTurret,
            12 => DmgTurret,

            13 => MovSpd,

            14 => DurEmp,

            _ => panic!(),
        }
    }
}
