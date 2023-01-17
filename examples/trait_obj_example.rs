trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

// dyn indicates a trait object
// since trait object is a DST, needs to wrap in Box
fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
    // by specifying  _ we rely on type  inference
    print(Box::new(20) as _);
    // since inference works
    print(Box::new(23));
}
