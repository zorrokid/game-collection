#[derive(Default, PartialEq, Clone)]
pub struct SoftwareTitle {
    pub id: u32,
    pub name: String,
    pub releases: Vec<SoftwareRelease>,
}

pub struct System {
    pub id: u32,
    pub name: String,
}

#[derive(Default, PartialEq, Clone)]
pub struct SoftwareRelease {
    pub id: u32,
    pub software_title_id: u32,
    pub system_id: u32,
    pub release_name: Option<String>,
}

pub struct SoftwareTitleListModel {
    pub id: u32,
    pub name: String,
}
