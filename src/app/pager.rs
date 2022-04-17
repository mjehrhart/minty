use super::controller::Application;
use crate::enums::enums;

impl Application {
    //
    pub fn pager(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Grid::new("grid_pager_main")
            .spacing(egui::Vec2::new(0.0, 0.0))
            .show(ui, |ui| {
                //
                ui.with_layout(
                    egui::Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.label("   ");
                    },
                );
                for i in 0..self.staging.len() {
                    if self.staging.len() > 1 {
                        if ui
                            .add_sized([25.0, 15.0], egui::Button::new((i + 1).to_string()))
                            .clicked()
                        {
                            self.selected_staging_index = i;

                            // Hide delete button
                            self.show_delete_button = false;

                            // Clear deletion panel
                            self.sub_staging = vec![];

                            // Clear counters
                            self.filter_audios = false;
                            self.filter_documents = false;
                            self.filter_images = false;
                            self.filter_others = false;
                            self.filter_videos = false;
                            for item in &mut self.staging[self.selected_staging_index] {
                                match item.file_type {
                                    enums::FileType::Audio => {
                                        self.filter_audios = true;
                                    }
                                    enums::FileType::Document => {
                                        self.filter_documents = true;
                                    }
                                    enums::FileType::Image => {
                                        self.filter_images = true;
                                    }
                                    enums::FileType::Other => {
                                        self.filter_others = true;
                                    }
                                    enums::FileType::Video => {
                                        self.filter_videos = true;
                                    }
                                    enums::FileType::None => {}
                                    enums::FileType::All => {}
                                }
                            }
                        }
                    }
                }
            });
    }
}
