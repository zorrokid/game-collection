use super::software_title_pane::SoftwareTitlePane;
use crate::models::software_title::SoftwareTitle;
use crate::repositories::software_title_repository::SoftwareTitleRepository;

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
    ui_state: UiState,
    new_software_title: Option<SoftwareTitle>,
    selected_software_title_id: Option<u32>,
    software_title_repository: SoftwareTitleRepository,
}

impl<'a> GameCollectionApp {
    pub fn new(_cc: &eframe::CreationContext<'a>) -> Self {
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
                            for software_title in self
                                .software_title_repository
                                .get_software_title_list_models()
                            {
                                ui.selectable_value(
                                    &mut self.selected_software_title_id,
                                    Some(software_title.id),
                                    software_title.name.as_str(),
                                );
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
                                    let id = self
                                        .software_title_repository
                                        .add_software_title(&new_software_title);
                                    self.ui_state.ui_mode = UiMode::Show;
                                    self.selected_software_title_id = Some(id);
                                }
                            }
                        }
                    }
                    if self.ui_state.ui_mode == UiMode::Show {
                        if let Some(selected_software_title_id) = self.selected_software_title_id {
                            SoftwareTitlePane::new(
                                &mut self.software_title_repository,
                                selected_software_title_id,
                            )
                            .show(ui);
                        }
                    }
                });
            });
        });
    }
}
