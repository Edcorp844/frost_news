#[derive(Debug, Clone)]
pub struct RequestParameters {
    language: Option<String>,
    country: Option<String>,
    query: Option<String>,
    from: Option<String>,
    to: Option<String>,
    page_size: Option<i32>,
    page: Option<i32>,
    category: Option<String>,
    sort_by: Option<String>,
}

impl RequestParameters {
    pub fn new() -> Self {
        RequestParameters {
            language: None,
            country: None,
            query: None,
            from: None,
            to: None,
            page_size: None,
            page: None,
            category: None,
            sort_by: None,
        }
    }
}

impl RequestParameters {
    // --- SETTERS (Builder Pattern) ---
    // These take the inner value, wrap in Some(), and return Self for chaining.

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to = Some(to.into());
        self
    }

    pub fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    pub fn page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.sort_by = Some(sort_by.into());
        self
    }

    // --- GETTERS ---
    // Returning references to avoid unnecessary allocations when checking values.

    pub fn get_language(&self) -> Option<String> {
        self.language.clone()
    }

    pub fn get_country(&self) -> Option<String> {
        self.country.clone()
    }

    pub fn get_query(&self) -> Option<String> {
        self.query.clone()
    }

    pub fn get_from(&self) -> Option<String> {
        self.from.clone()
    }

    pub fn get_to(&self) -> Option<String> {
        self.to.clone()
    }

    pub fn get_page_size(&self) -> Option<i32> {
        self.page_size
    }

    pub fn get_page(&self) -> Option<i32> {
        self.page
    }

    pub fn get_category(&self) -> Option<String> {
        self.category.clone()
    }

    pub fn get_sort_by(&self) -> Option<String> {
        self.sort_by.clone()
    }
}
