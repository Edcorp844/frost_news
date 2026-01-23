use crate::types::news_category::NewsSection;

pub struct SectionData {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub enum_name: &'static NewsSection,
}

pub const SECTIONS: &[SectionData] = &[
    SectionData {
        id: "business",
        name: "Business",
        icon: "x-office-presentation-symbolic",
        enum_name: &NewsSection::Business,
    },
    SectionData {
        id: "health",
        name: "Health",
        icon: "face-sick-symbolic",
        enum_name: &NewsSection::Health,
    },
    SectionData {
        id: "entertainment",
        name: "Entertainment",
        icon: "applications-multimedia-symbolic",
        enum_name: &NewsSection::Entertainment,
    },
    SectionData {
        id: "technology",
        name: "Technology",
        icon: "computer-symbolic",
        enum_name: &NewsSection::Technology,
    },
    SectionData {
        id: "science",
        name: "Science",
        icon: "applications-science-symbolic",
        enum_name: &NewsSection::Science,
    },
    SectionData {
        id: "sports",
        name: "Sports",
        icon: "emoji-activities-symbolic",
        enum_name: &NewsSection::Sports,
    },
];