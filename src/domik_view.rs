//
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
