
struct StructWithRefField { 
    a_ref: &str 
}

struct LifeTimeRequired<'a> {
    src : &'a str
}

impl <'a> LifeTimeRequired<'a> {

    fn new(src:&'a str) -> Self {
        Self { src }
    }

    fn print(&self) {
        println!("{}", self.src);
    }
}

fn main() { 
    let s = StructWithRefField { a_ref: "23"};
    
}