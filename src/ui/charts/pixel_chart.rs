use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use eframe::egui::{ColorImage, Image, TextureOptions, Ui, Vec2};

use crate::ui::charts::chart::Chart;

#[derive(Clone)]
pub struct PixelChart {
    pixel_array: Arc<RwLock<VecDeque<u8>>>,

    width : usize,
    height : usize,
}

impl PixelChart {
    pub fn new(width: usize, height:usize) -> PixelChart {
        PixelChart {
            pixel_array: Arc::new(RwLock::new(vec![0; width * height * 3].into())),
            width,
            height,
        }
    }

    pub fn add(&mut self, r:u8, g:u8, b:u8) {
        self.pixel_array.write().unwrap().pop_front();
        self.pixel_array.write().unwrap().pop_front();
        self.pixel_array.write().unwrap().pop_front();

        self.pixel_array.write().unwrap().push_back(r);
        self.pixel_array.write().unwrap().push_back(g);
        self.pixel_array.write().unwrap().push_back(b);

        self.pixel_array.write().unwrap().make_contiguous();
    }
}

impl Chart for PixelChart {
    fn update(&self, ui: &mut Ui) {
        let texture_options = TextureOptions::default();
        let texture_id = ui.ctx().load_texture("test_img", ColorImage::from_rgb([self.width,self.height], self.pixel_array.read().unwrap().as_slices().0),texture_options);

        ui.add(
            Image::new(&texture_id).fit_to_exact_size(Vec2::new(1024.0,1024.0))
        );
    }
}