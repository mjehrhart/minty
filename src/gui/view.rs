 
use crate::file::meta::Meta;
use crate::{finder::finder, };
use crate::{enums, };
//use file::meta::*;
//use super::*;
use super::controller::Application;

use egui::{ Color32, }; 
  use eframe::{
    egui,
    epi::{self, Storage},
};
 
extern crate byte_unit;
//use byte_unit::{Byte};
 
#[derive(Clone, Debug )]
pub struct DupeTable {
    pub name: String,
    pub count: i32,
    pub checksum: String,
    pub list: Vec<Meta>, 
    pub file_type: enums::enums::FileType
}

 
//#![windows_subsystem = "windows"] 
impl<'a> epi::App for Application<'a> {
    fn name(&self) -> &str {
        "Yafda"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
       
        self.configure_fonts(ctx);
     
        //let starting_dir = "/Users/matthew/Library"; 
        //let starting_dir ="/Users/matthew/.Trash";
        let starting_dir = "/Users/matthew/zz/file_types/";
        self.ctrl_starting_directory = starting_dir.to_string();
         
         /* 
        let start = std::time::Instant::now();
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
        self.b = d2;
        self.c = vec![]; */
         
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
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {


        fn filter_hashmap_by_filetype(
            mut d2: finder::Finder,
            ft: enums::enums::FileType,
        ) -> finder::Finder { 
            for collection in d2.data_set.clone().into_iter() {
                let (k, mut v) = collection;
        
                if ft != enums::enums::FileType::All {
                    v.retain(|x| x.file_type == ft);

                    if v.is_empty(){
                        d2.data_set.remove(&k);
                    }
                }
            }
        
            d2
        }

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
                    let start = std::time::Instant::now();
                    let dfer = crate::return_dfer2(&self.ctrl_starting_directory, self.filter_search_filetype); 
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

            //let sep = egui::Separator::default().spacing(5.);
            //ui.add_sized([143.0, 1.0], sep);
            //ui.add(sep);

            ui.add_space(4.0);
        });

        egui::SidePanel::left("my_left_side_panel").frame(my_frame1).show(ctx, |ui| {
 
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

 