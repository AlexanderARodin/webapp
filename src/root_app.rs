
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    txt: String,

    #[serde(skip)]
    pressed: bool,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(), pressed:false}
    }
}

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            println!("tryin to load..");
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}


impl eframe::App for RootApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        println!("saving..");
        eframe::set_value(storage, eframe::APP_KEY, self);
        println!("..saved");
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.label("WWWapp Template v1.07");
            ui.horizontal( |ui| {
                let btn = ui.button( "try to save TEXT" );
                ui.label( format!(" <{}>", self.pressed) );
                if btn.clicked(){
                    println!("clicked with PRESSURE!!!");
                    self.pressed = true;
            }
            });
            ui.text_edit_singleline(&mut self.txt);
            ui.label( format!("just edited: [{}]", self.txt) );
        });
    }
}
