#![allow(dead_code)]

use gtk::prelude::*;
use gtk::{cairo, gdk};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;

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
            .hexpand(true)
            .vexpand(true)
            .build();

        let overlay = gtk::Overlay::new();
        overlay.set_child(Some(&picture));

        // Subtle dark scrim at the bottom to make text/content pop
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_valign(gtk::Align::End);
        drawing_area.set_draw_func(|_, cr, w, h| {
            let h = h as f64;
            let w = w as f64;
            let blur_h = h * 0.6;
            let y = h - blur_h;

            let grad = cairo::LinearGradient::new(0.0, y, 0.0, h);
            grad.add_color_stop_rgba(0.0, 0.0, 0.0, 0.0, 0.0);
            grad.add_color_stop_rgba(1.0, 0.0, 0.0, 0.0, 0.5); // 50% black at very bottom

            cr.set_source(&grad).unwrap();
            cr.rectangle(0.0, y, w, blur_h);
            cr.fill().unwrap();
        });

        overlay.add_overlay(&drawing_area);
        Self { overlay, picture }
    }

    pub fn widget(&self) -> &gtk::Widget {
        self.overlay.upcast_ref()
    }

    pub fn set_content_fit(&self, content_fit: gtk::ContentFit) {
        self.picture.set_content_fit(content_fit);
    }
    pub fn set_paintable(&self, texture: Option<&gdk::Texture>) {
        let Some(texture) = texture else { return };

        // 1. Download as Straight RGBA (Avoids premultiplication artifacts during math)
        let w = texture.width() as usize;
        let h = texture.height() as usize;
        let mut pixels = vec![0u8; w * h * 4];
        texture.download(&mut pixels, w * 4); //.download_rgba(&mut pixels, w * 4);

        // 2. Downscale for performance (blurring 4K is slow and unnecessary)
        let (rgba, nw, nh) = Self::downscale_rgba(&pixels, w, h, 1280, 720);

        // 3. Separable Gaussian Blur (Faster O(N) vs O(N^2))
        let sigma = 60.0;
        let kernel = Self::gaussian_kernel_1d(sigma);

        let horizontal_pass = Self::blur_h(&rgba, nw, nh, &kernel);
        let blurred = Self::blur_v(&horizontal_pass, nw, nh, &kernel);

        // 4. Blend based on vertical gradient
        let final_pixels = Self::apply_gradient_blend(&rgba, &blurred, nw, nh);

        // 5. Convert back to texture
        let bytes = glib::Bytes::from(&final_pixels);
        let tex = gdk::MemoryTexture::new(
            nw as i32,
            nh as i32,
            gdk::MemoryFormat::B8g8r8a8,
            &bytes,
            nw * 4,
        );

        self.picture.set_paintable(Some(&tex));
    }

    // --- Math: Separable Blur ---

    fn gaussian_kernel_1d(sigma: f32) -> Vec<f32> {
        let radius = (sigma * 3.0).ceil() as usize;
        let size = radius * 2 + 1;
        let mut kernel = vec![0.0; size];
        let mut sum = 0.0;
        let two_sigma_sq = 2.0 * sigma * sigma;

        for i in 0..size {
            let x = i as f32 - radius as f32;
            let val = (-(x * x) / two_sigma_sq).exp();
            kernel[i] = val;
            sum += val;
        }
        kernel.iter_mut().for_each(|v| *v /= sum);
        kernel
    }

    fn blur_h(input: &[u8], w: usize, h: usize, kernel: &[f32]) -> Vec<u8> {
        let mut out = vec![0u8; input.len()];
        let radius = kernel.len() / 2;

        out.par_chunks_exact_mut(w * 4)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..w {
                    let mut acc = [0.0f32; 4];
                    for (i, &weight) in kernel.iter().enumerate() {
                        let ix = (x as isize + i as isize - radius as isize)
                            .clamp(0, w as isize - 1) as usize;
                        let idx = (y * w + ix) * 4;
                        for c in 0..4 {
                            acc[c] += input[idx + c] as f32 * weight;
                        }
                    }
                    let off = x * 4;
                    for c in 0..4 {
                        row[off + c] = acc[c] as u8;
                    }
                }
            });
        out
    }

    fn blur_v(input: &[u8], w: usize, h: usize, kernel: &[f32]) -> Vec<u8> {
        let mut out = vec![0u8; input.len()];
        let radius = kernel.len() / 2;
        out.par_chunks_exact_mut(w * 4)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..w {
                    let mut acc = [0.0f32; 4];
                    for (i, &weight) in kernel.iter().enumerate() {
                        let iy = (y as isize + i as isize - radius as isize)
                            .clamp(0, h as isize - 1) as usize;
                        let idx = (iy * w + x) * 4;
                        for c in 0..4 {
                            acc[c] += input[idx + c] as f32 * weight;
                        }
                    }
                    let off = x * 4;
                    for c in 0..4 {
                        row[off + c] = acc[c] as u8;
                    }
                }
            });

        out
    }

    fn apply_gradient_blend(orig: &[u8], blurred: &[u8], w: usize, h: usize) -> Vec<u8> {
        let mut out = vec![0u8; orig.len()];
        out.par_chunks_exact_mut(w * 4)
            .enumerate()
            .for_each(|(y, row)| {
                let t = (y as f32 / h as f32).powf(2.0); // Curve for smoother transition
                let blend = if t < 0.3 {
                    0.0
                } else {
                    ((t - 0.3) / 0.7).clamp(0.0, 1.0)
                };

                for x in 0..w {
                    let i = x * 4;
                    let global_i = (y * w + x) * 4;
                    for c in 0..4 {
                        let val = (orig[global_i + c] as f32 * (1.0 - blend))
                            + (blurred[global_i + c] as f32 * blend);
                        row[i + c] = val as u8;
                    }
                }
            });
        out
    }

    fn downscale_rgba(
        input: &[u8],
        w: usize,
        h: usize,
        max_w: usize,
        max_h: usize,
    ) -> (Vec<u8>, usize, usize) {
        let scale = (w as f32 / max_w as f32)
            .max(h as f32 / max_h as f32)
            .max(1.0);
        let nw = (w as f32 / scale) as usize;
        let nh = (h as f32 / scale) as usize;
        let mut out = vec![0u8; nw * nh * 4];
        for y in 0..nh {
            for x in 0..nw {
                let si = ((y * h / nh) * w + (x * w / nw)) * 4;
                let di = (y * nw + x) * 4;
                out[di..di + 4].copy_from_slice(&input[si..si + 4]);
            }
        }
        (out, nw, nh)
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
