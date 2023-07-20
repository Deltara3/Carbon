use crate::core::console::chip8::Chip8;
use eframe::egui::{Painter, Rounding, Pos2, Color32, Rect, Vec2, Key, Ui};

fn color(value: u8, colors: [Color32; 2]) -> Color32 {
    if value == 0 {
        return colors[0];
    } else {
        return colors[1];
    }
}

impl Chip8 {
    pub fn draw(&self, draw: &Painter, size: Vec2, colors: [Color32; 2]) {
        let scale;
        let scale_x = size[0] / 64.0;
        let scale_y = (size[1] - 24.0) / 32.0;

        if scale_x <= scale_y {
            scale = scale_x as u32;
        } else {
            scale = scale_y as u32;
        }

        for (y, row) in self.vram.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32 * scale) as u32;
                let y = (y as u32 * scale) as u32 + 24;
                let offset_x = (size[0] as u32 / 2) - (32 * scale);
                let offset_y = (((size[1] as u32 - 24) / 2) + 24) - ((16 * scale) + 24);


                draw.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(x as f32 + offset_x as f32, y as f32 + offset_y as f32),
                        Pos2::new(x as f32 + scale as f32 + offset_x as f32, y as f32 + scale as f32 + offset_y as f32),

                    ),
                    Rounding::none(),
                    color(col, colors)
                )
            }
        }
    }

    pub fn input(&mut self, ui: &Ui) {
        macro_rules! keypad {
            ($($code:literal = $key:ident),+) => {
                $( self.keypad[$code] = ui.input(|i| i.key_down(Key::$key)); )+
            }
        }

        keypad! {
            0x01 = Num1,
            0x02 = Num2,
            0x03 = Num3,
            0x0C = Num4,
            0x04 = Q,
            0x05 = W,
            0x06 = E,
            0x0D = R,
            0x07 = A,
            0x08 = S,
            0x09 = D,
            0x0E = F,
            0x0A = Z,
            0x00 = X,
            0x0B = C,
            0x0F = V
        }
    }
}