use gtk::prelude::*;

use crate::ui_build_herlper_functions::constants::news_categories::CATEGORIES;

#[derive(Debug, Clone)]
pub struct SideBar {
    parent: adw::ToolbarView,
    listbox: gtk::ListBox,
}

impl SideBar {
    pub fn new(parent: &adw::ToolbarView) -> Self {
        SideBar {
            parent: parent.clone(),
            listbox: gtk::ListBox::builder()
                .selection_mode(gtk::SelectionMode::Single)
                .margin_top(20)
                .css_classes(vec!["navigation-sidebar", "rich-list"])
                .margin_start(12)
                .margin_end(12)
                .build(),
        }
    }

    pub fn connect_sidebar<F>(&self, on_click: F)
    where
        F: Fn(String) + 'static,
    {
        self.listbox.connect_row_activated(move |_, row| {
            let id = row.widget_name();
            if !id.is_empty() && id != "GtkListBoxRow" {
                on_click(id.to_string());
            }
        });
    }

    pub fn build(&self) -> Result<(), String> {
        let content = self.listbox.clone();
        self.build_search_btn();
        self.build_news_categories();
        self.build_bottom_components();

        self.parent.set_content(Some(&content));

        Ok(())
    }

    fn build_search_btn(&self) {
        let row_box = gtk::Box::builder()
            .spacing(4)
            .margin_start(12)
            .margin_end(12)
            .css_classes(vec!["Category"])
            .build();

        let icon = gtk::Image::from_icon_name("system-search-symbolic");
        icon.set_pixel_size(18);
        row_box.append(&icon);
        row_box.append(&gtk::Label::new(Some("Search")));

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&row_box));
        row.set_widget_name("search");
        self.listbox.clone().append(&row);
    }

    fn build_news_categories(&self) {
        for cat in CATEGORIES {
            let row_box = gtk::Box::builder()
                .spacing(4)
                .margin_start(12)
                .margin_end(12)
                .css_classes(vec!["Category"])
                .build();

            let icon = gtk::Image::from_icon_name(cat.icon);
            icon.set_pixel_size(18);
            row_box.append(&icon);
            row_box.append(&gtk::Label::new(Some(cat.name)));

            let row = gtk::ListBoxRow::new();
            row.set_child(Some(&row_box));
            row.set_widget_name(cat.id);
            self.listbox.clone().append(&row);
        }

        self.listbox.set_header_func(|row, before| {
            let id = row.widget_name();

            // 1. If this is the Search row, never give it a header
            if id == "search" {
                row.set_header(None::<&gtk::Widget>);
                return;
            }

            // 2. If the row BEFORE this one was "search",
            // it means THIS row is the first actual category.
            // This is where we put the "CATEGORIES" label.
            let is_first_category = before
                .as_ref()
                .map(|b| b.widget_name() == "search")
                .unwrap_or(false);

            if is_first_category {
                let header = gtk::Label::builder()
                    .label("CATEGORIES")
                    .halign(gtk::Align::Start)
                    .css_classes(vec!["caption-heading"])
                    .margin_start(12)
                    .margin_bottom(6)
                    .margin_top(12) // Add some space between Search and Categories
                    .build();

                row.set_header(Some(&header));
            } else {
                row.set_header(None::<&gtk::Widget>);
            }
        });
    }

    pub fn select_first_category(&self) {
        // Index 0 is "Search", so Index 1 is the first Category
        if let Some(first_cat_row) = self.listbox.row_at_index(1) {
            self.listbox.select_row(Some(&first_cat_row));

            // Optional: If you want to trigger the 'on_click' logic immediately
            // so the news loads on startup:
            first_cat_row.activate();
        }
    }

    fn build_bottom_components(&self) {
        let row_box = gtk::Box::builder()
            .spacing(4)
            .margin_start(12)
            .margin_end(12)
            .css_classes(vec!["Category"])
            .build();

        let icon = gtk::Image::from_icon_name("system-search-symbolic");
        icon.set_pixel_size(18);
        row_box.append(&icon);
        row_box.append(&gtk::Label::new(Some("Search")));

        let row = gtk::ListBoxRow::new();
        row.set_child(Some(&row_box));
        row.set_widget_name("search");

        self.parent.clone().add_bottom_bar(&row);
    }
}
