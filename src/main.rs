mod enums;
mod file;
mod finder;
mod gui;

use crate::gui::controller::Application;
use home::home_dir;

use crate::enums::enums::FileAction;
use futures::executor; // 0.3.1
use image;
use std::{sync::Arc, time::Instant};

#[tokio::main]
async fn main() { 

    let icon_data2 = open_icon_data("/Users/matthew/dev/projects/minty/merged.png");
     

    let icon_bytes = include_bytes!("/Users/matthew/dev/projects/minty/merged.png");
    let icon = load_icon(&icon_bytes.to_vec());

    //let mut options = eframe::NativeOptions::default();
    let mut options = eframe::NativeOptions {
        icon_data: icon_data2,
        ..Default::default()
    };

    options.initial_window_size = Some(egui::Vec2::new(1300.0, 800.0));
    eframe::run_native(Box::new(gui::controller::Application::default()), options);

    //*************************************************************************************************************************************/
}

pub fn open_icon_data(path: &str) -> std::option::Option<eframe::epi::IconData> {
    let image_buffer = image::open(path).unwrap();
    let img = image_buffer.to_rgba8();
    let size = (img.width() as u32, img.height() as u32);
    let pixels = img.into_vec();
    let icon_data = eframe::epi::IconData {
        rgba: pixels.clone(),
        width: size.0,
        height: size.1,
    };
    //println!("{:?}", pixels);
    println!("{:?}", size.0);
    println!("{:?}", size.1);
    Some(icon_data)
}

pub fn load_icon(icon_bytes: &Vec<u8>) -> Option<eframe::epi::IconData> {
    if let Ok(image) = image::load_from_memory(icon_bytes) {
        let image = image.to_rgba8();
        let (width, height) = image.dimensions();
        Some(eframe::epi::IconData {
            width,
            height,
            rgba: image.as_raw().to_vec(),
        })
    } else {
        None
    }
}

fn return_dfer2(path: &str, filters: [bool; 5]) -> finder::finder::Finder {
    let mut ff = finder::finder::Finder::new();

    //let x =ff.fast_walk_dir(path).await;

    //**** Testing File Type Filtering for Search */
    //array =  [flag_audio,flag_document,flag_image,flag_other,flag_video]
    // let mut filters = [true; 5];
    // filters[2] = false; //ignore images (ie test)
    //Block to connect to async values

    executor::block_on(ff.rayon_walk_dir(path, filters));

    ff.adjust_file_order();
    ff
}

#[allow(dead_code)]
fn filter_hashmap_by_filetype(
    mut d2: finder::finder::Finder,
    ft: enums::enums::FileType,
) -> finder::finder::Finder {
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;

        v.retain(|x| x.file_type == ft);

        if v.is_empty() {
            d2.data_set.remove(&k);
        }
    }

    d2
}

#[allow(dead_code)]
fn spawnings() {
    let counter = Arc::new(std::sync::Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
