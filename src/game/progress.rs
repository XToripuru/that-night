pub enum Weapon {
    Ammo,
    Bomb,
    Turret,
    Emp,
}

pub struct Progress {
    pub killed: i32,    // might be [i32; 4] in the future
    pub used: [i32; 4], // used weapons
}

impl Progress {
    pub fn new() -> Self {
        Progress {
            killed: 0,
            used: [0; 4],
        }
    }
}
