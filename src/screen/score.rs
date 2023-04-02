use super::*;

pub struct Defeat;

impl Defeat {
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {}
    pub fn update(&self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        None
    }
}
