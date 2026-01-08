#![allow(dead_code)]

use gtk::prelude::*;
use gtk::{gdk, cairo};

pub struct BlurredBottomImage {
    overlay: gtk::Overlay,
    picture: gtk::Picture,
}

impl Clone for BlurredBottomImage {
    fn clone(&self) -> Self {
        Self {
            overlay: self.overlay.clone(),
            picture: self.picture.clone(),
        }
    }
}


impl BlurredBottomImage {
    pub fn new() -> Self {
        let picture = gtk::Picture::builder()
            .content_fit(gtk::ContentFit::Cover)
            .halign(gtk::Align::Fill)
            .valign(gtk::Align::Fill)
            .hexpand(true)
            .vexpand(true)
            .build();

        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&picture));
        
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_vexpand(true);
        drawing_area.set_hexpand(true);
        drawing_area.set_valign(gtk::Align::End);
        
        // Simpler approach - just use gradient with strong opacity
        drawing_area.set_draw_func(|_, cr, width, height| {
            let width_f = width as f64;
            let height_f = height as f64;
            
            if width_f <= 0.0 || height_f <= 0.0 {
                return;
            }
            
            let blur_height = height_f * 0.7; // Increased to 70%
            let blur_y = height_f - blur_height;
            
            // Save context
            cr.save().unwrap();
            
            // Draw STRONG dark overlay (simulates blur by darkening)
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.75);
            cr.rectangle(0.0, blur_y, width_f, blur_height);
            cr.fill().unwrap();
            
            // Add subtle texture by drawing some noise dots
            cr.set_operator(cairo::Operator::Overlay);
            for y in 0..(blur_height as i32 / 4) {
                for x in 0..(width_f as i32 / 4) {
                    let noise_value = ((x * 17 + y * 23) % 100) as f64 / 400.0;
                    cr.set_source_rgba(1.0, 1.0, 1.0, noise_value);
                    cr.rectangle(
                        (x * 4) as f64,
                        blur_y + (y * 4) as f64,
                        1.5, 1.5
                    );
                    cr.fill().unwrap();
                }
            }
            
            cr.restore().unwrap();
            
            // Add VERY STRONG gradient for maximum text contrast
            let gradient = cairo::LinearGradient::new(
                0.0,
                blur_y,
                0.0,
                height_f,
            );
            
            // Extremely dark gradient
            gradient.add_color_stop_rgba(0.0, 0.0, 0.0, 0.0, 0.0);
            gradient.add_color_stop_rgba(0.1, 0.0, 0.0, 0.0, 0.7);  // 70% opaque at 10%
            gradient.add_color_stop_rgba(0.3, 0.0, 0.0, 0.0, 0.9);  // 90% opaque at 30%
            gradient.add_color_stop_rgba(0.6, 0.0, 0.0, 0.0, 0.97); // 97% opaque at 60%
            gradient.add_color_stop_rgba(1.0, 0.0, 0.0, 0.0, 1.0);  // Solid black at bottom
            
            let _ = cr.set_source(&gradient);
            cr.rectangle(0.0, blur_y, width_f, blur_height);
            cr.fill().unwrap();
            
            // Extra solid black at very bottom
            let solid_height = height_f * 0.25;
            cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
            cr.rectangle(0.0, height_f - solid_height, width_f, solid_height);
            cr.fill().unwrap();
        });
        
        overlay.add_overlay(&drawing_area);

        Self {
            overlay,
            picture,
        }
    }

    pub fn widget(&self) -> &gtk::Widget {
        self.overlay.as_ref()
    }

    pub fn set_paintable(&self, paintable: Option<&impl IsA<gdk::Paintable>>) {
        self.picture.set_paintable(paintable);
    }

    pub fn set_content_fit(&self, fit: gtk::ContentFit) {
        self.picture.set_content_fit(fit);
    }
}

impl Default for BlurredBottomImage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct BlurredBottomImageBuilder {
    content_fit: Option<gtk::ContentFit>,
}

impl BlurredBottomImageBuilder {
    pub fn build(self) -> BlurredBottomImage {
        let image = BlurredBottomImage::new();
        
        if let Some(fit) = self.content_fit {
            image.set_content_fit(fit);
        }
        
        image
    }

    pub fn content_fit(mut self, fit: gtk::ContentFit) -> Self {
        self.content_fit = Some(fit);
        self
    }
}