//use crate::file::meta;

use super::*;
use egui::{ Color32, ScrollArea};
//use egui::{Image, Style};
use file::meta::*;
//use std::sync::{Arc, Mutex};
//use std::{array, borrow::Cow, collections::HashMap, error::Error};

//use rayon::prelude::*;

  use eframe::{
    egui,
    epi::{self, Storage},
};

//use tokio::io::{self, AsyncReadExt};
// use tokio::task;
// use tokio::{fs, join};

//use std::sync::mpsc::channel;
//use std::thread;

extern crate byte_unit;
use byte_unit::{Byte};

#[derive(Clone, Debug)]
pub struct DupeTable {
    name: String,
    count: i32,
    checksum: String,
    list: Vec<Meta>, 
    file_type: enums::enums::FileType
}

//TODO check self.b and self.a references!!!

#[derive(Clone)]
pub struct Application<'a> {
    a: finder::finder::Finder,
    b: finder::finder::Finder,
    c: Vec<file::meta::Meta>, 
    selected_collection: String,
    sort_left_panel: [&'a str; 3],
    sort_left_panel_index: usize, 
    ctrl_skip_display_dupes: bool,
    ctrl_starting_directory: String, 
    ctrl_filter_filetype: enums::enums::FileType,
    filter_search_filetype: [bool; 5],
    filters_filetype_counters: [i32;6],
    status_filetype_counters: bool,
    theme_prefer_light_mode: bool,
}

impl Application<'_> {
    pub fn default() -> Self {
        Self {
            a: finder::finder::Finder::new(),
            b: finder::finder::Finder::new(),
            c: vec![file::meta::Meta::new()], 
            selected_collection: String::from(""),
            sort_left_panel: ["Duplicates", "Name", "Size"],
            sort_left_panel_index: 0, 
            ctrl_starting_directory: "".to_string(), 
            ctrl_skip_display_dupes: true,
            ctrl_filter_filetype: enums::enums::FileType::All,
            filter_search_filetype: [true, true, true, false, true],    // [flag_audio,flag_document,flag_image,flag_other,flag_video]
            filters_filetype_counters: [0;6],                           //[flag_audio,flag_document,flag_image,flag_other,flag_video]; flag_all
            theme_prefer_light_mode: true,
            status_filetype_counters: false,
        }
    }
 
    fn drop_down_sort_by(&mut self, ui: &mut egui::Ui) {
 
        egui::Grid::new("grid_hide_singles")
            .striped(true)
            .num_columns(2)
            //.spacing(egui::Vec2::new(16.0, 20.0))
            .show(ui, |ui| {
                //ui.label("Hide Singles");
 
                 
                ui.add(toggle(&mut self.ctrl_skip_display_dupes));
                ui.end_row();
                if egui::ComboBox::new("siome123","")
                    .width(136.0) 
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
            });
    }
 
    fn configure_comparison_vec(&mut self, mut vec: Vec<DupeTable> ) -> Vec<DupeTable> {
  
        for item in self.a.data_set.iter() {
        //     //for mut item in d2.data_set.clone().iter_mut() { 
            let (k, v) = item;  
            println!("k::{:?}", k);
            if self.ctrl_skip_display_dupes == true {
                if v.len() > 1 { 
                    let dt = DupeTable{
                        name: v[0].name.to_string(),
                        count: v.len().try_into().unwrap(),
                        checksum: k.to_string(),
                        list: v.to_vec(),
                        file_type: v[0].file_type,
                    }; 
                    vec.push(dt);
                } 
            } else {
                let dt = DupeTable{
                    name: v[0].name.to_string(),
                    count: v.len().try_into().unwrap(),
                    checksum: k.to_string(),
                    list: v.to_vec(),
                    file_type: v[0].file_type,
                };
                vec.push(dt); 
            }  
        }
        vec
    }

    fn left_side_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
  
        fn sort_dupe_table(sort_left_panel_index: i32, vec: &mut Vec<DupeTable>){
            match sort_left_panel_index {
                0 => {
                    vec.sort_by(|a, b| b.count.cmp(&a.count)); //file count
                }
                1 => {
                    vec.sort_by(|a, b| b.name.cmp(&a.name));     //file name
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
        
        fn get_table_fields(dt: DupeTable) -> (String, String, String){
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
            match dt.file_type{
                enums::enums::FileType::Image => {
                    title = format!("🖼 {}", dt.name);
                },
                enums::enums::FileType::Audio => {
                    title = format!("🎵 {}", dt.name);
                },
                enums::enums::FileType::Video => {
                    title = format!("🎞 {}", dt.name);
                },
                enums::enums::FileType::Document => {
                    title = format!("📎 {}", dt.name);
                },
                enums::enums::FileType::Other => {
                    title = format!("📝 {}", dt.name);
                },
                enums::enums::FileType::None => {},
                enums::enums::FileType::All => {},
            }
          
            //title
            title = truncate(&title, 150).to_string(); 
            let diff = 150 - title.chars().count(); 
            if diff > 0 {
                for _ in 0..=diff {
                    title.push(' ');
                }
            } 
            //println!("title.len()::{}", &title.chars().count()); 

            //adjusted_byte
            let diff = 10 - adjusted_byte.to_string().chars().count();  
            let mut space:String = String::from("");
            for _ in 0..diff {
                space.push(' ');

            }
            let adjusted_byte = [space, adjusted_byte.to_string()].join("");

            //text_file_count
            let diff = 10 - text_file_count.to_string().chars().count();  
            let mut space:String = String::from("");
            for _ in 0..diff {
                space.push(' ');

            }
            let text_file_count = [space, text_file_count.to_string()].join("");
            
            (title, adjusted_byte, text_file_count)
        }
         
        //
        let mut vec_table = self.configure_comparison_vec(vec![]); 
        let row_height = 35.0;
        let num_rows = vec_table.len(); 
        let _ = sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut vec_table);

        //Style
        let mut style: egui::Style = (*_ctx.style()).clone();
        style.visuals.extreme_bg_color = egui::Color32::DARK_RED;                 
        style.visuals.faint_bg_color = egui::Color32::from_rgb(83, 115, 146);                   
        _ctx.set_style(style);

        ScrollArea::vertical()
            .id_source("main_scroll")
            .auto_shrink([false, false]) 
            .max_height(500.)
            .stick_to_right()
            .show_rows(ui, row_height, num_rows, |ui, row_range| { 
                for row in row_range {
                     
                    let (title, adjusted_byte, file_count) = get_table_fields(vec_table[row].clone());
 
                    egui::Grid::new("grid_main_labels")
                        .striped(true)
                        .num_columns(3)
                        .spacing(egui::Vec2::new(0.0, 10.0))
                        .show(ui, |ui| {
                            if ui
                                .add_sized([900.0, 35.0],egui::Button::new(
                                    egui::RichText::new(truncate(&title, 122).to_string())
                                    .color(egui::Color32::from_rgb(45, 51, 59) ) )
                                    .fill(egui::Color32::from_rgb(228, 244, 252))
                                )
                                .clicked()
                            {  
                                self.selected_collection = vec_table[row].checksum.to_string();
                                self.c = vec_table[row].list.to_vec(); 
                            }
                            ui.add_sized([100.0, 35.0],egui::Button::new(
                                egui::RichText::new(file_count)
                                .color(egui::Color32::from_rgb(45, 51, 59)) )
                                .fill(egui::Color32::from_rgb(228, 244, 252)));
                            ui.add_sized([100.0, 35.0],egui::Button::new(
                                egui::RichText::new(adjusted_byte)
                                .color(egui::Color32::from_rgb(45, 51, 59)) )
                                .fill(egui::Color32::from_rgb(228, 244, 252))); 
                            ui.end_row();    
                        });    
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
 
                    let t_counter = format!("{}", &title); //. ▶
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

    fn configure_fonts(&mut self, _ctx: &egui::Context){
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
}

impl<'a> epi::App for Application<'a> {
    fn name(&self) -> &str {
        "FUgly Finder"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
        
        self.configure_fonts(ctx);
     
         
        let dfer = return_dfer2("/Users/matthew/temp/", self.filter_search_filetype);
         
        println!("dfer::length::{:?}", &dfer.data_set.len());
        let start = Instant::now();
        let d2 = filter_hashmap_by_filetype(dfer, enums::enums::FileType::All);
  
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: filter_hashmap_by_filetype {:?}", duration);

        //let mut flag_counters = [0;6]; 
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

        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: .iter()::{:?}", duration);
 
        self.ctrl_starting_directory = "/Users/matthew/zz/".to_string(); 
  
        self.b = d2;
        self.c = vec![];

         
        //*************************************************************//
 
        
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

        let my_frame1 = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 2., top: 5., bottom: 2. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::YELLOW },
            fill: Color32::from_rgb(83, 115, 146),
            stroke: egui::Stroke::new(0.0, Color32::GOLD),
        };

        let my_frame2 = egui::containers::Frame {
            margin: egui::style::Margin { left: 10., right: 2., top: 5., bottom: 2. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::YELLOW },
            fill: Color32::from_rgb(244,244,244),
            stroke: egui::Stroke::new(0.0, Color32::GOLD),
        };

        egui::TopBottomPanel::top("top_panel").frame(my_frame2).show(ctx, |ui| {
            ui.add_space(2.0);


            //USED FOR TESTING ALIGNMENT
           /*  ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| { 
                let flipped = ui.layout().horizontal_align() == egui::Align::LEFT;
            
                if ui
                    .add_sized([143.0, 1.0], egui::Button::new("Temp Files"))
                    .clicked()
                {
                    //do something
                }
            });
 */
           

            egui::Grid::new("top_menu_grid").show(ui, |ui| {
               
                if ui
                    .add( egui::Button::new( egui::RichText::new("⎈ Run").color(egui::Color32::from_rgb(0,191,255)),
                    ))
                    .clicked()
                {
                    let start = Instant::now();
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
                    let duration = start.elapsed();
                    println!("Time elapsed in expensive_function() is: (update) {:?}", duration);
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
                        ui.label("Extensions:: anything not covered by the other filters. Checking this box can significantly increase the search time.");
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

                    match res {
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

        egui::SidePanel::left("my_left_side_panel").frame(my_frame1).show(ctx, |ui| {
           
            //DropDown SortBy
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                self.drop_down_sort_by(ui);
            });
 
            //Menu Filters
            ui.with_layout(egui::Layout::from_main_dir_and_cross_align(
                egui::Direction::TopDown,
                egui::Align::LEFT 
            ), |ui| {
                //ui.set_height(20.);
                if ui
                    .add_sized([143.0, 100.0], egui::Button::new("All Files"))
                    .clicked()
                {
                    self.ctrl_filter_filetype = enums::enums::FileType::All;
                    self.status_filetype_counters = true;

                    let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                    self.a = d2;

                    // let modifiers = ui.ctx().input().modifiers;
                    // ui.ctx().output().open_url = Some(egui::output::OpenUrl {
                    //     url: "/Users/matthew/temp/IMG_0435.jpg".to_owned(),
                    //     new_tab: modifiers.any(),
                    // });

                }
            }); 
            //self.filters_filetype_counters
            let title = format!("{}::{}", "Audio", self.filters_filetype_counters[0]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Audio;
                self.status_filetype_counters = true;

                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;
            }

            let title = format!("{}::{}", "Documents", self.filters_filetype_counters[1]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Document;
                self.status_filetype_counters = true;

                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;
            }

            let title = format!("{}::{}", "Images", self.filters_filetype_counters[2]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Image;
                self.status_filetype_counters = true;

                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;
            } 

            let title = format!("{}:{}", "Others", self.filters_filetype_counters[3]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Other;
                self.status_filetype_counters = true;

                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;
            }

            let title = format!("{}::{}", "Videos", self.filters_filetype_counters[4]);
            if ui
                .add_sized([143.0, 100.0], egui::Button::new(title))
                .clicked()
            {
                self.ctrl_filter_filetype = enums::enums::FileType::Video;
                self.status_filetype_counters = true;

                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;
            }

            /* ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.style_mut().wrap = Some(true);

                ui.label("This text will be red, monospace, and won't wrap to a new line");
            }); // the temporary settings are reverted here */
        });

        egui::TopBottomPanel::bottom("bottom_panel").frame(my_frame2).show(ctx, |ui| {
            //ui.set_height(20.);
            //ui.add_space(15.);
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                self.delete_all(ui);
                 
             });
             ui.add_space(2.);
        });

        egui::CentralPanel::default().frame(my_frame2).show(ctx, |ui| {
              

            //let layout = egui::Layout::from_main_dir_and_cross_align(egui::Direction::TopDown, egui::Align::LEFT);
 
           /*  ui
                .scope(|ui| {
                    let background_frame =  egui::containers::Frame {
                        margin: egui::style::Margin { left: 0., right: 0., top: 0., bottom: 0. },
                        rounding: egui::Rounding { nw: 0.0, ne: 0.0, sw: 0.0, se: 0.0 },
                        shadow: eframe::epaint::Shadow { extrusion: 1.0, color: Color32::YELLOW },
                        fill: Color32::from_rgb(193, 195, 116),
                        stroke: egui::Stroke::new(0.0, Color32::GOLD),
                    };

    
                    //.multiply_with_opacity(config.background_alpha);
                    background_frame
                        .show(ui, |ui| {
                             ui.add_sized([300.0, 35.0], egui::Button::new("test"));
                        })
                        .inner
                })
                .inner;
            */


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
    let start = Instant::now();
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;
 
        if ft != enums::enums::FileType::All {
            v.retain(|x| x.file_type == ft);

            if v.len() == 0 {
                d2.data_set.remove(&k);
            }
        }
    }

    let duration = start.elapsed();
        println!("Time elapsed:: filter_hashmap_by_filetype::{:?}", duration);
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
 