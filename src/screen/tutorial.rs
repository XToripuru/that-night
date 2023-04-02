use super::*;

pub struct Tutorial {
    frame: i32,
    playing: Option<Box<Screen>>,
    next: Option<Box<Screen>>,
}

impl Tutorial {
    pub fn new(playing: Option<Box<Screen>>) -> Self {
        Self {
            frame: 0,
            playing,
            next: None,
        }
    }
    pub fn render(&self, app: &App, draw: &Draw, state: &State) {
        draw.pixtext(
            "Tutorial",
            [0.0, state.h * 0.5 - 96.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        draw.pixtext(
            "CTRL + ARROW",
            [-state.w * 0.30, 32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );
        draw.pixtext(
            "SHOOT",
            [-state.w * 0.30, -32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        draw.pixtext(
            "Q",
            [-state.w * 0.15, 32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );
        draw.pixtext(
            "BOMB",
            [-state.w * 0.15, -32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        draw.pixtext("W", [0.0, 32.0], 19 * 2, (0, 0), state.font.clone());
        draw.pixtext("RUN", [0.0, -32.0], 19 * 2, (0, 0), state.font.clone());

        draw.pixtext(
            "E + ARROW",
            [state.w * 0.15, 32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );
        draw.pixtext(
            "TURRET",
            [state.w * 0.15, -32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        draw.pixtext(
            "R",
            [state.w * 0.3, 32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );
        draw.pixtext(
            "EMP",
            [state.w * 0.3, -32.0],
            19 * 2,
            (0, 0),
            state.font.clone(),
        );

        let alpha = 255 - ((self.frame as f32 / 40.0).sin().abs() * 200.0) as u8;

        draw.pixtext(
            "press SPACE to begin",
            [state.w * 0.5 - 132.0, -state.h * 0.5 + 32.0],
            19,
            (0, 0),
            state.font.clone(),
        )
        .rgba8(255, 255, 255, alpha);

        // draw.pixtext(
        //     "press SPACE to continue",
        //     [0.0, -state.h * 0.5 + 96.0],
        //     19 * 2,
        //     (0, 0),
        //     state.font.clone(),
        // )
        // .rgba8(255, 255, 255, alpha);
    }
    pub fn update(&mut self, app: &App, state: &mut State) -> Option<Box<Screen>> {
        self.frame += 1;
        self.next.take()
    }
    pub fn pressed(&mut self, app: &App, state: &mut State, k: Key) {
        match k {
            Key::Space => {
                state
                    .storage
                    .achievements
                    .insert(Achievement::Tutorial, AchievementState { state: true });
                self.next = self.playing.take();
            }
            Key::Escape => {
                self.next = Some(Box::new(Screen::Menu(Menu::new(state))));
            }
            _ => {}
        }
    }
}
