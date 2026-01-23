use std::{collections::BTreeMap, sync::Arc};

use crate::types::news_article::NewsArticle;

pub trait NewsHandler {
    fn on_news_received(grouped: BTreeMap<String, Vec<Arc<dyn NewsArticle>>>) -> Self;
    fn on_error(err: String) -> Self;
}
