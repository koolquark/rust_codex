use std::pin::Pin;

fn main() {
    let mut greeting = String::from("Hello Rust!");
    greeting.push_str(" Hello World!");

    let mut mutable_ref = &greeting;
    let pinned = Pin::new(mutable_ref);

    greeting.push_str(" Can't push now");

    println!("{}", greeting);
}
