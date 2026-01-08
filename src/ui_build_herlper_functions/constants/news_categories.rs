use crate::types::news_category::NewsCategory;


pub struct Category {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub enum_name: &'static NewsCategory
}

pub const CATEGORIES: &[Category] = &[
    Category {
        id: "general",
        name: "Top Stories",
        icon: "emoji-objects-symbolic",
        enum_name: &NewsCategory::General
    },
    Category {
        id: "business",
        name: "Business",
        icon: "x-office-presentation-symbolic",
        enum_name: &NewsCategory::Business
    },
    Category {
        id: "health",
        name: "Health",
        icon: "face-sick-symbolic",
        enum_name: &NewsCategory::Health
    },
    Category {
        id: "entertainment",
        name: "Entertainment",
        icon: "applications-multimedia-symbolic",
        enum_name: &NewsCategory::Entertainment
    },
    Category {
        id: "technology",
        name: "Technology",
        icon: "computer-symbolic",
        enum_name: &NewsCategory::Technology
    },
    Category {
        id: "science",
        name: "Science",
        icon: "applications-science-symbolic",
        enum_name: &NewsCategory::Science
    },
    Category {
        id: "sports",
        name: "Sports",
        icon: "emoji-activities-symbolic",
        enum_name: &NewsCategory::Sports
    },
];

