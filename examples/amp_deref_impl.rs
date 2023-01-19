use std::ops::Deref;

// Generic tuple struct
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello_takes_amp_str(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let x = 5;
    let y = MyBox::new(9);

    assert_eq!(5, x);
    assert_eq!(9, *y);
    // Since Deref is implemented for MyBox, here *y will be re-written as
    // *(y.deref())
    // Not that in the replacement * means plain deference

    // Deref Coercion
    let m = MyBox::new(String::from("Rust"));
    hello_takes_amp_str(&m);
    // here we have passed &m which is of type &MyBox<String>
    // &m is a reference to MyBox<String>
    // the expected type for hello_takes_amp_str is &str
    // If we had no Deref coercion we had to call like this
    // hello(&(*m)[..]);
    // *m -> String
    // & [..] -> full slice ie &str
    // Since we have Deref implementation for MyBox<T>
    // compiler doe the following
    // m.deref() -->  s : &String
    // Since String implements deref
    // s.deref() --> k: &str
}
