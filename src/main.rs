#![allow(non_snake_case)]

mod log_view;
mod main_app;
use main_app::MainApp;

mod domik_view;
mod audio;
mod midi_lib;

mod raadbg;
use raadbg::log;



#[ cfg(not(target_arch = "wasm32")) ]
fn main() -> Result<(), eframe::Error> {
    log::simple("MAIN has beed entered ->");
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };

    {
        eframe::run_native(
            "egui CrossApp",
            options,
            Box::new( |cc| Box::new(MainApp::new(cc)) )
        )
    }
}


#[ cfg(target_arch = "wasm32") ]
fn main() {
    log::simple("wasmMAIN has beed entered ->");
    
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "raa_canvas_id",
            options,
            Box::new( |cc| Box::new(MainApp::new(cc)) ),
        )
        .await
        .expect("failure with starting EFRAME");
    });
}
