use super::view::DupeTable;
use crate::{enums::enums::{self, FileAction}, file, finder::finder};
 
use egui::{ Color32, ScrollArea}; 
use eframe::{  egui, epi::{self, Storage}, };
  
extern crate byte_unit;
use byte_unit::{Byte};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use home::home_dir;

//TODO check self.b and self.a references!!!

#[derive(Clone)]
pub struct Application<'a> {
    //scroll_area: Option<egui::containers::scroll_area::ScrollAreaOutput<()>>,
    pub time_elapsed: std::time::Duration,
    pub fuzzy_search: String,
    pub e: Vec<DupeTable>,
    pub staging: Vec<Vec<DupeTable>>,
    pub selected_staging_index: usize,
    pub a: finder::Finder,
    pub b: finder::Finder,
    pub c: Vec<file::meta::Meta>,
    pub selected_collection: String,
    pub sort_left_panel: [&'a str; 3],
    pub sort_left_panel_index: usize,
    pub pager_size: Vec<usize>,
    pub pager_size_index: usize,
    pub ctrl_skip_display_dupes: bool,
    pub ctrl_starting_directory: String,
    pub ctrl_filter_filetype: enums::FileType,
    pub filter_search_filetype: [bool; 5],
    pub filters_filetype_counters: [i32; 6],
    pub status_filetype_counters: bool,
    pub theme_prefer_light_mode: bool,
}

impl<'a> Application<'_> {
    pub fn default() -> Self {
        Self { 
            time_elapsed: std::time::Duration::new(0, 0),
            fuzzy_search: String::from(""),
            e: vec![],
            staging: vec![],
            selected_staging_index: 0,
            a: finder::Finder::new(),
            b: finder::Finder::new(),
            c: vec![file::meta::Meta::new()],
            selected_collection: String::from(""),
            sort_left_panel: ["Duplicates", "Name", "Size"],
            pager_size: [3, 5, 10, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
            pager_size_index: 5,
            sort_left_panel_index: 0,
            ctrl_starting_directory: "".to_string(),
            ctrl_skip_display_dupes: false,
            ctrl_filter_filetype: enums::FileType::All,
            filter_search_filetype: [true, true, true, false, true], // [flag_audio,flag_document,flag_image,flag_other,flag_video]
            filters_filetype_counters: [0; 6], // [flag_audio,flag_document,flag_image,flag_other,flag_video]; flag_all
            theme_prefer_light_mode: true,
            status_filetype_counters: false,
        }
    }

    pub fn drop_down_sort_by(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if egui::ComboBox::new("siome123", "")
                .width(100.0)
                .show_index(
                    ui,
                    &mut self.sort_left_panel_index,
                    self.sort_left_panel.len(),
                    |i| self.sort_left_panel[i].to_owned(),
                )
                .clicked()
            {};
            ui.label("Hide Singles");
            ui.add(toggle(&mut self.ctrl_skip_display_dupes));

            ui.label("Page Size");
            if egui::ComboBox::new("siome123d", "")
                .width(100.0)
                .show_index(ui, &mut self.pager_size_index, self.pager_size.len(), |i| {
                    self.pager_size[i].to_owned().to_string()
                })
                .clicked()
            {};

            self.fuzzy_search_ui(ui);

            //Search Time Duration
            ui.label("Search Duration");
            ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_GREEN);
                let t = format!("{:?} seconds", self.time_elapsed.as_secs_f64().to_string());
                ui.label(t);
            }); // the temporary settings are reverted here
        });

        ui.add_space(4.);
    }

    pub fn fuzzy_search_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Filter");

        ui.scope(|ui| {
            ui.style_mut().wrap = Some(true);
            ui.visuals_mut().extreme_bg_color = egui::Color32::from_rgb(230, 230, 230);

            let response = ui.add(
                egui::TextEdit::singleline(&mut self.fuzzy_search)
                    .code_editor()
                    .desired_width(300.),
            );

            if response.changed() {
                //Do nothing here
            }
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                println!("lost focus");
                println!("{:?}", &self.fuzzy_search);

                //self.selected_staging_index

                if !self.staging.is_empty() {
                    let vec = &self.configure_comparison_vec(vec![]);

                    let matcher = SkimMatcherV2::default();
                    let mut vec_temp: Vec<DupeTable> = vec![];
                    for dt in vec {
                        let res = matcher.fuzzy_match(&dt.name, &self.fuzzy_search);
                        match res {
                            Some(_) => {
                                vec_temp.push(dt.clone());
                            }
                            None => {}
                        }
                    }

                    println!("vec_temp {:#?}", vec_temp);

                    //Reset Pager
                    self.selected_staging_index = 0;

                    //Step next
                    //self.e  = vec_temp; //self.configure_comparison_vec(vec![]);
                    //Application::<'a>::sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut self.staging[self.selected_staging_index]);

                    self.staging = vec![vec_temp];
                }
            }
        });
    }

    pub fn sort_dupe_table(sort_left_panel_index: i32, vec: &mut Vec<DupeTable>) {
        match sort_left_panel_index {
            0 => {
                vec.sort_by(|a, b| b.count.cmp(&a.count)); //file count
            }
            1 => {
                vec.sort_by(|a, b| b.name.cmp(&a.name)); //file name
            }
            2 => {
                vec.sort_by(|a, b| {
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

    pub fn pager(&mut self, ui: &mut egui::Ui) {
        let _main_dir = egui::Direction::LeftToRight;
        let _layout = egui::Layout::left_to_right()
            .with_main_wrap(true)
            .with_cross_align(egui::Align::Center);

        egui::Grid::new("grid_main_labels")
            .spacing(egui::Vec2::new(2.0, 0.0))
            .show(ui, |ui| {
                //TODO ui is showing extra button at the end
                for i in 0..self.staging.len() {
                    if ui
                        .add_sized([40.0, 35.0], egui::Button::new((i + 1).to_string()))
                        .clicked()
                    {
                        self.selected_staging_index = i;
                    }
                }
            });
    }

    pub fn left_side_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        fn get_table_fields(dt: DupeTable) -> (String, String, String) {
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
            match dt.file_type {
                enums::FileType::Image => {
                    title = format!("🖼 {}", dt.name);
                }
                enums::FileType::Audio => {
                    title = format!("🎵 {}", dt.name);
                }
                enums::FileType::Video => {
                    title = format!("🎞 {}", dt.name);
                }
                enums::FileType::Document => {
                    title = format!("📎 {}", dt.name);
                }
                enums::FileType::Other => {
                    title = format!("📝 {}", dt.name);
                }
                enums::FileType::None => {}
                enums::FileType::All => {}
            }

            //title
            title = truncate(&title, 150).to_string();
            let diff = 150 - title.chars().count();
            if diff > 0 {
                for _ in 0..=diff {
                    title.push(' ');
                }
            }

            //adjusted_byte
            let diff = 10 - adjusted_byte.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            let adjusted_byte = [space, adjusted_byte.to_string()].join("");

            //text_file_count
            let diff = 12 - text_file_count.to_string().chars().count();
            let mut space: String = String::from("");
            for _ in 0..diff {
                space.push(' ');
            }
            let text_file_count = [space, text_file_count.to_string()].join("");

            (title, adjusted_byte, text_file_count)
        }

        /* /Move all this into a clicked{} event
        //let mut vec_table = self.configure_comparison_vec(vec![]);
        //let _ = sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut vec_table); */

        if !self.staging.is_empty() {
            let mut num_rows = 0;
            let vec_table = self.staging[self.selected_staging_index].clone();
            //println!(" \n\n\n\n vec_table = {:#?}", &vec_table);

            if vec_table.len() < self.pager_size[self.pager_size_index] {
                num_rows = vec_table.len();
            } else {
                num_rows = self.pager_size[self.pager_size_index];
            }

            //Style
            let mut style: egui::Style = (*_ctx.style()).clone();
            style.visuals.extreme_bg_color = egui::Color32::DARK_RED;
            style.visuals.faint_bg_color = egui::Color32::from_rgb(83, 115, 146);
            _ctx.set_style(style);

            //ScrollArea aTable ScrollAreaOutput<()>
            let row_height = 35.0;
            ScrollArea::vertical()
                .id_source("main_scroll")
                .auto_shrink([false, false])
                .max_height(500.)
                .stick_to_right()
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for row in row_range {
                        let (title, adjusted_byte, file_count) =
                            get_table_fields(vec_table[row].clone());

                        egui::Grid::new("grid_main_labels")
                            .striped(true)
                            .num_columns(3)
                            .spacing(egui::Vec2::new(0.0, 10.0))
                            .show(ui, |ui| {
                                if ui
                                    .add_sized(
                                        [970.0, 35.0],
                                        egui::Button::new(
                                            egui::RichText::new(truncate(&title, 122).to_string())
                                                .color(egui::Color32::from_rgb(45, 51, 59)),
                                        )
                                        .fill(egui::Color32::from_rgb(228, 244, 252)),
                                    )
                                    .clicked()
                                {
                                    self.selected_collection = vec_table[row].checksum.to_string();
                                    self.c = vec_table[row].list.to_vec();
                                }
                                ui.add_sized(
                                    [100.0, 35.0],
                                    egui::Button::new(
                                        egui::RichText::new(file_count)
                                            .color(egui::Color32::from_rgb(45, 51, 59)),
                                    )
                                    .fill(egui::Color32::from_rgb(228, 244, 252)),
                                );
                                ui.add_sized(
                                    [100.0, 35.0],
                                    egui::Button::new(
                                        egui::RichText::new(adjusted_byte)
                                            .color(egui::Color32::from_rgb(45, 51, 59)),
                                    )
                                    .fill(egui::Color32::from_rgb(228, 244, 252)),
                                );
                                ui.end_row();
                            });
                    }
                }); //end of scroll
        }
    }

    pub fn bottom_side_panel(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_source("bottom_scroll")
            .auto_shrink([false, false])
            .max_height(200.)
            .stick_to_right()
            .show(ui, |ui| {
                //let mut counter = 1;
                for row in self.c.iter_mut() {
                    //********************************************************//

                    //Formatting text for gui
                    let date = get_created_date(&row.path);
                    match &date {
                        Ok(_) => {}
                        Err(e) => {
                            println!("derror::ui::mod.rs::10001{} ", e);
                            break;
                        }
                    }

                    let byte = Byte::from_bytes(row.file_size.try_into().unwrap());
                    let adjusted_byte = byte.get_appropriate_unit(false);

                    let mut title: String;
                    title = format!("{} ", row.path); //▶  ▶
                    title = truncate(&title, 125).to_string();

                    //'attemp to subtract with overflow'
                    let diff = 125 - title.chars().count();
                    let mut space = " ".to_string();
                    for _ in 0..diff {
                        space.push(' ');
                    }

                    title = [
                        title.to_string(),
                        space,
                        date.unwrap(),
                        adjusted_byte.to_string(),
                    ]
                    .join(" ");

                    //********************************************************//

                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut row.ui_event_status, title).clicked() {
                            if row.ui_event_status {
                                row.status = FileAction::Delete;
                            } else {
                                row.status = FileAction::Read;
                            }

                            let collection =
                                self.b.data_set.get_mut(&self.selected_collection).unwrap();
                            for mut row2 in collection {
                                if row2.path == row.path {
                                    if row.ui_event_status {
                                        //row2.set_status(FileAction::Delete);
                                        row2.status = FileAction::Delete;
                                        row2.ui_event_status = true;
                                    } else {
                                        //row2.set_status(FileAction::Read);
                                        row2.status = FileAction::Read;
                                        row2.ui_event_status = false;
                                    }
                                }
                            }

                            /* let modifiers = ui.ctx().input().modifiers;
                            ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                                url: row.path.to_owned(),
                                new_tab: modifiers.any(),
                            }); */
                        };
                        ui.hyperlink_to("VIEW", &row.path).on_hover_ui(|_ui| {});
                    });

                    //counter += 1;
                }
            }); //end of scroll
    }

    pub fn delete_collection(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(egui::Button::new(
                egui::RichText::new("Delete Below")
                    .color(egui::Color32::LIGHT_RED)
                    .monospace(),
            ))
            .clicked()
        {
            //Remove file from os first
            for row in &self.c {
                if row.status == FileAction::Delete {
                    std::fs::remove_file(&row.path).ok();
                }
            }

            //Remove file element from hashmap for gui first
            self.c.retain(|x| {
                (x.status == FileAction::None)
                    || (x.status == FileAction::Read)
                    || (x.status == FileAction::Save)
            });

            //Remove file element from hashmap for gui second
            let collection = self.b.data_set.get_mut(&self.selected_collection).unwrap();
            collection.retain(|x| {
                (x.status == FileAction::None)
                    || (x.status == FileAction::Read)
                    || (x.status == FileAction::Save)
            });
        };
    }

    pub fn delete_all(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_sized(
                [150.0, 2.0],
                egui::Button::new(
                    egui::RichText::new("Delete All Checked")
                        .color(egui::Color32::LIGHT_RED)
                        .monospace(),
                ),
            )
            .clicked()
        {
            //commented out for safe testing
            /* //Remove file from os step one
            for collection in &self.b.data_set {
                let (_, v) = collection;
                for item in v {
                    if item.status == FileAction::Delete {
                        std::fs::remove_file(&item.path).ok();
                    }
                }
            }

            //step two
            for collection in self.b.data_set.clone() {
                let (mut k, mut v) = collection;
                let mut cc = self.b.data_set.get_mut(&k.to_string()).unwrap();

                cc.retain(|x| {
                    (x.status == FileAction::None)
                        || (x.status == FileAction::Read)
                        || (x.status == FileAction::Save)
                });
            } */
        };
    }

    pub fn configure_fonts(&mut self, _ctx: &egui::Context) {
        //let mut style: egui::Style = (*ctx.style()).clone();
        // style.visuals.extreme_bg_color = egui::Color32::DARK_RED;
        // style.visuals.faint_bg_color = egui::Color32::LIGHT_BLUE;                           //highlights toggle ui background
        // style.visuals.code_bg_color = egui::Color32::from_rgb(45, 51, 59);

        // style.visuals.hyperlink_color = egui::Color32::from_rgb(0,191,255);                    //hyperlinks
        // style.visuals.override_text_color = Some(egui::Color32::from_rgb(45, 51, 59));            //Common Text (not text in main panel buttons)

        // style.visuals.button_frame = true;
        // style.visuals.collapsing_header_frame = true;                                                 //?
        // style.visuals.widgets.noninteractive.bg_fill = egui::Color32::DARK_RED;        //common background
        // style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(0., egui::Color32::DARK_RED);
        // style.visuals.widgets.inactive.bg_fill = egui::Color32::BROWN;
        // style.visuals.widgets.inactive.bg_fill = egui::Color32::LIGHT_RED;        //moouseover!
        // style.visuals.widgets.hovered.bg_fill = egui::Color32::YELLOW;            //moouseover!
        // style.visuals.widgets.active.bg_fill = egui::Color32::GRAY;                 //?
        // style.visuals.widgets.open.bg_fill = egui::Color32::GOLD;
        //ctx.set_style(style);
    }

    pub fn set_file_type_button<'b>(&mut self, ui: &mut egui::Ui, title: &str, index: usize) {
        let mut text = format!("{}::{}", title, self.filters_filetype_counters[index]);
        if index == 5 {
            text = title.to_string();
        }

        if ui
            .add_sized([140.0, 35.0], egui::Button::new(text))
            .clicked()
        {
            match index {
                0 => self.ctrl_filter_filetype = enums::FileType::Audio,
                1 => self.ctrl_filter_filetype = enums::FileType::Document,
                2 => self.ctrl_filter_filetype = enums::FileType::Image,
                3 => self.ctrl_filter_filetype = enums::FileType::Other,
                4 => self.ctrl_filter_filetype = enums::FileType::Video,
                5 => self.ctrl_filter_filetype = enums::FileType::All,
                _ => {}
            }

            //Step 0
            self.status_filetype_counters = true;

            //Step 1
            let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
            self.a = d2;

            //Reset Pager
            self.selected_staging_index = 0;

            //Step next
            self.e = self.configure_comparison_vec(vec![]);
            Application::<'a>::sort_dupe_table(
                self.sort_left_panel_index.try_into().unwrap(),
                &mut self.staging[self.selected_staging_index],
            );

            //println!("self.e {:?}", self.e);

            //scroll_offset --> self.offset = Some(Vec2::new(0.0, offset));
            //scroll_area = scroll_area.vertical_scroll_offset(0.0);
            //ui.scroll_to_cursor(Some(egui::Align::TOP));
        }
    }

    pub fn configure_comparison_vec(&mut self, mut vec: Vec<DupeTable>) -> Vec<DupeTable> {
        //Step 1
        for item in self.a.data_set.iter() {
            //for mut item in d2.data_set.clone().iter_mut() {
            let (k, v) = item;

            if self.ctrl_skip_display_dupes {
                if v.len() > 1 {
                    let dt = DupeTable {
                        name: v[0].name.to_string(),
                        count: v.len().try_into().unwrap(),
                        checksum: k.to_string(),
                        list: v.to_vec(),
                        file_type: v[0].file_type,
                    };
                    vec.push(dt);
                }
            } else {
                let dt = DupeTable {
                    name: v[0].name.to_string(),
                    count: v.len().try_into().unwrap(),
                    checksum: k.to_string(),
                    list: v.to_vec(),
                    file_type: v[0].file_type,
                };
                vec.push(dt);
            }
        }

        //println!("PRE sort");
        match self.sort_left_panel_index {
            0 => {
                vec.sort_by(|a, b| b.count.cmp(&a.count)); //file count
            }
            1 => {
                vec.sort_by(|a, b| b.name.cmp(&a.name)); //file name
            }
            2 => {
                vec.sort_by(|a, b| {
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
        //println!("AFTER sort");

        //Step 2; Paging Vector Readiness
        //Reset self.staging
        self.staging.clear();

        let pager_size = self.pager_size[self.pager_size_index];
        if vec.len() > pager_size {
            println!("step 2");
            let quot = vec.len() / pager_size;
            let rem = vec.len() % pager_size;

            for i in 0..quot {
                let y = (i + 1) * pager_size;
                let x = y - pager_size;

                println!("[x..y]: [{}..{}]", x, y);
                let v = vec[x..y].to_vec();
                self.staging.push(v.clone());
            }
            //rem
            {
                let y = quot * pager_size;
                let x = y - rem;

                println!("![x..y]: [{}..{}]", x, y);
                let v = vec[y..].to_vec();
                self.staging.push(v);
            }
        } else {
            self.staging.push(vec[..].to_vec());
        }

        vec
    }
}

//Helpers
fn filter_hashmap_by_filetype(mut d2: finder::Finder, ft: enums::FileType) -> finder::Finder {
 
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;

        if ft != enums::FileType::All {
            v.retain(|x| x.file_type == ft);

            if v.is_empty() {
                d2.data_set.remove(&k);
            }
        }
    }
 
    d2
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
                "F1l3 N0t f0und",
            ))
        }
    };
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
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
        let radius = 0.2 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp(
            (rect.left() + radius + 6.0)..=(rect.right() - radius - 6.0),
            how_on,
        );
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 1.95 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}
