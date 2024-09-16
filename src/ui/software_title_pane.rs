use crate::models::software_title::SoftwareTitle;
use crate::repositories::software_title_repository::SoftwareTitleRepository;
use eframe::egui;

pub struct SoftwareTitlePane<'a> {
    software_title_repository: &'a mut SoftwareTitleRepository,
    software_title: SoftwareTitle,
}

impl<'a> SoftwareTitlePane<'a> {
    pub fn new(
        software_title_repository: &'a mut SoftwareTitleRepository,
        software_title_id: u32,
    ) -> SoftwareTitlePane {
        let software_title = software_title_repository
            .get_software_title(software_title_id)
            .unwrap();
        SoftwareTitlePane {
            software_title_repository,
            software_title,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Name:");
        ui.label(self.software_title.name.clone());
    }
}
