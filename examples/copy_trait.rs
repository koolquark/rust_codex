// Copy
// - types those values can be duplicated simply by copying bits
// For variable bindings there are two semantics
// 1. Move Semantics
// 2. Copy Semantics

// Move and Copy semantics comes in play when variable binding happens.

// Foo does not implement Copy
struct Foo;

#[derive(Debug, Copy, Clone)]
struct Bar;

// Bar has Copy implemented
// Clone is a supertrait of Copy

// Implementing w/o derive
struct MyStruct;

impl Copy for MyStruct {}
impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}

fn main() {
    let x = Foo;
    let _y = x;

    // x is moved into y
    // default variable bindings have move semantics
    // now we can't use x

    // if a type implements Copy ; it has copy semantics
    // Bar has Copy Implemented

    let k = Bar;
    let _l = k;

    // k can still be used since k is copied (an not moved ) to l ;
    println!("{:?}", k);
}

// Under the hood both Copy and Move can result in bits being copied.
// The difference is in variable binding semantics, ie the symbol can be
// used for accessing the value.

// Copy vs Clone
// Copy happens implicitly during assignment
// Copy's bhav is not overloadable
// Copy is always simple bit wise copy
// Cloning is explicit, we do it by calling clone as in x.clone()
// Implementation of Clone can provide type specific behaviour
// For some types its not suitable to implement Copy
// For example consider the case of String
// String is basically a pointer and a value pointed to
// If Copy (ie simple bitwise copy) is implemented for String, this would
// mean to do bitwise copy of the pointer, which can subsequently cause double free
// However is type specific Clone (in which we bit copy the pointed to string) is done
// this is avoided
// So String has only Clone

// Clone is a supertrait of Copy
// Everything which is Copy must also implement Clone
// A type can implement Copy if all of its components implement copy
// Shared references are also Copy
// What can implement Copy ?
// Just see if bitwise copy would hurt the Rust semantics related to borrow, mem safety

// Eg
// Shared References &T can be copy
// Shared References &T can be copy even when T is not Copy
// Mutable shared references (&mut T) can't be Copy even when T is Copy
// String can't be copy
// Any type implementing Drop can't  be Copy, because its managing itswon resource
// Function pointer types are copy
// Closure types are Copy if captured (if capturing) values are Copy
// Is it good to implement Copy ?
// - Good and Bad
// - Copy if implemented becomes part of public API of your type
// - Chaning to only Clone later will be a problem ; breaking change
// - Copy brings the convineance for programmer
