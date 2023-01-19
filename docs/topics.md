## Links

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
