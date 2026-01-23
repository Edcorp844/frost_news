#[derive(Debug, Clone)]
pub enum NewsSection {
    General,
    Business,
    Health,
    Entertainment,
    Technology,
    Science,
    Sports,
}

impl NewsSection {
    pub fn to_key(&self) -> String {
        format!("{:?}", self)
    }
}
