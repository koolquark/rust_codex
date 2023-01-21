## Links

- RFC Book
  - https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html
- RustC Book
  - https://rustc-dev-guide.rust-lang.org/traits/hrtb.html
- Rust By Example Book

### Readings - Blogs, Videos, SO etc

- Implementing Iterator for a struct type
- https://stackoverflow.com/questions/30218886/how-to-implement-iterator-and-intoiterator-for-a-simple-struct
-

## Techniques for Learning and Understanding Rust Code

- If the following is not important for you, don't learn Rust
  - Need a safe and robust implementation of an application
  - Need reasonable performance along with the above one.
- doc.rs can show source code. Always use this feature to see
  source code
- doc.rs can show many versions of same lib, use right versions
- Use code navigation provided by editor
- Always understand who is the owner of a value, how ownership is
  getting transferred.
- Always look into the lifetimes involved.
- Read code extensively and understand the patterns used by professional developers
- Read Rust RFCs to understand design choices.

## Topics

- extern crate
- const
- pointer types in rust ; dereferencing
- whats ffi
- macros : procedural, function like and derive
- proc_macro crate
- macro hygiene
- raw identifiers
- Non Doc comments : line and doc
- Doc comments : line and doc
- tuples
- byte string vs string
- Value and Ownership
- compile rust code to assembly
- slice type
- Use of mod.rs
  - https://stackoverflow.com/questions/26435102/in-rust-what-is-the-purpose-of-a-mod-rs-file
- cargo modules tree generator
- static item, reference to static item vs static lifetime
- constant vs static item
- lazy_static
- static items and drop
- constant expression
- mutable static items
- Rules related to static
- external block
- dangling refrence
- behaviour of static items in generics
- Ownership, Borrow and transfer of ownership
- Understand the difference between generic parameter of impl
  and the item for which impl is implemented
- The newtype pattern for implementing external traits on external types
- Supertrait
  - a trait has to implement all methods of its supertrait
- Tuple Struct
- Autoref
- Deref
- AsRef
- Deref Coercion
- Referenced and Pointers

### References and Pointers

- https://stackoverflow.csom/questions/62232753/what-are-the-differences-between-a-pointer-and-a-reference-in-rust
- Don't see & and \* the C or C++ way.
  - Meanings are different here s
- Types of references and pointerss
  - Shared reference : &T
  - Mutable Reference: &mut T
  - Const Pointer : \*const
  - Mutable Pointer : \*mut

### Deref

- When Deref is implemented for a type SomeBox, and if b is SomeBox then
  `*b` will be re-written as `*(b.deref())`. In the re-written
  item , `*` means plain derefence.

### Deref Coercion

- Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
- This conversion applies many times ( w/o runtime cost ) to reach the final type.
- Supported
  - From &T to &U when T: Deref<Target=U>
  - From &mut T to &mut U when T: DerefMut<Target=U>
  - From &mut T to &U when T: Deref<Target=U>

### Reading Lifetime annotations

- For this example

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

- Lifetime annotations in function signatures infact declare a constraint relation between lifetimes of
  the involved references.
  Here the constraint is,
  ```
  for some lifetime 'a the function takes two parameters both of which are string slices that
  live at least as long enough as lifetime  'a. Also, the string slice returned from the function
  will live at least as long as lifetime a'
  ```
- The lifetimes specified are like generics and are part of function signature itself. When function is
  called the generic lifetimes gets concrete values.
  qualiz@@

## Examples

- raw string
  - r#"hello world"#
- byte b'H'
- byte string b"Hello"
- raw byte string br#"hello"#
-
