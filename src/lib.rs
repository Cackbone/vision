use std::error::Error;

use image::{io::Reader, DynamicImage};

#[derive(Clone)]
pub struct VisionImage(DynamicImage);

impl VisionImage {
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let dyn_image = Reader::open(path)?.with_guessed_format()?.decode()?;

        Ok(VisionImage(dyn_image))
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

    pub fn load(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let image = VisionImage::load(path)?;

        self.add(image);
        Ok(())
    }

    pub fn add(&mut self, image: VisionImage) {
        self.images.push(image)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
