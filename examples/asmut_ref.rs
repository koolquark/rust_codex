fn main() {
    // AsRef
    let mut container = Box::new(10);
    println!("Box = {}", container);

    let val = container.as_mut();
    *val = 20;

    println!("Box = {}", container);

    // AsMut
    let i_container = Box::new(20);
    println!("iBox = {}", i_container);

    let i_val = i_container.as_ref();
    // i_val = 40; can't do this ; its an immutable reference
    println!("i_val = {}", i_val);
}
