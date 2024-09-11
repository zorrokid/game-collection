use super::software_title::SoftwareTitle;

#[derive(Default)]
pub struct GameCollectionState {
    pub software_titles: Vec<SoftwareTitle>,
}
