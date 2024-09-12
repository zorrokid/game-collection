use super::software_title::SoftwareTitle;

#[derive(Default)]
pub struct GameCollectionState {
    software_titles: Vec<SoftwareTitle>,
}

impl GameCollectionState {
    pub fn get_software_titles(&self) -> &Vec<SoftwareTitle> {
        &self.software_titles
    }

    pub fn add_software_title(&mut self, software_title: SoftwareTitle) -> u32 {
        // TODO: temporary solution until saved to database
        let id = self.software_titles.len() as u32;
        let mut new_software_title = software_title.clone();
        new_software_title.id = id;
        self.software_titles.push(new_software_title);
        id
    }

    pub fn get_software_title(&self, id: u32) -> Option<SoftwareTitle> {
        self.software_titles.iter().find(|st| st.id == id).cloned()
    }
}
