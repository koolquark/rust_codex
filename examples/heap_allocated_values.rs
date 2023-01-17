fn main() {
    // Box<T>
    // - Type for heap allocation
    // - Provides ownership for this allocation to calling scope
    // - Drops the contents when they go out of scope
    // - Can be passed on efficiently since only a pointer needs to be copied
    // - Content lies in heap
    // - Boxed means heap-ed

    let val: u8 = 5; // this is in stack
                     // move to heap using Box

    let boxed: Box<u8> = Box::new(val);
    // move back to stack using dereferencing

    let val: u8 = *boxed;

    // Rust needs to understand how much memory to allocate for each type
}
