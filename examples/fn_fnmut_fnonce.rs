// Fn, FnMut and FnOnce are traits
// - they are implemented by types that can be invoked like functions.
// - corresponds to three kinds of methods that can be invoked on an instance.
// - or called as call operator
// - these traits implement a call function (extern "rust-call")
// - Fn
//   - call by reference
//   - this version of call operator takes an immutable receiver
//   - thus instances of Fn can be called repeatedly (on the receiver) without mutating state
// - FnMut
//   - call by mutable reference
//   - this version of call operator takes an mutable receiver
// - FnOnce
//   - call by value
//   - invoked on a value
//   - value gets consumed

// Note : Function Pointers (fn) and Fn,FnMut,FnOnce are different
// Fn,FnMut,FnOnce are traits.
// Trait means shared behaviour .. (extracted)

// For any type that implements Fn, &Fn implements Fn too
// FnOnce and FnMut are super traits of Fn
// Fn can be used as a parameter where FnMut or FnOnce is expected

// @TODO
fn main() {}
