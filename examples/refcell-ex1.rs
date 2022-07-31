use std::{cell::{RefCell, RefMut}}; 

fn main() {

    // refcell gives us interior mutability 
    // borrow checks at runtime ; may cause panics at runtime 

    // constructing 
    let ref_cell = RefCell::new("Hello Rust".to_owned());

     
    // ref_cell var is moved ; can't be used further 
    let hello_rust = ref_cell.into_inner();

    println!("ref_cell_inner = {}", hello_rust);

    // another cell
    let rcell_2 = RefCell::new("Cell2".to_owned()); 


    { 
        // scope isolation ; starting a block 

        // take a mutable borrow 
        // see the type RefMut
        let mut mut_borrow = rcell_2.borrow_mut();

        // only one immutable borrow in safe code
        *mut_borrow = "Its Cell2".to_owned();
    }

    // see the type Ref 
    let b = rcell_2.borrow();
    let _ = b ;

    // Can't do this, since we have other borrows in scope 
    // Will compile ; but runtime PANIC
    // let mutable_borrow = rcell_2.borrow_mut();

    // can have many immutable borrow in safe code
    println!("rcell_2 value = {}", rcell_2.borrow());
    println!("rcell_2 value = {}", rcell_2.borrow());

    
    let cplx = RefCell::new((5, 2));
    { 
        // get a mutable borrow 
        let cplx_mb = cplx.borrow_mut();

        // we get reference (mutable) to component of cplx 
        // managed via the supplied fn
        let real = RefMut::map(
            cplx_mb, |t| &mut t.0
        );
        println!("{}", real);

    }

    

    let point = RefCell::new((3,4));

    {
        // map and split to component refmuts 
        let (x, y ) = RefMut::map_split(

            point.borrow_mut(),
            |t|  (&mut t.0, &mut t.1)
        );

        println!("{:?} , {:?}", x, y);
    }

    


}