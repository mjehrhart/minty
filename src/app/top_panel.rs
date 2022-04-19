 use egui::Color32;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::{app::controller::DupeTable, enums, finder::finder};

use super::controller::Application;

impl Application {
    pub fn top_layout(&mut self, ctx: &egui::Context) {
        //
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

        egui::TopBottomPanel::top("top_panel")
            .frame(frame_style_1)
            .show(ctx, |ui| {
                // 
                ui.with_layout(egui::Layout::left_to_right(), |ui| {
                    //DIRECTORY IMAGE
                    let image_size = self.image_filter.size_vec2();
                    let image_button = egui::ImageButton::new(
                        self.image_directory.texture_id(ctx),
                        [image_size.x / 2., image_size.y / 2.],
                    )
                    .frame(true);

                    if ui.add(image_button).clicked() {
                        // ui.ctx().output().cursor_icon = egui::CursorIcon::Wait;

                        let res = rfd::FileDialog::new()
                            .set_directory(&home::home_dir().unwrap())
                            .pick_folder();

                        match res {
                            Some(_) => {
                                let f = res.unwrap().into_os_string().into_string();
                                self.directory = f.unwrap();
                            }
                            None => (),
                        };
                    }
                    self::Application::add_label(self, ui, self.directory.to_string());

                    // FILTER IMAGE, RUN IMAGE, TIMER IMAGE
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        //
                        // FILTER IMAGE
                        let image_size = self.image_filter.size_vec2();
                        let image_button = egui::ImageButton::new(
                            self.image_filter.texture_id(ctx),
                            [image_size.x / 2., image_size.y / 2.],
                        )
                        .frame(true);

                        ui.add_space(10.);
                        if ui.add(image_button).clicked() {
                            self.show_filter_bar = !self.show_filter_bar;
                        }

                        // RUN IMAGE
                        let image_size = self.image_filter.size_vec2();
                        let image_button = egui::ImageButton::new(
                            self.image_run.texture_id(ctx),
                            [image_size.x / 2., image_size.y / 2.],
                        )
                        .frame(true);

                        // RUNNING
                        if ui.add(image_button).clicked() {
                            // EVENT CLICKED ----- EVENT CLICKED ----- EVENT CLICKED
                            let start = std::time::Instant::now();
                            let ff = crate::return_dfer2(
                                &self.directory.to_string(),
                                self.filter_search_filetype,
                            );
                            let d2 = filter_hashmap_by_filetype(ff, enums::enums::FileType::All);

                            // Clear counters
                            self.filters_filetype_counters = [0; 6];
                            self.filter_audios = false;
                            self.filter_documents = false;
                            self.filter_images = false;
                            self.filter_others = false;
                            self.filter_videos = false;

                            // Set Filters (on/off)
                            for collection in d2.data_set.iter() {
                                let (_, v) = collection;

                                for row in v {
                                    match row.file_type {
                                        enums::enums::FileType::Audio => {
                                            self.filter_audios = true;
                                            self.filters_filetype_counters[0] += 1;
                                        }
                                        enums::enums::FileType::Document => {
                                            self.filter_documents = true;
                                            self.filters_filetype_counters[1] += 1;
                                        }
                                        enums::enums::FileType::Image => {
                                            self.filter_images = true;
                                            self.filters_filetype_counters[2] += 1;
                                        }
                                        enums::enums::FileType::Other => {
                                            self.filter_others = true;
                                            self.filters_filetype_counters[3] += 1;
                                        }
                                        enums::enums::FileType::Video => {
                                            self.filter_videos = true;
                                            self.filters_filetype_counters[4] += 1;
                                        }
                                        enums::enums::FileType::None => {}
                                        enums::enums::FileType::All => {}
                                    }
                                }
                            }
 
                            // Assign d2
                            self.finder = d2;

                            // Clear deletion panel
                            self.sub_staging.clear();

                            // Load DupeTable (self.dupe_table)
                            // Clear dupe_table before loading it
                            self.dupe_table.clear();
                            for item in self.finder.data_set.iter() {
                                let (k, v) = item;

                                let dt = DupeTable {
                                    name: v[0].name.to_string(),
                                    count: v.len().try_into().unwrap(),
                                    checksum: k.to_string(),
                                    list: v.to_vec(),
                                    file_type: v[0].file_type,
                                    visible: true,
                                };
                                self.dupe_table.push(dt);
                            }

                            // Count number of dupes found 
                            
                            for collection in &self.dupe_table { 
                                let x = collection.list.iter().filter( |&dt| dt.status == enums::enums::FileAction::Delete).count();
                                self.number_of_duplicates += x;
                                self.total_files_found += collection.list.len();

                            } 
                            //println!("total =>{}", self.total_files_found);
                            //println!("self.number_of_duplicates =>{}", self.number_of_duplicates);

                            //// ********************************** ////
                          
                            //
                            // Clear staging before loading it
                            self.staging.clear();
                            let pager_size = self.pager_size[self.pager_size_index];

                            if self.dupe_table.len() > pager_size {
                                let quot = self.dupe_table.len() / pager_size;
                                //let rem = self.dupe_table.len() % pager_size;
                         
                                for i in 0..quot {
                                    let y = (i + 1) * pager_size;
                                    let x = y - pager_size;
 
                                    let v = self.dupe_table[x..y].to_vec();
                                    self.staging.push(v.clone());
                                }
                                //rem
                                {
                                    let y = quot * pager_size; 
 
                                    let v = self.dupe_table[y..].to_vec();
                                    self.staging.push(v);
                                }
                            } else {
                                //
                                self.staging.push(self.dupe_table[..].to_vec());
                            }

                            // Reset pager index to 0
                            self.selected_staging_index = 0;
 
                            // Set Search Duration
                            let duration = start.elapsed();
                            self.time_elapsed = duration;
                        }

                        //Timer IMAGE
                        let image_size = self.image_filter.size_vec2();
                        let image_button = egui::ImageButton::new(
                            self.image_timer.texture_id(ctx),
                            [image_size.x / 2., image_size.y / 2.],
                        )
                        .frame(false);

                        if ui.add(image_button).clicked() {}
                        let time = format!("{:?}", self.time_elapsed);
                        self::Application::add_label(self, ui, time);
                    });
                });
                //});
            });

        egui::TopBottomPanel::top("top_panel2")
            .frame(frame_style_1)
            .show(ctx, |ui| {
                //
                ui.add_visible_ui(self.show_filter_bar, |ui| {
                    //
                    ui.group(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(), |ui| {
                            self::Application::checkbox_audio(self, ui, ctx);
                            self::Application::checkbox_documents(self, ui, ctx);
                            self::Application::checkbox_images(self, ui, ctx);
                            self::Application::checkbox_others(self, ui, ctx);
                            self::Application::checkbox_videos(self, ui, ctx);

                            ui.add_space(10.);
                            // DROPDOWN SORT BY
                            if egui::ComboBox::new("dropdown_sort_by", "")
                                .width(100.0)
                                .show_index(ui, &mut self.sort_by_index, self.sort_by.len(), |i| {
                                    self.sort_by[i].to_owned()
                                })
                                .clicked()
                            {};

                            // DROPDOWN NUMBER OF ROWS
                            if egui::ComboBox::new("number_of_rows", "")
                                .width(100.0)
                                .show_index(
                                    ui,
                                    &mut self.pager_size_index,
                                    self.pager_size.len(),
                                    |i| self.pager_size[i].to_owned().to_string(),
                                )
                                .changed()
                            {
                                // Change pager size when dropdown value changes
                                // Clear staging before loading it
                                self.staging.clear();
                                let pager_size = self.pager_size[self.pager_size_index];

                                if self.dupe_table.len() > pager_size {
                                    let quot = self.dupe_table.len() / pager_size;
                                    //let rem = self.dupe_table.len() % pager_size;
                                
                                    for i in 0..quot {
                                        let y = (i + 1) * pager_size;
                                        let x = y - pager_size;
 
                                        let v = self.dupe_table[x..y].to_vec();
                                        self.staging.push(v.clone());
                                    }
                                    //rem
                                    {
                                        let y = quot * pager_size;
                                        
                                        let v = self.dupe_table[y..].to_vec();
                                        self.staging.push(v);
                                    }
                                } else {
                                    //
                                    self.staging.push(self.dupe_table[..].to_vec());
                                }

                                // Reset to Pager Box index 0
                                self.selected_staging_index = 0; 

                                //// ********************************** ////
                            };

                            // FZZY FILTER
                            self::Application::add_label(self, ui, "Filter".to_string());
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.fuzzy_search)
                                    .code_editor()
                                    .desired_width(300.),
                            );

                            if response.changed() {
                                // DO Nothing
                            }

                            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                                let matcher = SkimMatcherV2::default();

                                //for mut collection in &mut self.dupe_table[..] {
                                for mut collection in &mut self.staging[self.selected_staging_index] {
                                    let res =
                                        matcher.fuzzy_match(&collection.name, &self.fuzzy_search);

                                    match res {
                                        Some(_) => {
                                            collection.visible = true;
                                        }
                                        None => {
                                            collection.visible = false;
                                        }
                                    }
                                }
                            }
                        });
                    });
                });
            });
    }
}

//Helpers
fn filter_hashmap_by_filetype(
    mut d2: finder::Finder,
    ft: enums::enums::FileType,
) -> finder::Finder {
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;

        if ft != enums::enums::FileType::All {
            v.retain(|x| x.file_type == ft);

            if v.is_empty() {
                d2.data_set.remove(&k);
            }
        }
    }

    d2
}
