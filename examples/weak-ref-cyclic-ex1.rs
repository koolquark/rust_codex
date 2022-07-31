use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Gadget {
    // Refering same type via Weak Reference
    // We have a self refering struct
    me: Weak<Gadget>,
}


impl Gadget {
    // constructing 
    // need to hold a self reference using weak ref 
    // to ensure proper deallocation
    fn new() -> Rc<Self> {
        // new_cyclic will call the supplied data_fn first
        // supplying it with a Weak Reference to the allocation
        // When Gadget is constructed we clone and use it
        // The clone is still a weak ref
        // Thus the self reference held is a weak ref
        // The new_cyclic has a signature 
        // pub fn new_cyclic<F>(data_fn: F) -> Rc<T>
        // where
        // F: FnOnce(&Weak<T>) -> T
        // The associated function new_cyclic knows the Rc<T> T from the 
        // supplied data_fn's type signature
        Rc::new_cyclic(|weak_ref_to_gadget| {
            let clone_of_weak_ref = weak_ref_to_gadget.clone();

            Gadget {
                me: clone_of_weak_ref,
            }
        })
    }

    fn me(&self) -> Rc<Self> {
        self.me.upgrade().unwrap()
    }
}

fn main() {
    let self_ref_gadget = Gadget::new();
    println!("{:?}", self_ref_gadget);
    println!("{:?}", self_ref_gadget.me);
    println!("{:?}", self_ref_gadget.me());

}
