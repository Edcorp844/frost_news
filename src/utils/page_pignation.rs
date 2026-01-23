use std::{collections::BTreeMap, sync::Arc};

use crate::types::news_article::NewsArticle;

#[derive(Debug, Clone)]
pub struct NewsPagination {
    pub pages: std::collections::BTreeMap<i32, BTreeMap<String, Vec<Arc<dyn NewsArticle>>>>,
    pub current_page: i32,
    pub total_results: usize,
}

impl NewsPagination {
    pub fn new() -> Self {
        Self {
            pages: std::collections::BTreeMap::new(),
            current_page: 1,
            total_results: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pages = std::collections::BTreeMap::new();
        self.current_page = 1;
        self.total_results = 0
    }
}
