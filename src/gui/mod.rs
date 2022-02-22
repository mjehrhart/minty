use crate::file::meta;

use super::*;
use egui::{CentralPanel, Color32, Context, ScrollArea};
use egui::{Image, Style};
use file::meta::*;
use std::sync::{Arc, Mutex};
use std::{array, borrow::Cow, collections::HashMap, error::Error};

use eframe::{
    egui,
    epi::{self, Frame, Storage},
};

use tokio::io::{self, AsyncReadExt};
use tokio::task;
use tokio::{fs, join};

use std::sync::mpsc::channel;
use std::thread;

extern crate byte_unit;
use byte_unit::{Byte, ByteUnit};
 
#[derive(Clone)]
pub struct Application<'a> {
    b: finder::finder::Finder,
    c: Vec<file::meta::Meta>,
    image: Vec<u8>,
    texture_size: egui::Vec2,
    //texture: Option<egui::TextureId>,
    texture: Option<egui::TextureHandle>,
    selected_collection: String,
    sort_left_panel: [&'a str; 3],
    sort_left_panel_index: usize,
    ctrl_chunk_size: f64,
    ctrl_view_mode: bool,
    ctrl_remove_mode: bool,
    ctrl_skip_display_dupes: bool,
    ctrl_starting_directory: String,
    ctrl_bucket: &'a str,
    ctrl_filter_filetype: enums::enums::FileType,
    filter_search_filetype: [bool; 5],
    filters_filetype_counters: [i32;6],
    theme_prefer_light_mode: bool,
}

impl Application<'_> {
    pub fn default() -> Self {
        Self {
            b: finder::finder::Finder::new(),
            c: vec![file::meta::Meta::new()],
            image: vec![],
            texture_size: egui::Vec2::new(16.0, 9.0),
            texture: None,
            selected_collection: String::from(""),
            sort_left_panel: ["Duplicates", "Name", "Size"],
            sort_left_panel_index: 0,
            ctrl_chunk_size: 81920.,
            ctrl_view_mode: false,
            ctrl_remove_mode: false,
            ctrl_starting_directory: "".to_string(),
            ctrl_bucket: "",
            ctrl_skip_display_dupes: true,
            ctrl_filter_filetype: enums::enums::FileType::All,
            filter_search_filetype: [true, true, true, false, true], // [flag_audio,flag_document,flag_image,flag_other,flag_video]
            filters_filetype_counters: [0;6],
            theme_prefer_light_mode: true,
        }
    }
 
    fn drop_down_sort_by(&mut self, ui: &mut egui::Ui) {
        //ui.vertical_centered_justified(|ui| {});

        //ui.add(toggle(&mut self.ctrl_skip_display_dupes));
        // ui.add_sized([100.0,10.0], egui::ComboBox::from_label("").show_index(
        //                        ui,
        //                 &mut self.sort_left_panel_index,
        //                 self.sort_left_panel.len(),
        //                 |i| self.sort_left_panel[i].to_owned(),
        // ));
  
        // let x = egui::ComboBox::from_label("label")
        // .width(137.0) 
        // .show_index(
        //     ui,
        //     &mut self.sort_left_panel_index,
        //     self.sort_left_panel.len(),
        //     |i| self.sort_left_panel[i].to_owned(),
        // );
 
        // if ui.add_sized([100.0, 35.0], egui::ComboBox::new("sdfd", "label") 
        //     .show_index(
        //         ui,
        //         &mut self.sort_left_panel_index,
        //         self.sort_left_panel.len(),
        //         |i| self.sort_left_panel[i].to_owned(),
        //     ))
        //     .clicked(){}
    

 
        egui::Grid::new("grid_hide_singles")
            .striped(true)
            .num_columns(2)
            //.spacing(egui::Vec2::new(16.0, 20.0))
            .show(ui, |ui| {
                //ui.label("Hide Singles");
 
                if egui::ComboBox::new("siome123","")
                    .width(137.0) 
                    .show_index(
                        ui,
                        &mut self.sort_left_panel_index,
                        self.sort_left_panel.len(),
                        |i| self.sort_left_panel[i].to_owned(),
                    )
                    .clicked()
                {
                     
                };



                ui.end_row();
                ui.add(toggle(&mut self.ctrl_skip_display_dupes));
                ui.end_row();
 
                // ui.label("Delete");
                // ui.add(toggle(&mut self.ctrl_remove_mode)); 
            });
    }

    fn left_side_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        
        fn filter_hashmap_by_filetype(
            mut d2: finder::finder::Finder,
            ft: enums::enums::FileType,
        ) -> finder::finder::Finder {
            for collection in d2.data_set.clone().into_iter() {
                let (k, mut v) = collection;

                if ft != enums::enums::FileType::All {
                    v.retain(|x| x.file_type == ft);

                    if v.len() == 0 {
                        d2.data_set.remove(&k);
                    }
                }
            }

            d2
        }

        let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);

        let mut comparison_vec: Vec<(String, i32, String, Vec<Meta>, String, enums::enums::FileType)> = vec![];
        //for mut item in self.b.data_set.iter_mut() {
        for mut item in d2.data_set.clone().iter_mut() {
            //TODO maybe make mutable
            let (k, v) = item; 
            if self.ctrl_skip_display_dupes == true {
                if v.len() > 1 {
                    ///////
                    comparison_vec.push((
                        v[0].name.to_string(),       //name
                        v.len().try_into().unwrap(), //number of files
                        k.to_string(),               //checksum
                        v.to_vec(),                  //list of files
                        k.to_string(),
                        v[0].file_type
                        
                    ));
                }
            } else {
                comparison_vec.push((
                    v[0].name.to_string(),       //name
                    v.len().try_into().unwrap(), //number of files
                    k.to_string(),               //checksum
                    v.to_vec(),                  //list of files
                    k.to_string(),
                    v[0].file_type
                ));
            }
        }

        let row_height = 9.0;
        let num_rows = comparison_vec.len();
        match self.sort_left_panel_index {
            0 => {
                comparison_vec.sort_by(|a, b| b.1.cmp(&a.1));
            }
            1 => {
                comparison_vec.sort_by(|a, b| b.0.cmp(&a.0));
            }
            2 => {
                comparison_vec.sort_by(|a, b| {
                    let mut a_total = 0;
                    for row in &a.3 {
                        a_total += row.file_size;
                    }

                    let mut b_total = 0;
                    for row in &b.3 {
                        b_total += row.file_size;
                    }

                    b_total.cmp(&a_total)
                });
            }
            _ => {}
        }

        ScrollArea::vertical()
            .id_source("main_scroll")
            .auto_shrink([false, false])
            .max_height(500.)
            .stick_to_right()
            .show_rows(ui, row_height, num_rows, |ui, row_range| { 
                for row in row_range {
                    let mut a_total = 0;
                    for item in &comparison_vec[row].3 {
                        a_total += item.file_size;
                    }

                    let byte = Byte::from_bytes(a_total.try_into().unwrap());
                    let adjusted_byte = byte.get_appropriate_unit(false);

                    let mut text_file_count = "      ".to_string();
                    if comparison_vec[row].1 > 1 {
                        text_file_count = format!("{} files", comparison_vec[row].1);
                    }

                    let mut title: String = String::from("");

                    match comparison_vec[row].5{
                        enums::enums::FileType::Image => {
                            title = format!(" 🖼 {}", comparison_vec[row].0);
                        },
                        enums::enums::FileType::Audio => {
                            title = format!(" 🎵  {}", comparison_vec[row].0);
                        },
                        enums::enums::FileType::Video => {
                            title = format!(" 🎞  {}", comparison_vec[row].0);
                        },
                        enums::enums::FileType::Document => {
                            title = format!(" 📎 {}", comparison_vec[row].0);
                        },
                        enums::enums::FileType::Other => {
                            title = format!(" 📝  {}", comparison_vec[row].0);
                        },
                        enums::enums::FileType::None => {},
                        enums::enums::FileType::All => {},
                    }
                  
                    title = truncate(&title, 110).to_string();

                    let diff = 115 - title.chars().count();
                    let mut space = " ".to_string();
                    for _ in 0..diff {
                        space.push(' ');
                    }

                    title = [
                        title.to_string(),
                        space,
                        text_file_count,
                        adjusted_byte.to_string(),
                    ]
                    .join("   ");

                    if self.theme_prefer_light_mode == true {
                        //Light Mode
                        let mut style: egui::Style = (*_ctx.style()).clone();
                        //style.visuals.extreme_bg_color = egui::Color32::DARK_RED;                 
                        style.visuals.faint_bg_color = egui::Color32::LIGHT_BLUE;                   
                        _ctx.set_style(style);
 

                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            ui.set_height(35.);
                            if ui
                                .add_sized([1000.0, 35.0],
                                    egui::Button::new(
                                        egui::RichText::new(title)
                                            //.color(egui::Color32::WHITE)
                                            .color(egui::Color32::from_rgb(48,48,48))
                                            //.size(14.5)
                                            //.raised()
                                            .monospace(),
                                    ).frame(true)
                                    .fill(egui::Color32::from_rgb(228, 244, 252)), //137, 207, 240,// 45, 51, 59
                                )
                                .clicked()
                            { 
                                let image_path = comparison_vec[row].3[0].path.clone();
                                //self.create_image_texture(ctx, &image_path, ui);
                                self.selected_collection = comparison_vec[row].4.to_string();
                                self.c = comparison_vec[row].3.to_vec(); 
                            }
                        }); 
                    } else {
                        //Dark Mode
                        let mut style: egui::Style = (*_ctx.style()).clone();
                        //style.visuals.extreme_bg_color = egui::Color32::DARK_RED;                 
                        style.visuals.faint_bg_color = egui::Color32::LIGHT_BLUE;                   
                        _ctx.set_style(style);

                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            ui.set_height(35.);
                            if ui
                                .add_sized([1200.0, 35.0],
                                    egui::Button::new(
                                        egui::RichText::new(title)
                                            //.color(egui::Color32::WHITE)
                                            //.color(egui::Color32::from_rgb(48,48,48))
                                            //.size(14.5)
                                            //.raised()
                                            .monospace(),
                                    )
                                    //.fill(egui::Color32::from_rgb(228, 244, 252)), //137, 207, 240,// 45, 51, 59
                                )
                                .clicked()
                            { 
                                let image_path = comparison_vec[row].3[0].path.clone();
                                //self.create_image_texture(ctx, &image_path, ui);
                                self.selected_collection = comparison_vec[row].4.to_string();
                                self.c = comparison_vec[row].3.to_vec(); 
                            }
                        }); 

                    }
                }
            }); //end of scroll
    }

    fn bottom_side_panel(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_source("bottom_scroll")
            .auto_shrink([false, false])
            .max_height(200.)
            .stick_to_right()
            .show(ui, |ui| {
                let mut counter = 1;
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
 
                    let t_counter = format!("{} ", ""); //. ▶
                    ui.horizontal(|ui| {

                        if ui.checkbox(&mut row.ui_event_status, t_counter).clicked() {
 
                            if row.ui_event_status == true {
                                row.status = FileAction::Delete;
                            } else {
                                row.status = FileAction::Read;
                            }

                            let collection =
                                self.b.data_set.get_mut(&self.selected_collection).unwrap();
                            for mut row2 in collection {
                                if row2.path == row.path { 
                                    if row.ui_event_status == true {
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

                        };
                        ui.hyperlink_to(&title, &row.path).on_hover_ui(|ui| {});
                    });

                    counter += 1;
                }
            }); //end of scroll
    }

    fn delete_collection(&mut self, ui: &mut egui::Ui) {
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

    fn delete_all(&mut self, ui: &mut egui::Ui) {
        if ui
            .add_sized([150.0, 2.0], egui::Button::new(
                egui::RichText::new("Delete All Checked")
                    .color(egui::Color32::LIGHT_RED)
                    .monospace(),
            ))
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

}

impl<'a> epi::App for Application<'a> {
    fn name(&self) -> &str {
        "FUgly Finder"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
        //self.configure_fonts(ctx);
        fn filter_hashmap_by_filetype(
            mut d2: finder::finder::Finder,
            ft: enums::enums::FileType,
        ) -> finder::finder::Finder {
            for collection in d2.data_set.clone().into_iter() {
                let (k, mut v) = collection;

                if ft != enums::enums::FileType::All {
                    v.retain(|x| x.file_type == ft);

                    if v.len() == 0 {
                        d2.data_set.remove(&k);
                    }
                }
            }

            d2
        }
 
        let start = Instant::now();
        let dfer = return_dfer2("/Users/matthew/zz/", self.filter_search_filetype);
         
        println!("dfer::length::{:?}", &dfer.data_set.len());
        let d2 = filter_hashmap_by_filetype(dfer, enums::enums::FileType::All);
 
        //let mut flag_counters = [0;6];
        for collection in d2.data_set.iter(){
            let (k,v) = collection;

            for row in v{
                match row.file_type{
                    enums::enums::FileType::Audio => {
                        //flag_counters[0] += 1;
                        self.filters_filetype_counters[0] += 1;
                    } 
                    enums::enums::FileType::Document => {
                        //flag_counters[1] += 1;
                        self.filters_filetype_counters[1] += 1;
                    },
                    enums::enums::FileType::Image => {
                        //flag_counters[2] += 1;
                        self.filters_filetype_counters[2] += 1;
                    }, 
                    enums::enums::FileType::Other => {
                        //flag_counters[3] += 1;
                        self.filters_filetype_counters[3] += 1;
                    },
                    enums::enums::FileType::Video => {
                        //flag_counters[4] += 1;
                        self.filters_filetype_counters[4] += 1;
                    },
                    enums::enums::FileType::None => {},
                    enums::enums::FileType::All => {},
                }
            }
        }

        self.ctrl_starting_directory = "/Users/matthew/zz/".to_string(); 

        //self.b = dfer;
        self.b = d2;
        self.c = vec![];

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        //*************************************************************//

        let mut style: egui::Style = (*ctx.style()).clone();
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
        ctx.set_style(style);

        
        //Light Theme
        //ctx.set_visuals(egui::Visuals::light());
 
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "Droid Sans Mono".to_owned(),
            egui::FontData::from_static(include_bytes!("../Droid Sans Mono Nerd Font Complete Mono.otf")),
        ); 

        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Droid Sans Mono".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("Droid Sans Mono".to_owned()); 

        ctx.set_fonts(fonts);

        //self.create_image_texture(ctx, "/Users/matthew/temp/foldera/t/IMG_0059.JPG", ui);

        /* let x = Arc::new(Mutex::new(Application {
            b: finder::finder::Finder::new(),
            c: vec![file::meta::Meta::new()],
            image: vec![],
            texture_size: egui::Vec2::new(16.0, 9.0),
            texture: None,
            selected_collection: String::from(""),
            sort_left_panel: ["Duplicates", "Name", "Size"],
            sort_left_panel_index: 0,
            ctrl_chunk_size: 2048.,
            ctrl_view_mode: false,
            ctrl_remove_mode: false,
            ctrl_starting_directory: "".to_string(),
            ctrl_bucket: "",
            ctrl_skip_display_dupes: true,
        }));
        {
            // launch thread asynchronuously...
            let alias = x.clone(); // will refer to the same Mutex<Foo>
            let thread1 = thread::spawn(move || {
                let mut mutref = alias.lock().unwrap();
                mutref.create_image_texture("/Users/matthew/temp/foldera/t/IMG_0059.JPG");
            });

            thread1.join().unwrap();

            async {
                //cannot infer type for type parameter `E` declared on the enum `Result`
                let x:std::io::Result<(Vec<u8>, std::io::Error)> = self.create_image_texture("/Users/matthew/temp/foldera/t/IMG_0059.JPG").await;
                x
            };

        } */
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {

        let my_frame = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 2., top: 5., bottom: 2. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::YELLOW },
            fill: Color32::LIGHT_BLUE,
            stroke: egui::Stroke::new(0.0, Color32::GOLD),
        };

        egui::TopBottomPanel::top("top_panel").frame(my_frame).show(ctx, |ui| {
            ui.add_space(2.0);

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                //ui.set_min_height(50.);
 
                if ui
                    .add( egui::Button::new( egui::RichText::new("⎈ Run").color(egui::Color32::from_rgb(0,191,255)),
                    ))
                    .clicked()
                {
                    let dfer = return_dfer2(&self.ctrl_starting_directory, self.filter_search_filetype); 
                    let d2 = filter_hashmap_by_filetype(dfer, enums::enums::FileType::All);
                  
                    self.filters_filetype_counters = [0;6];
                    for collection in d2.data_set.iter(){
                        let (_,v) = collection;

                        for row in v{
                            match row.file_type{
                                enums::enums::FileType::Audio => {
                                    //flag_counters[0] += 1;
                                    self.filters_filetype_counters[0] += 1;
                                } 
                                enums::enums::FileType::Document => {
                                    //flag_counters[1] += 1;
                                    self.filters_filetype_counters[1] += 1;
                                },
                                enums::enums::FileType::Image => {
                                    //flag_counters[2] += 1;
                                    self.filters_filetype_counters[2] += 1;
                                }, 
                                enums::enums::FileType::Other => {
                                    //flag_counters[3] += 1;
                                    self.filters_filetype_counters[3] += 1;
                                },
                                enums::enums::FileType::Video => {
                                    //flag_counters[4] += 1;
                                    self.filters_filetype_counters[4] += 1;
                                },
                                enums::enums::FileType::None => {},
                                enums::enums::FileType::All => {},
                            }
                        }
                    }

                    self.b = d2;
                }

                if ui
                    .add( egui::Button::new( egui::RichText::new("🔵 Theme").color(egui::Color32::from_rgb(0,191,255)),
                    ))
                    .clicked()
                {
               
                    self.theme_prefer_light_mode = !self.theme_prefer_light_mode ;
                    if self.theme_prefer_light_mode == false{
                        ctx.set_visuals(egui::Visuals::dark()); 
                    } else {
                        ctx.set_visuals(egui::Visuals::light());
                    }
                }

                //[flag_audio,flag_document,flag_image,flag_other,flag_video] 
                if ui
                    .checkbox(&mut self.filter_search_filetype[0], "Audio")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | 3gp | aa | aac | aax | act | aiff | amr|  ape | au | flac | gsm | m4a | m4b | m4p | mp3 | mpc | mogg | ogg | raw | sln | tta | voc | vox | wav | wma |");
                    })
                    .clicked() 
                { 
                }
                if ui
                    .checkbox(&mut self.filter_search_filetype[1], "Documents")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | doc | docx | txt | xls | pdf | ppt | vcs | zip |");
                    })
                    .clicked()
                { 
                }
                if ui
                    .checkbox(&mut self.filter_search_filetype[2], "Images")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | dds | jpg | jpeg | heic | heif | png | psd | tif | tiff| tga | thm |");
                    })
                    .clicked()
                { 
                }
                if ui
                    .checkbox(&mut self.filter_search_filetype[3], "Other")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: anything not covered by the other filters");
                    })
                    .clicked()
                { 
                } 
                if ui
                    .checkbox(&mut self.filter_search_filetype[4], "Videos")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | avi | mpg | mpeg | mov | mp4 |");
                    })
                    .clicked()
                { 
                }

                //Open Directory
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Select Directory")
                            //.color(egui::Color32::GREEN)
                            .monospace(),
                    ))
                    .clicked()
                { 
                    ui.ctx().output().cursor_icon = egui::CursorIcon::Wait;

                    let path = std::env::current_dir().unwrap();
                    let res = rfd::FileDialog::new()
                        // .add_filter("text", &["txt", "rs"])
                        .set_directory(&path)
                        .pick_folder();

                    let folder = match res {
                        Some(_) => {
                            self.c = vec![]; 
                            let f = res.unwrap().clone().into_os_string().into_string(); 
                            self.ctrl_starting_directory = f.unwrap(); 
                        }
                        None => (),
                    };
                } 
                //Directory Label
                ui.scope(|ui| {
                    
                    ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_RED);
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                    ui.style_mut().wrap = Some(false);

                    ui.label(self.ctrl_starting_directory.to_string());
                }); // the temporary settings are reverted here
                 
  
                ui.end_row();
            });

            let sep = egui::Separator::default().spacing(5.);
            //ui.add_sized([143.0, 1.0], sep);
            ui.add(sep);

           // ui.add_space(1.0);
        });

        egui::SidePanel::left("my_side_panel").frame(my_frame).show(ctx, |ui| {
            //ui.add_space(7.0);

            //DropDown SortBy
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.drop_down_sort_by(ui);
            });

            let sep = egui::Separator::default().spacing(5.);
            ui.add_sized([143.0, 1.0], sep);
 
          
            //Menu Filters
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(
                egui::Direction::TopDown,
                egui::Align::LEFT), |ui| {
                //ui.set_height(20.);
                if ui
                    .add_sized([143.0, 100.0], egui::Button::new("All Files").frame(true))
                    .clicked()
                {
                    self.ctrl_filter_filetype = enums::enums::FileType::All;

                    // println!("copied_text::{:?}", ui.output().copied_text); //.copied_text = what_the_user_is_interested_in;

                    // let modifiers = ui.ctx().input().modifiers;
                    // ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                    //     url: "/Users/matthew/temp/IMG_0435.jpg".to_owned(),
                    //     new_tab: modifiers.any(),
                    // });

                }
            });

 
            // let sep = egui::Separator::default().spacing(5.);
            //  ui.add_sized([143.0, 1.0], sep);

            //self.filters_filetype_counters
            let title = format!("{}::{}", "Audio", self.filters_filetype_counters[0]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Audio;
            }

            let title = format!("{}::{}", "Documents", self.filters_filetype_counters[1]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Document;
            }

            let title = format!("{}::{}", "Images", self.filters_filetype_counters[2]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Image;
            } 

            let title = format!("{}:{}", "Others", self.filters_filetype_counters[3]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Other;
            }

            let title = format!("{}::{}", "Videos", self.filters_filetype_counters[4]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Video;
            }

            /* ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.style_mut().wrap = Some(true);

                ui.label("This text will be red, monospace, and won't wrap to a new line");
            }); // the temporary settings are reverted here */
        });

        egui::TopBottomPanel::bottom("bottom_panel").frame(my_frame).show(ctx, |ui| {
            //ui.set_height(20.);
            //ui.add_space(15.);
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                self.delete_all(ui);
                 
             });
             ui.add_space(2.);
        });

        egui::CentralPanel::default().frame(my_frame).show(ctx, |ui| {
              
            ui.with_layout(
                egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::RightToLeft,
                    egui::Align::LEFT,
                ),
                |ui| {
                    ui.vertical(|ui| {
                     
                        //MainPanel
                        self.left_side_panel(ui, ctx);

                        //Seperator
                        let sep = egui::Separator::default().spacing(5.);
                        ui.add(sep);

                        //Delete Collections
                        self.delete_collection(ui);

                        //CheckBoxes
                        self.bottom_side_panel(ui);

                        ui.allocate_space(ui.available_size());
                    });
                },
            );
        });

        // Resize the native window to be just the size we need it to be:
        //_frame.set_window_size(ctx.used_size());
 
    }
}

fn filter_hashmap_by_filetype(
    mut d2: finder::finder::Finder,
    ft: enums::enums::FileType,
) -> finder::finder::Finder {
 
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;
 
        if ft != enums::enums::FileType::All {
            v.retain(|x| x.file_type == ft);

            if v.len() == 0 {
                d2.data_set.remove(&k);
            }
        }
    }

    d2
}

 
pub fn get_created_date(path: &str) -> std::io::Result<String> {
    let metadata = match std::fs::metadata(path) {
        Ok(f) => {
            if let Ok(time) = f.created() {
                let datetime: chrono::DateTime<chrono::Local> = time.into();
                let t: String = datetime.format("%m/%d/%Y").to_string();
                return Ok(t);
            } else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "foo"));
            }
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "F1l3 N0t f0und",
            ))
        }
    };
}

pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(8.0, 1.0);
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
        let circle_x = egui::lerp((rect.left() + radius+6.0)..=(rect.right() - radius -6.0), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 1.95 *  radius, visuals.bg_fill, visuals.fg_stroke);
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

/* fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image_data = image_data;
    use image::GenericImageView;
    let image = image::load_from_memory(&image_data).expect("Failed to load image");
    let size = [(image.width() / 1) as usize, (image.height() / 1) as usize];
    println!("size::{:?}", size);
    let texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);
    println!("texture_size::{:?}", texture_size);
    let image_buffer = image.to_rgba8();
    //let pixels = image_buffer.into_vec();
    let pixels = image_buffer.as_flat_samples();

    // Allocate a texture:d
    // self.texture = Some(_frame.alloc_texture(image));
    // self.texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);

    // let image = image::load_from_memory(image_data)?;
    // let size = [500, 500];
    // let image_buffer = image.to_rgba8();
    // let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        //[texture_size.x as usize, texture_size.y as usize],
        size,
        pixels.as_slice(),
    ))
}
 */
// ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
//     ui.button("I am becoming wider as needed");
// });
// ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {

// match res {
//     Some(_) => {
//         let file = res.unwrap().clone().into_os_string().into_string();
//         match file {
//             Ok(file) => {
//                 self.ctrl_starting_directory = file;
//                 _frame.set_window_size(ctx.used_size());
//             }
//             Err(_) => {}
//         };
//     }
//     None => {}
// }

// ui.horizontal_wrapped(|ui| {
//     for i in 0..5 {
//         ui.group(|ui| {
//             ui.label(&format!("Item {}", i));
//             ui.button("x");
//         });
//     }
// });

// for collection in &self.b.data_set {
//     let (_, v) = collection;
//     for item in v {
//         if item.status == FileAction::Delete {

//         }
//     }
// }

// let collection = self.b.data_set.get_mut(&self.selected_collection).unwrap();
// collection.retain(|x| {
//     (x.status == FileAction::Empty)
//         || (x.status == FileAction::Read)
//         || (x.status == FileAction::Save)
// });
/* fn create_image_texture2(path: &str) -> Result<(eframe::epi::Image), String> {
    //ui.image(self.texture, self.texture_size);
    //ui.add(egui::ImageButton::new(self.texture.unwrap(), self.texture_size));

    fn get_file_bytes(path: &str) -> std::io::Result<(Vec<u8>)> {
        //-> Result<std::vec::Vec<u8>, _> { //Vec<u8>{

        let f = std::fs::File::open(path)?;
        let mut reader = std::io::BufReader::new(f);
        let mut buffer = Vec::new();

        // Read file into vector.
        std::io::Read::read_to_end(&mut reader, &mut buffer)?;

        Ok(buffer)
    }

    let image_data = get_file_bytes(path); //buffer;
    match image_data {
        Ok(byte_array) => {
            use image::GenericImageView;
            let image = image::load_from_memory(&byte_array).expect("Failed to load image");
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.into_vec();
            let size = [(image.width() / 1) as usize, (image.height() / 1) as usize];

            let image = epi::Image::from_rgba_unmultiplied(size, &pixels);

            // Allocate a texture:d
            //let texture = Some(_frame.alloc_texture(image));
            //let texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);

            //self.texture = Some(_frame.alloc_texture(image));
            //self.texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);

            //ui.image(self.texture.unwrap(), self.texture_size);
            //return Ok((texture, texture_size))
            return Ok((image));
        }
        Err(_) => todo!(),
    }
}
 */

 /* let fa: f64 = self.ctrl_chunk_size;
                            let mut ua: u32 = 0;
                            ua = (fa.round() as u32);
                            let chunk_size = ua as usize; */

 // let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                            //     let image = load_image(include_bytes!("/Users/matthew/temp/foldera/t/IMG_0059.JPG")).unwrap();
                            //     ctx.load_texture("rust-logo", image)
                            // });
