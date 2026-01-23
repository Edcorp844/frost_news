use adw::prelude::*;
use gtk::{gdk::Texture, gio};

use crate::types::cache::ImageCache;

#[derive(Clone)]
pub struct ImageLoader;

impl ImageLoader {
    pub fn new() -> Self {
        ImageLoader
    }

    pub async fn load_and_cache_image(
        &self,
        url: String,
        cache: ImageCache,
    ) -> Result<Texture, Box<dyn std::error::Error>> {
        cache
            .get_or_load(&url, |load_url| {
                let owned_url = load_url.to_string();
                async move {
                    // Load from network if not in cache
                    let (bytes, _) = gio::File::for_uri(&owned_url)
                        .load_bytes_future()
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

                    Texture::from_bytes(&bytes)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
                }
            })
            .await
    }

    pub fn load_picture_image(
        &self,
        picture: &gtk::Picture,
        url: Option<String>,
        cache: ImageCache,
    ) {
        if let Some(url) = url {
            let picture = picture.clone();
            let cache = cache.clone();
            let loader = self.clone(); // Clone self to move into async block

            relm4::spawn_local(async move {
                match loader.load_and_cache_image(url, cache).await {
                    Ok(texture) => {
                        picture.set_paintable(Some(&texture));
                    }
                    Err(_e) => {
                        // eprintln!("Failed to load image: {}", e);
                    }
                }
            });
        }
    }
}
