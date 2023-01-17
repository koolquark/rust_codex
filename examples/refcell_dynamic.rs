// see https://doc.rust-lang.org/core/cell/index.html

// Rust memory safety core tennets
// We can have only following references
// - Have several immutable references ie &T to the object (Shared Reference/ Aliasing)
// - Or (ie one of )
// - Have one mutable reference (&mut T) to the object (Mutable Reference/Unique reference)
// - Can't have same in scope

// How is this enforced  ? By the compiler
// What if we need multiple  references to an object and yet mutate it ?
// Shared mutable containers exist in rust for controlled mutability
// even when aliasing is present.

// Cell, RefCell for controlled interior mutability of shared objects
// Comes with some hazards

use std::cell::Cell;

struct CachedLastRead {
    // imagine that this is a large store of something ; get is costly
    // we implememnt a last_read cache ; which allows us to take the last read value
    // However logically we want a CachedLastRead instance to be immutable after construction
    large_store: Vec<i32>,
    last_read: Cell<i32>,
}

impl CachedLastRead {
    fn last_read(&self) -> i32 {
        self.last_read.get()
    }
    fn get(&self, index: usize) -> i32 {
        let value = self.large_store.get(index).unwrap();
        self.last_read.set(*value);
        *value
    }
}
fn main() {
    // not declared as mutable
    let cached = CachedLastRead {
        large_store: vec![1, 2, 3, 4],
        last_read: Cell::new(0),
    };

    // .get under the hood mutates last_read field
    println!("{}", cached.get(2));
    println!("{}", cached.get(3));
    println!("{}", cached.last_read());
}
