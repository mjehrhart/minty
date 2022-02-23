mod enums;
mod file;
mod finder;
mod gui;

use crate::enums::enums::FileAction;
use futures::executor; // 0.3.1
use std::{
    sync::Arc,
    time::{Instant},
};

fn return_dfer2(path: &str, filters:[bool;5]) -> finder::finder::Finder { 

    let mut ff = finder::finder::Finder::new();

    //let x =ff.fast_walk_dir(path).await; 
    //**** Testing File Type Filtering for Search */
    //array =  [flag_audio,flag_document,flag_image,flag_other,flag_video]
            // let mut filters = [true; 5];
            // filters[2] = false; //ignore images (ie test)
    //Block to connect to async values
     executor::block_on(ff.rayon_walk_dir(&path, filters));

    ff.adjust_file_order();
    ff 
}

fn filter_hashmap_by_filetype(
    mut d2: finder::finder::Finder,
    ft: enums::enums::FileType,
) -> finder::finder::Finder {
    for collection in d2.data_set.clone().into_iter() {
        let (k, mut v) = collection;

        v.retain(|x| x.file_type == ft);

        if v.len() == 0 {
            d2.data_set.remove(&k);
        }
    }

    d2
}

#[tokio::main]
async fn main() {
    //let start = Instant::now();
    let mut flag_filters = [true;5];
    flag_filters[3] = false;
    let dfer = return_dfer2("/Users/matthew/Documents/", flag_filters);
    let d2 = filter_hashmap_by_filetype(dfer, enums::enums::FileType::Document);
    //println!("#len::{:#?}", d2.data_set.len());

    //  flag_counters =  [flag_audio,flag_document,flag_image,flag_other,flag_video]
    let mut flag_counters = [0;6];
    for collection in d2.data_set.iter(){
        let (_,v) = collection;

        for row in v{
            match row.file_type{
                enums::enums::FileType::Audio => {
                    flag_counters[0] += 1;
                } 
                enums::enums::FileType::Document => {
                    flag_counters[1] += 1;
                },
                enums::enums::FileType::Image => {
                    flag_counters[2] += 1;
                }, 
                enums::enums::FileType::Other => {
                    flag_counters[3] += 1;
                },
                enums::enums::FileType::Video => {
                    flag_counters[4] += 1;
                },
                enums::enums::FileType::None => {},
                enums::enums::FileType::All => {},
            }
        }
    }
    //println!("#flag_counters::{:#?}",flag_counters );
    //let duration = start.elapsed();

    //*************************************************************************************************************************************/
    //sandbox();

    //*************************************************************************************************************************************/
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::Vec2::new(1300.0, 750.0));
    eframe::run_native(Box::new(gui::Application::default()), options);

    //*************************************************************************************************************************************/
    //println!("#collections::{:#?}", dfer.data_set );

    //println!("Time elapsed in expensive_function() is: {:?}", duration);
}

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
