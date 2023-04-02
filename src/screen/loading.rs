use super::*;

pub struct Loading {
    frame: i32,
}

impl Loading {
    pub fn new() -> Self {
        Self { frame: 0 }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        let p = self.frame as f32 / 60.0;

        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(512.0, 16.0)
            .rgba8(32, 32, 32, 255);

        draw.rect()
            .x_y(-256.0 + p * 512.0 * 0.5, 0.0)
            .w_h(p * 512.0, 16.0)
            .rgba8(255, 255, 255, 255);

        draw.pixtext(
            format!("{}%", (p * 100.0).round() as i32),
            [0.0, -48.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, 255);
    }
    pub fn update(&mut self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        //Some(Box::new(Screen::Menu(Menu::new())))

        self.frame += 1;

        if self.frame <= 60 {
            None
        } else {
            Some(Box::new(Screen::Menu(Menu::new(state))))
        }
    }
}
