use std::ops::Deref;

fn main() {
    let abox = ABox("abox");
    let bbox = BBox(abox);
    let cbox = CBox(bbox);

    // we are passing a reference to CBox to a function that
    // takes &str as the argument
    takes_str(&cbox);
}

fn takes_str(boxname: &str) {
    println!("{}", boxname);
}

struct ABox<T>(T);
struct BBox<T>(T);
struct CBox<T>(T);

impl<T> Deref for ABox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for BBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for CBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
