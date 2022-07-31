// Example of Weak Pointers 

use std::rc::Rc; 
use std::ptr; 

fn main() {

    // strong is an owning ref to the allocation in the heap
    let strong = Rc::new("Allocation in the Heap".to_owned());

    // Now, we are taking a non-owing ref to the allocation on the heap 
    // using downgrade operation of Rc 
    let weak_aka_non_owning_ref = Rc::downgrade(&strong);

    // Checking if both references point to same allocation on heap 
    assert!(ptr::eq(&*strong, weak_aka_non_owning_ref.as_ptr())); 

    assert_eq!("Allocation in the Heap", 
        unsafe{ &*weak_aka_non_owning_ref.as_ptr()});

    drop(strong);

   

}