use byte_unit::Byte;
use eframe::{
    egui,
    epi::{self, Storage},
};
use egui::{Color32, ScrollArea, Sense};
use egui_extras::RetainedImage;

use crate::{
    enums::enums::{self, FileAction},
    file::{self, meta::Meta},
    finder::finder,
};

#[derive(Clone, Debug)]
pub struct DupeTable {
    pub name: String,
    pub count: i32,
    pub checksum: String,
    pub list: Vec<Meta>,
    pub file_type: enums::FileType,
    pub visible: bool,
}

pub struct Application {
    pub staging_checksum: String,
    pub staging: Vec<Vec<DupeTable>>,
    pub sub_staging: Vec<file::meta::Meta>,
    pub dupe_table: Vec<DupeTable>,
    pub finder: finder::Finder,
    pub directory: String,
    //
    pub time_elapsed: std::time::Duration,
    pub show_filter_bar: bool,
    pub show_delete_button: bool,
    //
    pub filter_search_filetype: [bool; 5],
    pub filters_filetype_counters: [i32; 6],
    //
    pub filter_all: bool,
    pub filter_audios: bool,
    pub filter_documents: bool,
    pub filter_images: bool,
    pub filter_videos: bool,
    pub filter_others: bool,
    //
    pub flag_checkbox_audios: bool,
    pub flag_checkbox_documents: bool,
    pub flag_checkbox_images: bool,
    pub flag_checkbox_others: bool,
    pub flag_checkbox_videos: bool,
    //
    pub image_checkbox_audios: RetainedImage,
    pub image_checkbox_documents: RetainedImage,
    pub image_checkbox_others: RetainedImage,
    pub image_checkbox_images: RetainedImage,
    pub image_checkbox_videos: RetainedImage,
    //
    pub image_filter: RetainedImage,
    pub image_directory: RetainedImage,
    pub image_run: RetainedImage,
    pub image_timer: RetainedImage,
    pub image: RetainedImage,
    pub image_delete: RetainedImage,
    //
    pub sort_by: [String; 3],
    pub sort_by_index: usize,
    pub pager_size: Vec<usize>,
    pub pager_size_index: usize,
    pub selected_staging_index: usize,
    pub fuzzy_search: String,
}

// from_svg_bytes
// from_image_bytes

impl Application {
    pub fn default() -> Self {
        Self {
            staging_checksum: String::from(""),
            sub_staging: vec![file::meta::Meta::new()],
            staging: vec![],               //used in paging
            dupe_table: vec![],            //ui uses this for show and tell
            finder: finder::Finder::new(), //runs the search
            directory: String::from("/Users/matthew/zz/file_types"),
            //
            time_elapsed: std::time::Duration::new(0, 0),
            show_filter_bar: true,
            show_delete_button: false,
            //
            filter_search_filetype: [false, true, false, false, false],
            filters_filetype_counters: [0; 6],
            //
            filter_all: true,
            filter_audios: true,
            filter_documents: true,
            filter_images: true,
            filter_videos: true,
            filter_others: true,
            //
            flag_checkbox_audios: false,
            flag_checkbox_documents: true,
            flag_checkbox_images: false,
            flag_checkbox_others: false,
            flag_checkbox_videos: false,
            //
            sort_by_index: 0,
            sort_by: [
                "Duplicates".to_string(),
                "Name".to_string(),
                "Size".to_string(),
            ],
            pager_size: [100, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
            pager_size_index: 4,
            selected_staging_index: 0,
            fuzzy_search: String::from(""),
            //
            image_checkbox_audios: RetainedImage::from_image_bytes(
                "Audio_Checkbox",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_documents: RetainedImage::from_image_bytes(
                "Document_Checkbox",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap(),
            image_checkbox_images: RetainedImage::from_image_bytes(
                "Image_Checkbox",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_others: RetainedImage::from_image_bytes(
                "Other_Checkbox",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            image_checkbox_videos: RetainedImage::from_image_bytes(
                "Video_Checkbox",
                include_bytes!("../../resources/unchecked.png"),
            )
            .unwrap(),
            //
            image_filter: RetainedImage::from_image_bytes(
                "Filter",
                include_bytes!("../../resources/filter1.png"),
            )
            .unwrap(),
            image_run: RetainedImage::from_image_bytes(
                "Run",
                include_bytes!("../../resources/play.png"),
            )
            .unwrap(),
            image_directory: RetainedImage::from_image_bytes(
                "Directory",
                include_bytes!("../../resources/folder.png"),
            )
            .unwrap(),
            image_timer: RetainedImage::from_image_bytes(
                "Time",
                include_bytes!("../../resources/chronometer.png"),
            )
            .unwrap(),
            image: RetainedImage::from_image_bytes(
                "Audio",
                include_bytes!("../../resources/checked.png"),
            )
            .unwrap(),
            image_delete: RetainedImage::from_image_bytes(
                "Delete",
                include_bytes!("../../resources/delete-3.png"),
            )
            .unwrap(),
        }
    }
}

impl epi::App for Application {
    fn name(&self) -> &str {
        "Sandbox"
        //crate::defines::APP_NAME
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {}

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
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

        // LEFT PANEL
        egui::SidePanel::left("left_panel")
            .frame(frame_style_1)
            .min_width(113.)
            .show(ctx, |ui| {
                ui.add_space(70.);
                self::Application::left_menu(self, ui, ctx);
            });

        // TOP PANEL
        self::Application::top_layout(self, ctx);

        // MAIN
        self::Application::main_layout(self, ctx);

        // DELETION/SUB PANEL
        egui::TopBottomPanel::bottom("bottom_sub_panel")
            .frame(frame_style_1)
            .show(ctx, |ui| {

                // BUTTON DELETE ROW 
                let image_size = self.image_delete.size_vec2();
                let image_button = egui::ImageButton::new(
                    self.image_delete.texture_id(ctx),
                    [image_size.x / 32., image_size.y / 32.],
                )
                .frame(true);

                ui.add_visible_ui(self.show_delete_button, |ui| {
     
                    if ui
                        .add(image_button)
                        .clicked()
                    {
                        // EVENT DELETE ITEMS
                        // Remove file from os first and then remove from vec[]
                        let collection = self
                            .finder
                            .data_set
                            .get_mut(&self.staging_checksum)
                            .unwrap();

                        // Loop through in reverse to maintain order when deleting
                        for i in (0..collection.len()).rev() {
                            if collection[i].status == FileAction::Delete
                                && collection[i].checksum == self.staging_checksum
                            {
                                std::fs::remove_file(&collection[i].path).ok();

                                // Deincrement file count on left side panel
                                match collection[i].file_type {
                                    enums::FileType::Image => {
                                        self.filters_filetype_counters[2] -= 1;
                                    }
                                    enums::FileType::Audio => {
                                        self.filters_filetype_counters[0] -= 1;
                                    }
                                    enums::FileType::Video => {
                                        self.filters_filetype_counters[4] -= 1;
                                    }
                                    enums::FileType::Document => {
                                        self.filters_filetype_counters[1] -= 1;
                                    }
                                    enums::FileType::Other => {
                                        self.filters_filetype_counters[3] -= 1;
                                    }
                                    enums::FileType::None => {}
                                    enums::FileType::All => {}
                                }

                                // Remove element from collection gui
                                collection.remove(i);
                            }
                        }

                        ///////////////////////////////////////////////////
                        self.dupe_table.clear();
                        for item in self.finder.data_set.iter() {
                            let (k, v) = item;

                            if v.len() > 0 {
                                // BUG HERE
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
                        }

                        // Clear staging before loading it
                        self.staging.clear();
                        let pager_size = self.pager_size[self.pager_size_index];

                        if self.dupe_table.len() > pager_size {
                            let quot = self.dupe_table.len() / pager_size;
                            let rem = self.dupe_table.len() % pager_size;

                            for i in 0..quot {
                                let y = (i + 1) * pager_size;
                                let x = y - pager_size;

                                let v = self.dupe_table[x..y].to_vec();
                                self.staging.push(v.clone());
                            }
                            //rem
                            {
                                let y = quot * pager_size;
                                let x = y - rem;

                                let v = self.dupe_table[y..].to_vec();
                                self.staging.push(v);
                            }
                        } else {
                            self.staging.push(self.dupe_table[..].to_vec());
                        }
                    }
                });

                // DELETION/SUB TABLE
                ui.add_space(5.0);
                ScrollArea::vertical()
                    .id_source("bottom_scroll2")
                    .auto_shrink([false, false])
                    .max_height(130.)
                    .min_scrolled_height(130.)
                    .stick_to_right()
                    .show(ui, |ui| {
                        //
                        ui.vertical(|ui| {
                            for row in self.sub_staging[..].iter_mut() {
                                //********************************************************//

                                //Formatting text for gui
                                let date = get_created_date(&row.path);
                                match &date {
                                    Ok(_) => {}
                                    Err(_) => {
                                        break;
                                    }
                                }

                                let byte = Byte::from_bytes(row.file_size.try_into().unwrap());
                                let adjusted_byte = byte.get_appropriate_unit(false);

                                let mut title: String;
                                title = format!("▶ {} ", row.path); //▶
                                title = truncate(&title, 94).to_string();
                                //'attemp to subtract with overflow'
                                let diff = 95 - title.chars().count();
                                let mut space = " ".to_string();
                                for _ in 0..diff {
                                    space.push(' ');
                                }

                                title = [title.to_string(), space].join(" ");

                                ///////////////////////////////////////////////////////////////

                                egui::Grid::new("deletion_grid")
                                    .striped(true)
                                    .num_columns(5)
                                    .min_row_height(20.0)
                                    .spacing(egui::Vec2::new(0.0, 0.0))
                                    .show(ui, |ui| {
                                        if ui.checkbox(&mut row.ui_event_status, " ").clicked() {
                                            //
                                            if row.ui_event_status {
                                                row.status = FileAction::Delete;
                                            } else {
                                                row.status = FileAction::Read;
                                            }

                                            // Below is needed for Deleting files from OS and GUI
                                            // Step 1
                                            let collection = self
                                                .finder
                                                .data_set
                                                .get_mut(&self.staging_checksum)
                                                .unwrap();
                                            // Step 2
                                            for mut row2 in collection {
                                                if row2.path == row.path {
                                                    if row.ui_event_status {
                                                        row2.status = FileAction::Delete;
                                                        row2.ui_event_status = true;
                                                    } else {
                                                        row2.status = FileAction::Read;
                                                        row2.ui_event_status = false;
                                                    }
                                                }
                                            }
                                        };
                                        ui.add_sized(
                                            [400.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(title)
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(date.unwrap())
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Label::new(
                                                egui::RichText::new(adjusted_byte.to_string())
                                                    .color(egui::Color32::from_rgb(200, 200, 200))
                                                    .monospace(),
                                            ),
                                        );
                                        ui.add_sized(
                                            [100.0, 15.0],
                                            egui::Hyperlink::from_label_and_url("VIEW", &row.path),
                                        );
                                        ui.end_row();
                                    });
                            }
                        });
                    }); //end of scroll
            });
    }

    fn on_exit(&mut self) {
        // DO Nothing
    }
}

pub fn get_created_date(path: &str) -> std::io::Result<String> {
    let _metadata = match std::fs::metadata(path) {
        Ok(f) => {
            if let Ok(time) = f.created() {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                let t: String = datetime.format("%m/%d/%Y").to_string();
                return Ok(t);
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "foo"));
            }
        }
        Err(_e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "File not Found: 100221",
            ))
        }
    };
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
