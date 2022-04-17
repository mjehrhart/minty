mod enums;
mod file;
mod finder;
mod gui;
 
use futures::executor; // 0.3.1
use image; 

#[tokio::main]
async fn main() { 
 
    let icon_bytes = include_bytes!("resources/merged.png");
    let icon_data = load_icon(&icon_bytes.to_vec());
 
    let options = eframe::NativeOptions {
        icon_data: icon_data,
        resizable: false,
        initial_window_size: Some(egui::Vec2::new(1300.0, 800.0)),
        decorated: true,
        transparent: true,
        ..Default::default()
    };
 
    eframe::run_native(Box::new(gui::controller::Application::default()), options);

    //*************************************************************************************************************************************/
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
    //Block to connect to async values 
    executor::block_on(ff.rayon_walk_dir(path, filters));

    ff.adjust_file_order();

    ff
}
