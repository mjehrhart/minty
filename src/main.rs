mod app;
mod enums;
mod file;
mod finder;

use futures::executor; // 0.3.1

fn main() {
    // let sd = "/Users/matthew/zz/file_types";
    // let filter = [true, true, false, false, true]; 
    // let x = return_dfer2(sd, filter);
    // println!("x => {:?}", x);

    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(egui::Vec2::new(1200.0, 800.0)),
        decorated: true,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(app::controller::Application::default()), options);
}

fn return_dfer2(path: &str, filters: [bool; 5]) -> finder::finder::Finder {
    let mut ff = finder::finder::Finder::new();
    //Block to connect to async values
    executor::block_on(ff.rayon_walk_dir(path, filters));

    ff.adjust_file_order();

    ff
}
