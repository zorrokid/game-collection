use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GameCollection",
        native_options,
        Box::new(|cc| Ok(Box::new(GameCollectionApp::new(cc)))),
    )
}

#[derive(Default)]
struct GameCollectionApp;

impl GameCollectionApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GameCollectionApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Game Collection");
        });
    }
}
