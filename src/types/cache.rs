#![allow(dead_code)]

use gtk::gdk::Texture;
use gtk::glib::Bytes;
use gtk::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct ImageCache(Arc<Mutex<HashMap<String, Texture>>>);

impl ImageCache {
    /// Create a new empty image cache
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }

    /// Get an image from cache, returns None if not found
    pub fn get(&self, url: &str) -> Option<Texture> {
        self.0.lock().unwrap().get(url).cloned()
    }

    /// Insert an image into cache
    pub fn insert(&self, url: String, texture: Texture) {
        self.0.lock().unwrap().insert(url, texture);
    }

    /// Remove an image from cache
    pub fn remove(&self, url: &str) -> Option<Texture> {
        self.0.lock().unwrap().remove(url)
    }

    /// Clear all cached images
    pub fn clear(&self) {
        self.0.lock().unwrap().clear();
    }

    /// Get the number of cached images
    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.0.lock().unwrap().is_empty()
    }

    /// Check if an image is cached
    pub fn contains(&self, url: &str) -> bool {
        self.0.lock().unwrap().contains_key(url)
    }

    /// Get all cached URLs (useful for debugging)
    pub fn urls(&self) -> Vec<String> {
        self.0.lock().unwrap().keys().cloned().collect()
    }

    /// Get approximate memory usage of cached textures (in bytes)
    pub fn memory_usage(&self) -> usize {
        self.0
            .lock()
            .unwrap()
            .values()
            .map(|texture| {
                let width = texture.width() as usize;
                let height = texture.height() as usize;
                width * height * 4 // Approximate: 4 bytes per pixel (RGBA)
            })
            .sum()
    }

    /// Remove the least recently used images to stay under a memory limit
    pub fn prune(&self, max_memory_mb: usize) {
        let max_bytes = max_memory_mb * 1024 * 1024;
        let current_usage = self.memory_usage();

        if current_usage > max_bytes {
            // Simple implementation: clear all if over limit
            // For a real LRU, you'd need to track access times
            println!("Cache exceeds {} MB, clearing...", max_memory_mb);
            self.clear();
        }
    }

    /// Try to load from cache first, then from network if not cached
    pub async fn get_or_load<F, Fut>(
        &self,
        url: &str,
        load_fn: F,
    ) -> Result<Texture, Box<dyn std::error::Error>>
    where
        F: FnOnce(&str) -> Fut,
        Fut: std::future::Future<Output = Result<Texture, Box<dyn std::error::Error>>>,
    {
        // Check cache first
        if let Some(cached) = self.get(url) {
            return Ok(cached);
        }

        // Load from network
        let texture = load_fn(url).await?;

        // Cache the result
        self.insert(url.to_string(), texture.clone());

        Ok(texture)
    }
    /// Load texture from bytes and cache it
    pub fn insert_from_bytes(&self, url: String, bytes: &Bytes) -> Result<Texture, gtk::glib::Error> {
        match Texture::from_bytes(bytes) {
            Ok(texture) => {
                self.insert(url, texture.clone());
                Ok(texture)
            }
            Err(e) => Err(e),
        }
    }
}

// Convenience function for backward compatibility
pub fn create_image_cache() -> ImageCache {
    ImageCache::new()
}

// Default implementation
impl Default for ImageCache {
    fn default() -> Self {
        Self::new()
    }
}
