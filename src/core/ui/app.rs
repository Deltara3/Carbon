use eframe::egui;
use crate::core::ui::{State, WindowState};
use crate::core::ui::bar;
use crate::core::ui::window;

pub struct Carbon {
    state: State,
    window_state: WindowState
}

impl Default for Carbon {
    fn default() -> Carbon {
        return Carbon {
            state: State::new(),
            window_state: WindowState::new()
        }
    }
}

impl eframe::App for Carbon {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut visual = egui::Visuals::dark();
        visual.window_shadow = egui::epaint::Shadow::NONE;
        visual.popup_shadow = egui::epaint::Shadow::NONE;
        ctx.set_visuals(visual);

        bar::main(&mut self.state, &mut self.window_state, ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            window::render(&mut self.state, &mut self.window_state, ctx);
            
            let size = frame.info().window_info.size;
            let paint = ui.painter();

            if let Some(ref mut chip) = &mut self.state.chip.console {
                if chip.operror {
                    self.state.emulating = false;
                    self.window_state.uhoh = true;
                }

                if self.state.emulating {
                    for _ in 0 .. self.state.chip.cps as u32 {
                        chip.input(ui);
                        chip.cycle();
                    }

                    chip.decrement_timers();
                }

                chip.draw(paint, size, self.state.chip.colors);

                if chip.quirk.display && self.state.emulating {  }
            }
        });

        ctx.request_repaint();
    }
}