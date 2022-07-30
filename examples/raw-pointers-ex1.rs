
use std::ptr ; 

#[derive(Debug)]
struct Ex {
    a: u8,
    b: String
}

// Raw Pointers example 1 
fn main() {

    let ex = Ex { a: 12, b: "Rust".to_string()};
    let mut ex_mut = Ex { a: 13, b: "Rusty".to_string()};

    let rp = ptr::addr_of!(ex); 
    let rp_mut =  ptr::addr_of_mut!(ex_mut);

    println!("{:?},{:?}", rp, rp_mut);

    // deferencing using raw pointers is an unsafe operation
    unsafe {
        println!("{:?} == {:?}", *rp, ex);
    }
    unsafe {
       (*rp_mut).a = 100;
       (*rp_mut).b = "Hello Rust".to_string();
       println!("{:?} == {:?}", ex_mut, *rp_mut);
    }

    
}