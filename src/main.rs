use game_collection::ui::game_collection_app::GameCollectionApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "GameCollection",
        native_options,
        Box::new(|cc| Ok(Box::new(GameCollectionApp::new(cc)))),
    )
}
