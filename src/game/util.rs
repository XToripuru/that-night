use super::*;

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub trait TextExt {
    fn pixtext(
        &self,
        text: impl AsRef<str>,
        pos: [f32; 2],
        size: u32,
        align: (i8, i8),
        font: Font,
    ) -> Drawing<primitive::Text>;
}

impl TextExt for Draw {
    fn pixtext(
        &self,
        text: impl AsRef<str>,
        [x, y]: [f32; 2],
        size: u32,
        align: (i8, i8),
        font: Font,
    ) -> Drawing<primitive::Text> {
        self
            //.translate(Vec3::new(0.0, (-100.0 + size as f32 * 0.5).round(), 0.0))
            .text(text.as_ref())
            .x_y(x.round(), (y - 100.0 + size as f32 * 0.7).round())
            .font(font)
            .font_size(size)
            .center_justify()
            .align_text_top()
            .no_line_wrap()
    }
}

pub fn min(v1: f32, v2: f32) -> i32 {
    if v1 < v2 {
        v1 as i32
    } else {
        v2 as i32
    }
}

pub fn max(v1: f32, v2: f32) -> i32 {
    if v1 > v2 {
        v1 as i32
    } else {
        v2 as i32
    }
}

pub fn dist(x: i32, y: i32, z: i32, t: i32) -> i32 {
    (x - z).abs() + (y - t).abs()
}
