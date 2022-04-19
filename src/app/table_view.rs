use std::collections::HashMap;

use super::controller::Application;
use crate::{app::controller::DupeTable, enums, file::meta::Meta};
use byte_unit::Byte;
use egui::{Color32, ScrollArea};

impl Application {
    pub fn main_layout(&mut self, ctx: &egui::Context) {
        //
        fn get_table_fields(dt: DupeTable) -> (String, String, String, String) {
            let mut a_total = 0;
            for item in &dt.list {
                a_total += item.file_size;
            }

            let byte = Byte::from_bytes(a_total.try_into().unwrap());
            let adjusted_byte = byte.get_appropriate_unit(false);

            let mut text_file_count = String::from("");
            if dt.count > 1 {
                text_file_count = format!("{} files", dt.count);
            }

            let mut title: String = String::from("");
            let mut icon: String = String::from("");
            match dt.file_type {
                enums::enums::FileType::Image => {
                    title = format!("{}", dt.name);
                    icon = "🖼".to_string();
                }
                enums::enums::FileType::Audio => {
                    title = format!("{}", dt.name);
                    icon = "🎵".to_string();
                }
                enums::enums::FileType::Video => {
                    title = format!("{}", dt.name);
                    icon = "🎞".to_string();
                }
                enums::enums::FileType::Document => {
                    title = format!("{}", dt.name);
                    icon = "📎".to_string();
                }
                enums::enums::FileType::Other => {
                    title = format!("{}", dt.name);
                    icon = "📝".to_string();
                }
                enums::enums::FileType::None => {}
                enums::enums::FileType::All => {}
            }

            // title
            title = title.to_lowercase();
            if title.len() < 100 {
                title = title[0..title.len()].to_string();
            } else {
                title = title[0..100].to_string();
            }
            let diff = 100 - title.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            title = ["  ".to_string(), title.to_string(), space].join(" ");

            // adjusted_byte
            let diff = 10 - adjusted_byte.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            let adjusted_byte = [space, adjusted_byte.to_string()].join("");

            // text_file_count
            let diff = 12 - text_file_count.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            let text_file_count = [space, text_file_count.to_string()].join("");

            (title, adjusted_byte, text_file_count, icon)
        }

        {
            let frame_style_1 = egui::containers::Frame {
                margin: egui::style::Margin {
                    left: 10.,
                    right: 2.,
                    top: 5.,
                    bottom: 2.,
                },
                rounding: egui::Rounding {
                    nw: 1.0,
                    ne: 1.0,
                    sw: 1.0,
                    se: 1.0,
                },
                shadow: eframe::epaint::Shadow {
                    extrusion: 0.0,
                    color: Color32::TRANSPARENT,
                },
                fill: Color32::from_rgb(49, 90, 125),
                stroke: egui::Stroke::new(0.0, Color32::from_rgb(244, 244, 244)),
            };

            let mut table_vec = vec![];
            let mut number_of_rows = 0;

            if !self.staging.is_empty() {
                //
                for item in &mut self.staging[self.selected_staging_index] {
                    if item.visible {
                        number_of_rows += 1;

                        table_vec.push(item);
                    }
                }

                match self.sort_by_index {
                    0 => {
                        table_vec.sort_by(|a, b| b.count.cmp(&a.count)); //file count
                    }
                    1 => {
                        table_vec.sort_by(|a, b| a.name.cmp(&b.name)); //file name
                    }
                    2 => {
                        table_vec.sort_by(|a, b| {
                            let mut a_total = 0;
                            for row in &a.list {
                                a_total += row.file_size;
                            }

                            let mut b_total = 0;
                            for row in &b.list {
                                b_total += row.file_size;
                            }

                            b_total.cmp(&a_total)
                        });
                    }
                    _ => {}
                }
            }

            egui::CentralPanel::default()
                .frame(frame_style_1)
                .show(ctx, |ui| {
                    let row_height = 35.0;
                    ScrollArea::vertical()
                        .id_source("main_scroll")
                        .auto_shrink([false, false])
                        .max_height(490.)
                        .min_scrolled_height(490.)
                        .stick_to_right()
                        .show_rows(ui, row_height, number_of_rows, |ui, row_range| {
                            for row in row_range {
                                //
                                let (title, adjusted_byte, file_count, icon) =
                                    get_table_fields(table_vec[row].clone());

                                egui::Grid::new("grid_main_labels")
                                    .striped(true)
                                    .num_columns(4)
                                    .striped(true)
                                    .spacing(egui::Vec2::new(10.0, 0.0))
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [25., 35.0],
                                            egui::Label::new(
                                                egui::RichText::new(icon)
                                                    .color(egui::Color32::from_rgb(255, 255, 255))
                                                    .monospace(),
                                            )
                                            //.fill(egui::Color32::from_rgb(49, 90, 125)),
                                        );
                                        ui.add_sized(
                                            [60.0, 35.0],
                                            egui::Label::new(
                                                egui::RichText::new(adjusted_byte)
                                                    .color(egui::Color32::from_rgb(255, 255, 255))
                                                    .monospace(),
                                            )
                                            //.fill(egui::Color32::from_rgb(49, 90, 125)),
                                        );
                                        ui.add_sized(
                                            [60.0, 35.0],
                                            egui::Label::new(
                                                egui::RichText::new(file_count)
                                                    .color(egui::Color32::from_rgb(255, 255, 255))
                                                    .monospace(),
                                            )
                                            //.fill(egui::Color32::from_rgb(49, 90, 125)),
                                        ); 
                                        // TITLE & CLICKED
                                        if ui
                                            .add_sized(
                                                [500.0, 35.0],
                                                egui::Button::new(
                                                    egui::RichText::new(title)
                                                        .color(egui::Color32::from_rgb(
                                                            255, 255, 255,
                                                        ))
                                                        .monospace(),
                                                )
                                                .fill(egui::Color32::from_rgb(49, 90, 125)),
                                            )
                                            .clicked()
                                        {
                                            // Show delete button
                                            self.show_delete_button = true;

                                            // Assign data to deletion panel
                                            self.sub_staging = (table_vec[row].list[..]).to_vec();

                                            // Store Checksum for sub_staging
                                            self.staging_checksum =
                                                table_vec[row].checksum.to_string();
                                        };

                                        ui.end_row();
                                    });
                            } // end for row in row_range
                        }); //end of scroll

                    let sep = egui::Separator::default();
                    //ui.add_space(5.0);
                    ui.add(sep);
                });

            egui::TopBottomPanel::bottom("bottom_sub_panel_pager")
                .frame(frame_style_1)
                .show(ctx, |ui| {
                    self::Application::pager(self, ui );
                    ui.add_space(5.0);
                });
        }
    }
}
