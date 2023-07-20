use eframe::egui;
use crate::core::ui::State;
use crate::core::ui::bar;
use crate::core::ui::window;

pub struct Carbon {
    state: State
}

impl Default for Carbon {
    fn default() -> Carbon {
        return Carbon {
            state: State::new()
        }
    }
}

impl eframe::App for Carbon {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut visual = egui::Visuals::dark();
        visual.window_shadow = egui::epaint::Shadow::NONE;
        visual.popup_shadow = egui::epaint::Shadow::NONE;
        ctx.set_visuals(visual);

        bar::main(&mut self.state, ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            window::render(&mut self.state, ctx);
            
            let size = frame.info().window_info.size;
            let paint = ui.painter();

            if let Some(ref mut chip) = &mut self.state.chip.console {
                if chip.operror {
                    self.state.emulating = false;
                    self.state.uhoh = true;
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