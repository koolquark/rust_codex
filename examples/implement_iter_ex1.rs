use std::iter::IntoIterator;

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8;

    type IntoIter = PixelIterator;

    fn into_iter(self) -> Self::IntoIter {
        PixelIterator {
            pixel: self,
            index: 0,
        }
    }
}

pub struct PixelIterator {
    pixel: Pixel,
    index: usize,
}

impl Iterator for PixelIterator {
    type Item = i8;
    fn next(&mut self) -> Option<i8> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}
fn main() {
    let p = Pixel {
        r: 10,
        g: 11,
        b: 16,
    };
    for c in p {
        println!("{}", c);
    }
}
