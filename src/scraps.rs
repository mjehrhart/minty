

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

//ui.ctx().output().cursor_icon = egui::CursorIcon::Wait


//TODO: address permission Read issues that stem from here
//let mut perms = fs::metadata(&base_path).permissions()?;
//let metadata = std::fs::metadata(&base_path).unwrap();
//metadata.permissions().set_readonly(true);
//println!("{:?}", &base_path);
// perms.set_readonly(true);
// fs::set_permissions("foo.txt", perms)?;
// // Ok(())

//let temp = base_path.clone();

// let f = File::open("hello.txt");

// let f = match f {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {:?}", e),
//         },
//         other_error => {
//             panic!("Problem opening the file: {:?}", other_error)
//         }
//     },
// };



/* #[allow(dead_code)]
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
} */

/* #[allow(dead_code)]
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
} */
