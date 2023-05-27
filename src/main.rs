#![allow(non_snake_case)]

mod root_app;
use crate::root_app::*;


#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    println!("MAIN has beed entered..");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };

    eframe::run_native(
        "some text 1",
        options,
        Box::new( |cc| Box::new(RootApp::new(cc)) )
   )
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    println!("in WASM doen't work..");

    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "raa_canvas_id",
            options,
            Box::new( |cc| Box::new(RootApp::new(cc)) ),
        )
        .await
        .expect("failure with starting EFRAME");
    });
}


