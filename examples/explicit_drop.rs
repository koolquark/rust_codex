fn main() {
    let v = vec![1, 2, 3];

    // calls Drop implementation of the type
    // has no effect on types implementing Copy
    drop(v);
}
