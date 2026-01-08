use std::sync::Arc;

use crate::newsdata::datap_structures::NewsDataArticle;


pub trait NewsArticle {
    fn author(&self) -> Option<String>;
    fn title(&self) -> String;
    fn description(&self) -> Option<String>;
    fn url(&self) -> String;
    fn url_to_image(&self) -> Option<String>;
    fn published_at(&self) -> String;
    fn content(&self) -> Option<String>;
    fn source(&self) -> String;
}


impl<T: NewsArticle + ?Sized> NewsArticle for Arc<T> {
    fn author(&self) -> Option<String> { (**self).author() }
    fn title(&self) -> String { (**self).title() }
    fn description(&self) -> Option<String> { (**self).description() }
    fn url(&self) -> String { (**self).url() }
    fn url_to_image(&self) -> Option<String> { (**self).url_to_image() }
    fn published_at(&self) -> String { (**self).published_at() }
    fn content(&self) -> Option<String> { (**self).content() }
    fn source(&self) -> String { (**self).source() }
}

impl NewsArticle for crate::news_api::data_structures::NewsAPIArticle {
    fn author(&self) -> Option<String> {
        self.author.clone()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn url_to_image(&self) -> Option<String> {
        self.url_to_image.clone()
    }

    fn published_at(&self) -> String {
        self.published_at.clone()
    }

    fn content(&self) -> Option<String> {
        self.content.clone()
    }

    fn source(&self) -> String {
        self.source.clone().name
    }
}



impl NewsArticle for crate::gnews::data_structures::GNewsArticle {
    fn author(&self) -> Option<String> {
        None
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn description(&self) -> Option<String> {
        Some(self.description.clone())
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn url_to_image(&self) -> Option<String> {
       Some( self.image.clone())
    }

    fn published_at(&self) -> String {
        self.published_at.clone()
    }

    fn content(&self) -> Option<String> {
        Some(self.content.clone())
    }

    fn source(&self) -> String {
        self.source.clone().name
    }
}

impl NewsArticle for NewsDataArticle{
    fn author(&self) -> Option<String> {
        None
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    fn url(&self) -> String {
        self.url.clone()
    }

    fn url_to_image(&self) -> Option<String> {
        self.image_url.clone()
    }

    fn published_at(&self) -> String {
        self.published_at.clone()
    }

    fn content(&self) -> Option<String> {
        self.content.clone()
    }

    fn source(&self) -> String {
        self.source_id.clone()
    }
}
