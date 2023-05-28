
struct DomikView {
    title: String,
}
impl Default for DomikView {
    fn default() -> Self {
        Self{ title: "simple DoMiKkk".to_owned() }
    }
}
impl DomikView {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn updateUI(&mut self, ui: &mut egui::Ui ) {
            ui.label("WWWapp Template v2.00");
            let btn = ui.button( "try to save TEXT" );
            if btn.clicked(){
                println!("clicked simple!");
            }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct RootApp {
    txt: String,

    #[serde(skip)]
    pressed: bool,

    #[serde(skip)]
    domikView: DomikView,
}

impl Default for RootApp {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(), pressed:false, domikView: DomikView::new() }
    }
}

impl RootApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {

        egui::Window::new("simple DoMiKkkk").show( ctx, |ui| {
            self.domikView.updateUI( ui );
        });

        egui::Window::new("tst wnd").show( ctx, |ui| {
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
