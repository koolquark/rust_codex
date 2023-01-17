struct DropOne;

impl Drop for DropOne {
    // takes a mutable reference to self
    // compiler calls this implicitly ;
    // this function cannot be called explicitly Drop::drop(..)
    // If a type implements copy, that same type can't implement Drop
    fn drop(&mut self) {
        println!("Dropping DropOne");
    }
}

fn main() {
    let _d1 = DropOne;
    let _d2 = DropOne;
}
