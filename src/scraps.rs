

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