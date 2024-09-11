use crate::models::{game_collection_state::GameCollectionState, software_title::SoftwareTitle};
use eframe::egui;

#[derive(Default, PartialEq)]
enum UiMode {
    #[default]
    Show,
    Add,
}

#[derive(Default)]
pub struct UiState {
    pub ui_mode: UiMode,
}

#[derive(Default)]
pub struct GameCollectionApp {
    state: GameCollectionState,
    ui_state: UiState,
    new_software_title: Option<SoftwareTitle>,
}

impl GameCollectionApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for GameCollectionApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    if ui.button("Add software title").clicked() {
                        self.ui_state.ui_mode = UiMode::Add;
                        self.new_software_title = Some(SoftwareTitle::default());
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                            for software_title in &self.state.software_titles {
                                ui.label(&software_title.name);
                            }
                        });
                    });
                });
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    if self.ui_state.ui_mode == UiMode::Add {
                        if let Some(new_software_title) = &mut self.new_software_title {
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut new_software_title.name);
                            if ui.button("Save").clicked() {
                                if let Some(new_software_title) = self.new_software_title.take() {
                                    self.state.software_titles.push(new_software_title);
                                    self.ui_state.ui_mode = UiMode::Show;
                                }
                            }
                        }
                    }
                    ui.label("Hello, world!");
                });
            });
        });
    }
}
