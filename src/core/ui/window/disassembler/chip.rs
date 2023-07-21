use eframe::egui;
use egui_extras::{Column, TableBuilder};
use crate::core::ui::State;
use crate::core::disasm::chip8::DataType;

pub fn render(state: &mut State, ui: &mut egui::Ui) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::exact(10.0))
            .column(Column::exact(50.0))
            .column(Column::exact(45.0))
            .column(Column::exact(120.0))
            .header(22.0, |mut header| {
                header.col(|ui| {
                    ui.heading("");
                });
                header.col(|ui| {
                    ui.heading("At");
                });
                header.col(|ui| {
                    ui.heading("Op");
                });
                header.col(|ui| {
                    ui.heading("Mnemonic");
                });
            })
            .body(|body| {
                if let Some(chip) = &state.chip.console {
                    let dis = &chip.rom.disassembly;

                    body.rows(20.0, dis.len(), |row_index, mut row| {
                        if let Some(instr) = dis[row_index].get_instruction() {
                            row.col(|ui| {
                                ui.label(match instr.address == chip.reg.pc - 0x200 {
                                    true => "➡",
                                    false => " "
                                });
                            });

                            row.col(|ui| {
                                ui.label(instr.info[0].clone());
                            });

                            row.col(|ui| {
                                ui.label(instr.info[1].clone());
                            });

                            row.col(|ui| {
                                ui.label(instr.info[2].clone());
                            });
                        }
                    });
                }
            });
    });
}