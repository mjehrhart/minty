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

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use home;

extern crate byte_unit;
use byte_unit::{Byte};

#[derive(Clone, Debug )]
pub struct DupeTable {
    name: String,
    count: i32,
    checksum: String,
    list: Vec<Meta>, 
    file_type: enums::enums::FileType
}

//TODO check self.b and self.a references!!!
//#![windows_subsystem = "windows"]

#[derive(Clone)]
pub struct Application<'a> {
    //scroll_area: Option<egui::containers::scroll_area::ScrollAreaOutput<()>>,
    time_elapsed: std::time::Duration, 
    fuzzy_search: String,
    e: Vec<DupeTable>, 
    staging: Vec<Vec<DupeTable>>,
    selected_staging_index: usize,
    a: finder::finder::Finder,
    b: finder::finder::Finder,
    c: Vec<file::meta::Meta>, 
    selected_collection: String,
    sort_left_panel: [&'a str; 3],
    sort_left_panel_index: usize, 
    pager_size: Vec<usize>,
    pager_size_index: usize,
    ctrl_skip_display_dupes: bool,
    ctrl_starting_directory: String, 
    ctrl_filter_filetype: enums::enums::FileType,
    filter_search_filetype: [bool; 5],
    filters_filetype_counters: [i32;6],
    status_filetype_counters: bool,
    theme_prefer_light_mode: bool,
}

//move into own page
impl<'a> Application<'_> {
    pub fn default() -> Self {
        Self { 
            //scroll_area: None,
            time_elapsed: std::time::Duration::new(0, 0),
            fuzzy_search: String::from(""),
            e: vec![], 
            staging: vec![], 
            selected_staging_index: 0,
            a: finder::finder::Finder::new(),
            b: finder::finder::Finder::new(),
            c: vec![file::meta::Meta::new()], 
            selected_collection: String::from(""),
            sort_left_panel: ["Duplicates", "Name", "Size"],
            pager_size: [3, 5, 10, 1_000, 10_000, 25_000, 35_000, 50_000, 100_000].to_vec(),
            pager_size_index: 5,
            sort_left_panel_index: 0, 
            ctrl_starting_directory: "".to_string(), 
            ctrl_skip_display_dupes: false,
            ctrl_filter_filetype: enums::enums::FileType::All,
            filter_search_filetype: [true, true, true, false, true],     // [flag_audio,flag_document,flag_image,flag_other,flag_video]
            filters_filetype_counters: [0;6],                            // [flag_audio,flag_document,flag_image,flag_other,flag_video]; flag_all
            theme_prefer_light_mode: true,
            status_filetype_counters: false,
        }
    }
 
    fn drop_down_sort_by(&mut self, ui: &mut egui::Ui) {
  
        ui.horizontal(|ui| {
            if egui::ComboBox::new("siome123","")
                    .width(100.0) 
                    .show_index(
                        ui,
                        &mut self.sort_left_panel_index,
                        self.sort_left_panel.len(),
                        |i| self.sort_left_panel[i].to_owned(),
                    )
                    .clicked()
                { 
                }; 
                ui.label("Hide Singles"); 
                ui.add(toggle(&mut self.ctrl_skip_display_dupes));
                 
                ui.label("Page Size"); 
                if egui::ComboBox::new("siome123d","")
                    .width(100.0) 
                    .show_index(
                        ui,
                        &mut self.pager_size_index,
                        self.pager_size.len(), 
                        |i| self.pager_size[i].to_owned().to_string(),
                    )
                    .clicked()
                { 
                };

                self.fuzzy_search_ui(ui);

                //Search Time Duration
                ui.label("Search Duration" ); 
                ui.scope(|ui| { 
                    ui.visuals_mut().override_text_color = Some(egui::Color32::DARK_GREEN);   
                    let t = format!("{:?} seconds", self.time_elapsed.as_secs_f64().to_string());
                    ui.label(t );
                }); // the temporary settings are reverted here
                     
        });
 
            ui.add_space(4.);
    }

    fn fuzzy_search_ui(&mut self, ui: &mut egui::Ui){
        ui.label("Filter"); 
    
        ui.scope(|ui| { 
            ui.style_mut().wrap = Some(true);
            ui.visuals_mut().extreme_bg_color = egui::Color32::from_rgb(230,230,230);       

            let response = ui.add( egui::TextEdit::singleline(&mut self.fuzzy_search).code_editor()
                .desired_width(300.));

            if response.changed() {
                //Do nothing here
            }
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                println!("lost focus");
                println!("{:?}", &self.fuzzy_search);
 
                //self.selected_staging_index

                if !self.staging.is_empty(){
 
                    let vec = &self.configure_comparison_vec(vec![]); 

                    let matcher = SkimMatcherV2::default();
                    let  mut vec_temp:Vec<DupeTable> = vec![];
                    for dt in vec{
                        let res = matcher.fuzzy_match(&dt.name, &self.fuzzy_search); 
                        match res{
                            Some(_) => { 
                                vec_temp.push(dt.clone());
                        }
                            None => {},
                        }
                    }

                    println!("vec_temp {:#?}", vec_temp );

                    //Reset Pager
                    self.selected_staging_index = 0;
                    
                    //Step next  
                    //self.e  = vec_temp; //self.configure_comparison_vec(vec![]);  
                    //Application::<'a>::sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut self.staging[self.selected_staging_index]);
                    
                    self.staging = vec![ vec_temp ];
                }


            }
        }); 
    }
  
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
 
    fn pager( &mut self, ui: &mut egui::Ui){

 
        let main_dir = egui::Direction::LeftToRight;
        let layout = egui::Layout::left_to_right().with_main_wrap(true).with_cross_align(egui::Align::Center);
 
        egui::Grid::new("grid_main_labels") 
        .spacing(egui::Vec2::new(2.0, 0.0))
        .show(ui, |ui| {

            //TODO ui is showing extra button at the end
            for i in 0..self.staging.len() {
                if ui.add_sized([40.0, 35.0], egui::Button::new((i+1).to_string()))
                    .clicked()
                    {
                        self.selected_staging_index = i; 
                    }
            }
        }); 
    }

    fn left_side_panel(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {

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

            //adjusted_byte
            let diff = 10 - adjusted_byte.to_string().chars().count();  
            let mut space:String = String::from("");
            for _ in 0..diff {
                space.push(' ');

            }
            let adjusted_byte = [space, adjusted_byte.to_string()].join("");

            //text_file_count
            let diff = 12 - text_file_count.to_string().chars().count();  
            let mut space:String = String::from("");
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
 

            if vec_table.len() < self.pager_size[self.pager_size_index]{
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
            let sa = ScrollArea::vertical()
                .id_source("main_scroll")
                .auto_shrink([false, false]) 
                .max_height(500.)
                .stick_to_right()
                .show_rows(ui, row_height, num_rows, |ui, row_range| { 
                    for row in row_range {
                        
                        let (title, adjusted_byte, file_count) = get_table_fields(  vec_table[row].clone()  ); 
    
                        egui::Grid::new("grid_main_labels")
                            .striped(true)
                            .num_columns(3)
                            .spacing(egui::Vec2::new(0.0, 10.0))
                            .show(ui, |ui| {
                                if ui
                                    .add_sized([970.0, 35.0],egui::Button::new(
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
            } ); //end of scroll
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

    fn set_file_type_button<'b>(&mut self, ui: &mut egui::Ui, title: &str, index: usize){
         
        let mut text = format!("{}::{}", title, self.filters_filetype_counters[index]);
        if index == 5 {
            text = title.to_string();
        }
         
            if ui
                .add_sized([140.0, 35.0], egui::Button::new(text))
                .clicked()
            {
                match index {
                    0 => self.ctrl_filter_filetype = enums::enums::FileType::Audio,
                    1 => self.ctrl_filter_filetype = enums::enums::FileType::Document,
                    2 => self.ctrl_filter_filetype = enums::enums::FileType::Image,
                    3 => self.ctrl_filter_filetype = enums::enums::FileType::Other,
                    4 => self.ctrl_filter_filetype = enums::enums::FileType::Video,
                    5 => self.ctrl_filter_filetype = enums::enums::FileType::All,
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
                self.e  = self.configure_comparison_vec(vec![]);  
                Application::<'a>::sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut self.staging[self.selected_staging_index]);
                 
                //println!("self.e {:?}", self.e);  

                //scroll_offset --> self.offset = Some(Vec2::new(0.0, offset));
                //scroll_area = scroll_area.vertical_scroll_offset(0.0);
                //ui.scroll_to_cursor(Some(egui::Align::TOP));
            }
    }
 
    fn configure_comparison_vec(&mut self, mut vec: Vec<DupeTable> ) -> Vec<DupeTable> {
   
        //Step 1
        for item in self.a.data_set.iter() {
        //for mut item in d2.data_set.clone().iter_mut() { 
            let (k, v) = item;  
            
            if self.ctrl_skip_display_dupes {
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
 
        //println!("PRE sort");
         match self.sort_left_panel_index {
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
        //println!("AFTER sort");
 
        //Step 2; Paging Vector Readiness
        //Reset self.staging
        self.staging.clear();
  
        let pager_size = self.pager_size[self.pager_size_index];
        if vec.len() >  pager_size{
            println!("step 2");
            let quot = vec.len()/ pager_size;
            let rem = vec.len() % pager_size;
            
            for i in 0..quot{

                let y = (i+1) * pager_size;
                let x = y-pager_size;

                println!("[x..y]: [{}..{}]",x,y);
                let v = vec[x..y].to_vec(); 
                self.staging.push(v.clone() );
            }
            //rem
            {
                let y = quot * pager_size;
                let x = y-rem;

                println!("![x..y]: [{}..{}]",x,y);
                let v = vec[y..].to_vec(); 
                self.staging.push(v.clone() );
            }
        } else {
            self.staging.push(vec[..].to_vec());
        }

        vec
    }

}

//maybe try to put the sort in configure_comparison_vec()

impl<'a> epi::App for Application<'a> {
    fn name(&self) -> &str {
        "FUgly"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
        let start = Instant::now();
        self.configure_fonts(ctx);
     
        let starting_dir = "/Users/matthew/zz/file_types/";
        //let starting_dir = "/Users/matthew/Library"; 
        //let starting_dir ="/Users/matthew/.Trash";
         
        let dfer = return_dfer2(starting_dir, self.filter_search_filetype); 
        println!("dfer::length::{:?}", &dfer.data_set.len());
         
        let d2 = filter_hashmap_by_filetype(dfer, enums::enums::FileType::All);  
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
        self.time_elapsed = duration;   
        self.ctrl_starting_directory = starting_dir.to_string();  
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
            margin: egui::style::Margin { left: 5., right: 5., top: 5., bottom: 2. },
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow { extrusion: 0.0, color: Color32::YELLOW },
            fill: Color32::from_rgb(244, 244, 244), //83, 115, 146
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
            ui.add_space(4.0);
  
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
                    //println!("Time elapsed in expensive_function() is: (update) {:?}", duration);
                    self.time_elapsed = duration;
                }

                if ui
                    .add( egui::Button::new( egui::RichText::new("🔵 Theme").color(egui::Color32::from_rgb(0,191,255)),
                    ))
                    .clicked()
                {
               
                    self.theme_prefer_light_mode = !self.theme_prefer_light_mode ;
                    if !self.theme_prefer_light_mode {
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
                    .clicked() { }
                if ui
                    .checkbox(&mut self.filter_search_filetype[1], "Documents")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | doc | docx | txt | xls | pdf | ppt | vcs | zip |");
                    })
                    .clicked() { }
                if ui
                    .checkbox(&mut self.filter_search_filetype[2], "Images")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | dds | jpg | jpeg | heic | heif | png | psd | tif | tiff| tga | thm |");
                    })
                    .clicked() { }
                if ui
                    .checkbox(&mut self.filter_search_filetype[3], "Other")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: anything not covered by the other filters. Checking this box can significantly increase the search time.");
                    })
                    .clicked() { } 
                if ui
                    .checkbox(&mut self.filter_search_filetype[4], "Videos")
                    .on_hover_ui(|ui| {
                        ui.label("Extensions:: | avi | mpg | mpeg | mov | mp4 |");
                    })
                    .clicked() { }

                //Open Directory
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Select Directory")
                             .color( egui::Color32::from_rgb(0,191,255) )
                            .monospace(),
                    ))
                    .clicked()
                { 
                    ui.ctx().output().cursor_icon = egui::CursorIcon::Wait;

                    //let path = std::env::current_dir().unwrap();
                    let path = home::home_dir().unwrap();
                    let res = rfd::FileDialog::new()
                        // .add_filter("text", &["txt", "rs"])
                        .set_directory(&path)
                        .pick_folder();

                    match res {
                        Some(_) => {
                            self.c = vec![]; 
                            let f = res.unwrap().into_os_string().into_string();  //res.unwrap().clone().into_os_string().into_string(); 
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
            //ui.add(sep);

            ui.add_space(4.0);
        });

        egui::SidePanel::left("my_left_side_panel").frame(my_frame1).show(ctx, |ui| {
           
            //DropDown SortBy 
            //self.drop_down_sort_by(ui); 
 
            //test 
            /* if ui
                .add_sized([143.0, 35.0], egui::Button::new("Test Me"))
                .clicked()
            { 
                //Step 0
                self.ctrl_filter_filetype = enums::enums::FileType::All;
                self.status_filetype_counters = true;

                //Step 1
                let d2 = filter_hashmap_by_filetype(self.b.clone(), self.ctrl_filter_filetype);
                self.a = d2;

                //Step next
                self.e  = self.configure_comparison_vec(vec![]);  

                //Application::<'a>::sort_dupe_table(self.sort_left_panel_index.try_into().unwrap(), &mut self.staging[self.selected_staging_index]);
                //Application::<'a>::sort_dupe_table(0, &mut self.staging[self.selected_staging_index] );
                //println!("self.staging {:#?}", self.staging);

            } */
   
            //Menu Filters
            self.set_file_type_button(ui, "All Files", 5);
            self.set_file_type_button(ui, "Audio", 0);
            self.set_file_type_button(ui, "Documents", 1);
            self.set_file_type_button(ui, "Images", 2);
            self.set_file_type_button(ui, "Other", 3);
            self.set_file_type_button(ui, "Videos", 4); 
        });

        egui::TopBottomPanel::bottom("bottom_panel").frame(my_frame2).show(ctx, |ui| {
       
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                self.delete_all(ui);
                 
             });
             ui.add_space(2.);
        });

        egui::CentralPanel::default().frame(my_frame2).show(ctx, |ui| {
              
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
                     
                        //DropDown SortBy 
                        self.drop_down_sort_by(ui);
                         

                        //MainPanel
                        self.left_side_panel(ui, ctx);

                        //Seperator
                        let sep = egui::Separator::default().spacing(5.);
                        ui.add(sep);

                        //Pager
                        self.pager(ui);
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

            if v.is_empty(){
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
 
            /* ui.scope(|ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
                ui.style_mut().wrap = Some(true);

                ui.label("This text will be red, monospace, and won't wrap to a new line");
            }); // the temporary settings are reverted here */


                /* ui
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
                             ui.add_sized([100.0, 35.0], egui::Button::new("test"));
                        })
                        .inner
                })
                .inner; */