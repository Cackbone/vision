use std::error::Error;
use std::collections::HashMap;

use image::{io::Reader, DynamicImage, Pixel, Rgba};

#[derive(Clone)]
pub struct VisionImage(DynamicImage);

impl VisionImage {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let dyn_image = Reader::open(path)?.with_guessed_format()?.decode()?;

        Ok(VisionImage(dyn_image))
    }


    pub fn get_main_colors(&self, colors_nb: usize) -> Vec<Rgba<u8>> {
        let rgba = self.0.to_rgba();
        let mut counter: HashMap<Rgba<u8>, usize> = HashMap::new();

        for pixel in rgba.pixels() {
            *counter.entry(*pixel).or_insert(0) += 1;
        }

        let mut counter_sorted: Vec<(&Rgba<u8>, &usize)> = counter.iter().collect();
        counter_sorted.sort_by(|a, b| b.1.cmp(a.1));

        counter_sorted.iter().take(colors_nb).map(|x| *x.0).collect()
    }
}

pub struct Vision {
    images: Vec<VisionImage>,
}

impl Vision {
    pub fn new() -> Self {
        Vision {
            images: Vec::new()
        }
    }

    pub fn load_one(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let image = VisionImage::load(path)?;

        self.add(image);
        Ok(())
    }

    pub fn load(&mut self, paths: Vec<&str>) -> Result<(), Box<dyn Error>> {
        for path in paths {
            self.load_one(path)?
        }
        Ok(())
    }

    pub fn add(&mut self, image: VisionImage) {
        self.images.push(image)
    }

    pub fn get_main_colors(&self, colors_nb: usize) -> Vec<Rgba<u8>> {
        let mut counter: HashMap<Rgba<u8>, usize> = HashMap::new();

        for image in self.images.iter() {
            let rgba = image.0.to_rgba();

            for pixel in rgba.pixels() {
                *counter.entry(*pixel).or_insert(0) += 1;
            }
        }

        let mut counter_sorted: Vec<(&Rgba<u8>, &usize)> = counter.iter().collect();
        counter_sorted.sort_by(|a, b| b.1.cmp(a.1));

        counter_sorted.iter().take(colors_nb).map(|x| *x.0).collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
