use std::{iter::IntoIterator, path::Prefix};

struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}

impl IntoIterator for Pixel {
    type Item = i8;

    type IntoIter = PixelIterator;

    // here self means an instance of Pixel, ie an owned value.
    // Here into_iter takes ownership of the value and hence that instance
    // cannot be used afterwards. The value gets moved into.
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
    // here self means the instance of PixelIteratorRef
    // which holds the state of iteration (ie index)
    // and the state has to be updated on each step
    // , thus &self
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

// Implementing for reference ie &Pixel
impl<'a> IntoIterator for &'a Pixel {
    type Item = i8;
    type IntoIter = PixelIteratorRef<'a>;

    // here self is a shared reference ( aka immutable reference ) to Pixel
    // so the into_iter does not take ownership of the instance of Pixel
    fn into_iter(self) -> Self::IntoIter {
        PixelIteratorRef {
            pixel: self,
            index: 0,
        }
    }
}

pub struct PixelIteratorRef<'a> {
    pixel: &'a Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIteratorRef<'a> {
    type Item = i8;
    // here self means the instance of PixelIteratorRef
    // which holds the state of iteration (ie index)
    // and the state has to be updated on each step
    // , thus &self
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

// Implementing for reference ie &mut Pixel
impl<'a> IntoIterator for &'a mut Pixel {
    type Item = &'a mut i8;
    type IntoIter = PixelIteratorMut<'a>;

    // here self is a mutablereference ( aka unique reference ) to Pixel
    // so the into_iter does not take ownership of the instance of Pixel
    // borrow rules apply
    fn into_iter(self) -> Self::IntoIter {
        PixelIteratorMut {
            pixel: self,
            index: 0,
        }
    }
}

pub struct PixelIteratorMut<'a> {
    pixel: &'a mut Pixel,
    index: usize,
}

impl<'a> Iterator for PixelIteratorMut<'a> {
    type Item = &'a mut i8;
    // here self means the instance of PixelIteratorRef
    // which holds the state of iteration (ie index)
    // and the state has to be updated on each step
    // , thus &self
    fn next(&mut self) -> Option<&mut i8> {
        let result = match self.index {
            0 => &mut self.pixel.r,
            1 => &mut self.pixel.g,
            2 => &mut self.pixel.b,
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
    // borrow of moved value : p value borrowed here after move
    // println!("g = {}", p.g);

    println!("--------");

    let p: Pixel = Pixel {
        r: 20,
        g: 30,
        b: 40,
    };

    let r = &p;

    for c in r {
        println!("{}", c);
    }

    println!("g = {}", p.g);

    println!("--------");

    let mut p: Pixel = Pixel {
        r: 10,
        g: 110,
        b: 30,
    };

    let r = &mut p;

    for mut c in r {
        let new_val = *c - 1;
        println!("current : {} | changing to : {}", c, new_val);
        *c = new_val;
    }

    println!("g = {}", p.g);
}
