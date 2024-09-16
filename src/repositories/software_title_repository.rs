use crate::models::software_title::{SoftwareTitle, SoftwareTitleListModel};

#[derive(Default)]
pub struct SoftwareTitleRepository {
    software_titles: Vec<SoftwareTitle>,
}

impl SoftwareTitleRepository {
    pub fn get_software_titles(&self) -> &Vec<SoftwareTitle> {
        &self.software_titles
    }

    pub fn add_software_title(&mut self, software_title: &SoftwareTitle) -> u32 {
        let max_id = self
            .software_titles
            .iter()
            .map(|st| st.id)
            .max()
            .unwrap_or(0);

        let new_id = max_id + 1;
        let mut software_title = software_title.clone();
        software_title.id = new_id;
        self.software_titles.push(software_title.clone());
        new_id
    }

    pub fn get_software_title(&self, id: u32) -> Option<SoftwareTitle> {
        self.software_titles.iter().find(|st| st.id == id).cloned()
    }

    pub fn get_software_title_list_models(&self) -> Vec<SoftwareTitleListModel> {
        self.software_titles
            .iter()
            .map(|st| SoftwareTitleListModel {
                id: st.id,
                name: st.name.clone(),
            })
            .collect()
    }
}
