use std::rc::Rc; 

fn main() {

    // Rc<String>
    let an_rc = Rc::new("Hello Rust".to_owned());

    // able to call String Function directly on Rc 
    // since Rc implements Deref for T = String 
    // Rc does not implement DerefMut
    let as_str = an_rc.as_str();

    println!("{:?}", as_str);

    // cloning an Rc using fully qualified syntax  
    // creates new reference to existing allocation
    // this does not involve heap allocation
    let an_rc_clone = Rc::clone(&an_rc);

    println!("{:?}", an_rc_clone);
    

}