//! Left side of app - the side filters

use egui::{Color32, ScrollArea};
use egui_extras::RetainedImage;

use super::controller::Application;
use crate::{enums::enums, finder::finder};

impl Application {
    pub fn set_theme(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ctx.set_visuals(egui::Visuals::light());

        let mut style: egui::Style = (*ctx.style()).clone();
        // if style.visuals.widgets.noninteractive.bg_fill == Color32::from_gray(30) {
        style.visuals.widgets.noninteractive.bg_fill = Color32::WHITE;
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke {
            width: 1.0,
            color: Color32::BLACK,
        };

        style.visuals.widgets.active.bg_fill = Color32::WHITE;
        style.visuals.widgets.active.fg_stroke = egui::Stroke {
            width: 1.0,
            color: Color32::BLACK,
        };

        style.visuals.widgets.inactive.bg_fill = Color32::WHITE;
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke {
            width: 1.0,
            color: Color32::BLACK,
        };

        // style.visuals.widgets.inactive.bg_stroke = egui::Stroke {
        //     width: 1.0,
        //     color:  Color32::from_rgb(70, 130, 180),
        // };

        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(70, 180, 120);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke {
            width: 1.0,
            color: Color32::from_rgb(70, 130, 180),
        };

        style.visuals.widgets.open.bg_fill = Color32::from_rgb(70, 130, 180);
        style.visuals.widgets.open.fg_stroke = egui::Stroke {
            width: 1.0,
            color: Color32::from_rgb(70, 130, 180),
        };
        ctx.set_style(style);
    }

    pub fn add_label(&mut self, ui: &mut egui::Ui, text: String) {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(egui::Color32::from_rgb(255, 255, 255)),
        ));
    }

    pub fn add_label_with_hover(&mut self, ui: &mut egui::Ui, text: String, hover_text: String) {
        ui.add(egui::Label::new(
            egui::RichText::new(text).color(egui::Color32::from_rgb(255, 255, 255)),
        ))
        .on_hover_ui(|ui| {
            ui.label(hover_text);
        });
    }

    pub fn set_toggle_name(&self, name: &str, index: usize) -> String {
        let number_pretty = self::Application::prettify(self.filters_filetype_counters[index]);
        let full_name = [name, &number_pretty].join(" ");

        full_name
    }

    pub fn left_menu(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let color32_blue = Color32::from_rgb(123, 167, 204);
        // let color32_blue_2 = Color32::from_rgb(70, 130, 180);
        let color32_purple = Color32::from_rgb(180, 70, 75);
        // let color32_orange = Color32::from_rgb(180, 120, 70);
        let color32_orange = Color32::from_rgb(230, 149, 0);
        let color32_green = Color32::from_rgb(70, 180, 120);
        let color32_green_2 = Color32::from_rgb(180, 175, 70);
        let color32_green_2 = Color32::from_rgb(255, 208, 0);

        // AUDIO
        ui.add_space(15.0);
        let name = self::Application::set_toggle_name(&self, "???? Audio ::", 0);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_audios,
                color32_blue,
                name,
            ))
            .clicked()
        {
            if self.filter_audios {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Audio {
                            collection.visible = true;
                        }
                    }
                }
            } else {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Audio {
                            collection.visible = false;
                        }
                    }
                }
            }
        }

        // DOCUMENTS
        ui.add_space(15.0);
        let name = self::Application::set_toggle_name(&self, "???? Docs ::", 1);
        //let name = ["???? Docs ::", &self.filters_filetype_counters[1].to_string()].join(" ");
        if ui
            .add(self::Application::toggle(
                &mut self.filter_documents,
                color32_orange,
                name,
            ))
            .clicked()
        {
            if self.filter_documents {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Document {
                            collection.visible = true;
                        }
                    }
                }
            } else {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Document {
                            collection.visible = false;
                        }
                    }
                }
            }
        }

        // IMAGES
        ui.add_space(15.0);

        let name = self::Application::set_toggle_name(&self, "???? Images ::", 2);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_images,
                color32_purple,
                name,
            ))
            .clicked()
        {
            if self.filter_images {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Image {
                            collection.visible = true;
                        }
                    }
                }
            } else {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Image {
                            collection.visible = false;
                        }
                    }
                }
            }
        }

        // OTHERS
        ui.add_space(15.0);
        let name = self::Application::set_toggle_name(&self, "???? Other ::", 3);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_others,
                color32_green,
                name,
            ))
            .clicked()
        {
            if self.filter_others {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Other {
                            collection.visible = true;
                        }
                    }
                }
            } else {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Other {
                            collection.visible = false;
                        }
                    }
                }
            }
        }

        // VIDEOS
        ui.add_space(15.0);
        let name = self::Application::set_toggle_name(&self, "???? Video ::", 4);
        if ui
            .add(self::Application::toggle(
                &mut self.filter_videos,
                color32_green_2,
                name,
            ))
            .clicked()
        {
            if self.filter_videos {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Video {
                            collection.visible = true;
                        }
                    }
                }
            } else {
                if !self.staging.is_empty() {
                    for collection in &mut self.staging[self.selected_staging_index] {
                        if collection.file_type == enums::FileType::Video {
                            collection.visible = false;
                        }
                    }
                }
            }
        }
     

        // Mini Dashboard
        ui.add_space(250.0);
        let sep = egui::Separator::default();
        ui.add_sized([100., 10.], sep);
        ui.add_space(11.0);
        self::Application::add_label(self, ui, "Total Files".to_string());
        self::Application::add_label(
            self,
            ui,
            self::Application::prettify(self.total_files_found),
        );
        self::Application::add_label(self, ui, "Duplicates Found".to_string());
        self::Application::add_label(
            self,
            ui,
            self::Application::prettify(self.number_of_duplicates),
        );
    }

    pub fn toggle_ui(
        ui: &mut egui::Ui,
        on: &mut bool,
        color32: Color32,
        text: String,
    ) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(6.0, 0.6);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed(); // report back that the value changed
        }

        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.43 * rect.height();

            ui.scope(|ui| {
                //ui.visuals_mut().override_text_color = Some(egui::Color32::from_white_alpha(100));
                ui.add_space(3.);
                ui.visuals_mut().override_text_color = Some(egui::Color32::from_rgb(255, 255, 255));
                ui.label(text);
            });

            ui.painter().rect(rect, radius, color32, visuals.bg_stroke);

            // Paint the circle, animating it from left to right with `how_on`:
            let circle_x = egui::lerp(
                (rect.left() + radius + 40.0)..=(rect.right() - radius - 20.0),
                how_on,
            );
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter().circle(
                center,
                1.95 * radius,
                color32,
                egui::Stroke::new(0.0, color32),
            );
        }

        response
    }
    pub fn toggle(on: &mut bool, color32: Color32, text: String) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| self::Application::toggle_ui(ui, on, color32, text)
    }

    pub fn prettify<T>(num: T) -> String
    where
        T: std::fmt::Display,
    {
        let y = num.to_string();
        let mut formatted_number = String::new();

        let a = y.chars().rev().enumerate();
        for (i, ch) in a {
            if i != 0 && i % 3 == 0 {
                formatted_number.insert(0, ',');
            }
            formatted_number.insert(0, ch);
        }

        formatted_number
    }
}
