use std::error::Error;

use image::{io::Reader, DynamicImage};

pub struct Vision {
    images: Vec<DynamicImage>,
}

impl Vision {
    pub fn new() -> Self {
        Vision {
            images: Vec::new()
        }
    }

    pub fn load(&mut self, file: &str) -> Result<(), Box<dyn Error>> {
        let image = Reader::open(file)?.with_guessed_format()?.decode()?;

        self.images.push(image);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
