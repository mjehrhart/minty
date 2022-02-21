use crate::file::meta;

use super::*;
use egui::Image;
use egui::{CentralPanel, Color32, Context, ScrollArea};
use file::meta::*;
use std::sync::{Arc, Mutex};
use std::{array, borrow::Cow, collections::HashMap, error::Error};

use eframe::{
    egui,
    epi::{self, Frame, Storage},
};

use std::sync::mpsc::channel;
use std::thread;

extern crate byte_unit;
use byte_unit::{Byte, ByteUnit};

pub const PADDING: f32 = 5.0;

#[derive(Clone)]
pub struct Application<'a> {
    b: finder::finder::Finder<'a>,
    c: Vec<file::meta::Meta<'a>>,
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
        }
    }

    /* fn configure_fonts(&self, ctx: &egui::CtxRef) {
        let mut font_def = egui::FontDefinitions::default();
        font_def.font_data.insert(
            "Droid".to_owned(),
            egui::FontData::from_static(include_bytes!(
                "../Droid Sans Mono Nerd Font Complete Mono.otf"
            )),
        );

        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 25.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (egui::FontFamily::Proportional, 15.),
        );
        font_def
            .fonts_for_family
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Droid".to_string());
        ctx.set_fonts(font_def);
    } */

    fn drop_down_sort_by(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {});

        egui::Grid::new("grid_hide_singles")
            .striped(true)
            .num_columns(3)
            //.spacing(egui::Vec2::new(16.0, 20.0))
            .show(ui, |ui| {
                if egui::ComboBox::from_label("")
                    .show_index(
                        ui,
                        &mut self.sort_left_panel_index,
                        self.sort_left_panel.len(),
                        |i| self.sort_left_panel[i].to_owned(),
                    )
                    .clicked()
                {
                    println!("{:?}", self.sort_left_panel_index);
                };
                ui.label("Hide Singles");
                ui.add(toggle(&mut self.ctrl_skip_display_dupes));

                // ui.label("Move");
                // ui.add(toggle(&mut self.ctrl_view_mode));
                // ui.label("Delete");
                // ui.add(toggle(&mut self.ctrl_remove_mode));

                ui.end_row();
            });
    }

    fn left_side_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        //println!("\n\n{}", "updating left_side_panel()");
        let mut comparison_vec: Vec<(String, i32, String, Vec<Meta>, String)> = vec![];
        for mut item in self.b.data_set.iter_mut() {
            //TODO maybe make mutable
            let (k, v) = item;
            //println!("{}", k);
            if self.ctrl_skip_display_dupes == true {
                if v.len() > 1 {
                    ///////
                    comparison_vec.push((
                        v[0].name.to_string(),       //name
                        v.len().try_into().unwrap(), //number of files
                        k.to_string(),               //checksum
                        v.to_vec(),                  //list of files
                        k.to_string(),
                    ));
                }
            } else {
                comparison_vec.push((
                    v[0].name.to_string(),       //name
                    v.len().try_into().unwrap(), //number of files
                    k.to_string(),               //checksum
                    v.to_vec(),                  //list of files
                    k.to_string(),
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
            .max_height(400.)
            .stick_to_right()
            .show_rows(ui, row_height, num_rows, |ui, row_range| {
                //println!("main::row_range{:?}", row_range);
                for row in row_range {
                    let mut a_total = 0;
                    for item in &comparison_vec[row].3 {
                        a_total += item.file_size;
                    }

                    let byte = Byte::from_bytes(a_total.try_into().unwrap());
                    let adjusted_byte = byte.get_appropriate_unit(false);

                    let mut text_file_count = " ".to_string();
                    if comparison_vec[row].1 > 1 {
                        text_file_count = format!("{} files", comparison_vec[row].1);
                    }

                    let mut title: String;
                    title = format!(" 🖼  {}", comparison_vec[row].0); //▶
                    title = truncate(&title, 115).to_string();

                    let diff = 115 - title.chars().count() - 10;
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

                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.add_space(PADDING);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new(title)
                                        .color(egui::Color32::GRAY)
                                        .monospace(),
                                )
                                .fill(egui::Color32::from_rgb(27, 27, 27)),
                            )
                            .clicked()
                        {
                            // let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                            //     let image = load_image(include_bytes!("/Users/matthew/temp/foldera/t/IMG_0059.JPG")).unwrap();
                            //     ctx.load_texture("rust-logo", image)
                            // });
 
                            let image_path = comparison_vec[row].3[0].path.clone();
                            //self.create_image_texture(ctx, &image_path, ui);
                            self.selected_collection = comparison_vec[row].4.to_string();
                            self.c = comparison_vec[row].3.to_vec();

                            println!("self.image::{:?}", self.image);
                        }
                    });
                    //} //end of if else
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
                            //println!("derror::ui::mod.rs::10001{} ", e);
                            break;
                        }
                    }

                    let byte = Byte::from_bytes(row.file_size.try_into().unwrap());
                    let adjusted_byte = byte.get_appropriate_unit(false);

                    let mut title: String;
                    title = format!("{} ", row.path); //▶  ▶
                    title = truncate(&title, 93).to_string();

                    //'attemp to subtract with overflow'
                    let diff = 95 - title.chars().count();
                    let mut space = " ".to_string();
                    for _ in 0..diff {
                        space.push(' ');
                    }

                    title = [
                        title.to_string(),
                        space,
                        row.file_points.unwrap().to_string(),
                        date.unwrap(),
                        adjusted_byte.to_string(),
                    ]
                    .join(" ");

                    //********************************************************//

                    let t_counter = format!("{} ", ""); //. ▶
                    ui.horizontal(|ui| {
                        if ui.checkbox(&mut row.event_status, t_counter).clicked() {
                            //println!("row.event_status::{}", row.event_status);
                            if row.event_status == true {
                                row.status = FileAction::Delete;
                            } else {
                                row.status = FileAction::Read;
                            }

                            let collection =
                                self.b.data_set.get_mut(&self.selected_collection).unwrap();
                            for mut row2 in collection {
                                if row2.path == row.path {
                                    println!("\n{:#?}", 1);
                                    if row.event_status == true {
                                        //row2.set_status(FileAction::Delete);
                                        row2.status = FileAction::Delete;
                                        row2.event_status = true;
                                    } else {
                                        //row2.set_status(FileAction::Read);
                                        row2.status = FileAction::Read;
                                        row2.event_status = false;
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
                (x.status == FileAction::Empty)
                    || (x.status == FileAction::Read)
                    || (x.status == FileAction::Save)
            });

            //Remove file element from hashmap for gui second
            let collection = self.b.data_set.get_mut(&self.selected_collection).unwrap();
            collection.retain(|x| {
                (x.status == FileAction::Empty)
                    || (x.status == FileAction::Read)
                    || (x.status == FileAction::Save)
            });
        };
    }

    fn delete_all(&mut self, ui: &mut egui::Ui) {
        if ui
            .add(egui::Button::new(
                egui::RichText::new("Delete All Checked")
                    .color(egui::Color32::DARK_RED)
                    .monospace(),
            ))
            .clicked()
        {
            //Remove file from os
            for collection in &self.b.data_set {
                let (_, v) = collection;
                for item in v {
                    if item.status == FileAction::Delete {
                        std::fs::remove_file(&item.path).ok();
                    }
                }
            }

            for collection in self.b.data_set.clone() {
                let (mut k, mut v) = collection;
                let mut cc = self.b.data_set.get_mut(&k.to_string()).unwrap();

                cc.retain(|x| {
                    (x.status == FileAction::Empty)
                        || (x.status == FileAction::Read)
                        || (x.status == FileAction::Save)
                });
            }
        };
    }

    /* fn create_image_texture2(&mut self, _frame: &epi::Frame, path: &str) -> std::io::Result<()> {
           //ui.image(self.texture, self.texture_size);
           //ui.add(egui::ImageButton::new(self.texture.unwrap(), self.texture_size));
           //if self.texture.is_none() {
           // Load the image:

           let f = std::fs::File::open(path)?;
           let mut reader = std::io::BufReader::new(f);
           let mut buffer = Vec::new();

           // Read file into vector.
           std::io::Read::read_to_end(&mut reader, &mut buffer)?;

           let image_data = buffer;
           use image::GenericImageView;
           let image = image::load_from_memory(&image_data).expect("Failed to load image");
           let image_buffer = image.to_rgba8();
           let pixels = image_buffer.into_vec();
           let size = [(image.width() / 1) as usize, (image.height() / 1) as usize];

           let image = epi::Image::from_rgba_unmultiplied(size, &pixels);

           // Allocate a texture:d
           self.texture = Some(_frame.alloc_texture(image));
           self.texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);
           //ui.image(self.texture.unwrap(), self.texture_size);

           //}
           Ok(())
       }
    */

    //std::io::Error
    async fn create_image_texture(&mut self, path: &str) -> std::io::Result<(Vec<u8>,std::io::Error)> {
        println!("async fn create_image_texture(&mut self, path: &str) -> std::io::Result<(Vec<u8>,std::io::Error)>");
        //ui: &mut egui::Ui,
        //ctx: &egui::Context,
        //let a : Arc<Mutex<Server>> = Arc::new(Mutex::new(self));
        //let mut selfm = a.lock().unwrap();
 
        // Load the image:
        fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
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

        //TODO Concurrency Image

        //let (sync_sender:SyncSender<i32>, receiver) = std::sync::mpsc::sync_channel(2);
        ////let (tx, rx) = channel();

        // let mut image_data: Vec<u8>;
        let path: String = path.to_string();

        // let tx = tx.clone();
        // let handle3 = std::thread::spawn(move || {
        //     println!("Inside1 -> thread::spawn1");

        let f = std::fs::File::open(path);
        match f {
            Ok(f) => {
                let mut reader = std::io::BufReader::new(f);
                let mut buffer = Vec::new();
                println!("Inside2");
                // Read file into vector.
                std::io::Read::read_to_end(&mut reader, &mut buffer)?;

                //self.image = buffer;
                println!("byte::{:#?}", &buffer);
                //let mut image_data: Vec<u8> = buffer;

                return Ok((buffer,std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "F1l3 N0t f0und",
                )))

                //self.image = image_data.clone();
                //let image = load_image(&image_data).unwrap();

                //println!("Inside3");
                // let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                //     let image = load_image(include_bytes!("/Users/matthew/temp/foldera/t/IMG_0059.JPG")).unwrap();
                //     ctx.load_texture("rust-logo", image)
                // });

                //println!("Inside4");
                //ui.image(texture, texture.size_vec2());
                //tx.send(image).unwrap();
 
            }
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "F1l3 N0t f0und",
                ))
            }
        }
 
        //});
        // let recieved: std::sync::mpsc::Receiver<i32> = rx.recv().unwrap();
        // handle3.join().unwrap();
        //End Concurrency
  
        //println!("Inside4."); 

        // Allocate a texture:d
        ////self.texture = Some(_frame.alloc_texture(recieved));
        //self.texture_size = egui::Vec2::new((size[0] / 20) as f32, (size[1] / 20) as f32);
        //ui.image(self.texture.unwrap(), self.texture_size);
 
        //Ok(())
    }
}

impl<'a> epi::App for Application<'a> {
    fn name(&self) -> &str {
        "FUgly Finder"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn Storage>) {
        //self.configure_fonts(ctx);

        let fa: f64 = self.ctrl_chunk_size;
        let mut ua: u32 = 0;
        ua = (fa.round() as u32);
        let chunk_size = ua as usize;
        let flag_remove: bool = false;
        let flag_view: bool = false;

        let haystack = "/Users/matthew/temp/foldera/t"; 
        let dfer = return_dfer2(
            haystack.to_string(),
            chunk_size.try_into().unwrap(),
            flag_view,
            flag_remove,
        );

        self.ctrl_view_mode = dfer.flag_view;
        self.ctrl_remove_mode = dfer.flag_remove;
        self.ctrl_starting_directory = dfer.starting_directory.to_string();
        self.ctrl_bucket = dfer.bucket;
        self.b = dfer;
        self.c = vec![];

        //*************************************************************//
        //self.create_image_texture(ctx, "/Users/matthew/temp/foldera/t/IMG_0059.JPG", ui);

        let x = Arc::new(Mutex::new(Application {
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

        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(35.);

            ui.with_layout(
                egui::Layout::from_main_dir_and_cross_align(
                    egui::Direction::RightToLeft,
                    egui::Align::LEFT,
                ),
                |ui| {
                    ui.vertical(|ui| {
                        //DropDown SortBy
                        self.drop_down_sort_by(ui);

                        // if !self.texture.is_none() {
                        //     // ui.add(egui::ImageButton::new(
                        //     //     self.texture.unwrap(),
                        //     //     self.texture_size,
                        //     // ));
                        // }

                        //MaibPanel
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.label(" ");
                },
            );

            egui::Grid::new("some_unique_id").show(ui, |ui| {
                ui.set_min_height(25.);
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Test")
                            .color(egui::Color32::DARK_GREEN)
                            .monospace(),
                    ))
                    .clicked()
                {
                    println!("\n\n\nself.c{:#?}", &self.c);
                }
                if ui
                    .add(egui::Button::new(
                        egui::RichText::new("Open")
                            .color(egui::Color32::GREEN)
                            .monospace(),
                    ))
                    .clicked()
                {
                    println!("fn open()");
                    let path = std::env::current_dir().unwrap();
                    let res = rfd::FileDialog::new()
                        // .add_filter("text", &["txt", "rs"])
                        .set_directory(&path)
                        .pick_folder();

                    let folder = match res {
                        Some(_) => {
                            self.c = vec![];

                            let f = res.unwrap().clone().into_os_string().into_string();
                            let fa: f64 = self.ctrl_chunk_size;
                            let mut ua: u32 = 0;
                            ua = (fa.round() as u32);
                            let chunk_size = ua as usize;

                            let dfer = return_dfer2(
                                f.unwrap(),
                                chunk_size.try_into().unwrap(),
                                self.ctrl_view_mode,
                                self.ctrl_remove_mode,
                            );

                            self.ctrl_view_mode = dfer.flag_view;
                            self.ctrl_remove_mode = dfer.flag_remove;
                            self.ctrl_starting_directory = dfer.starting_directory.to_string();
                            self.ctrl_bucket = dfer.bucket;
                            self.b = dfer;
                        }
                        None => (),
                    };
                }
                ui.add(
                    egui::Slider::new(&mut self.ctrl_chunk_size, (1024.0 * 4.0)..=(80960.0 * 5.0))
                        .logarithmic(false)
                        .clamp_to_range(true)
                        .smart_aim(true)
                        .text("Byte Size"),
                );
                /* egui::Grid::new("textures")
                .striped(true)
                .num_columns(4)
                //.spacing(egui::Vec2::new(16.0, 20.0))
                .show(ui, |ui| {
                    ui.label("Hide");
                    ui.add(toggle(&mut self.ctrl_skip_display_dupes));
                    ui.label("Move");
                    ui.add(toggle(&mut self.ctrl_view_mode));
                    // ui.label("Delete");
                    // ui.add(toggle(&mut self.ctrl_remove_mode));

                    ui.end_row();
                }); */
                ui.label(self.ctrl_starting_directory.to_string());
                ui.label("");
                ui.end_row();
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(5.);
            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                self.delete_all(ui);
                ui.add_space(5.);
            });
        });

        // Resize the native window to be just the size we need it to be:
        //_frame.set_window_size(ctx.used_size());
    }
}

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
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
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

fn load_image(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
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
