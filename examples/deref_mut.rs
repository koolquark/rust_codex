use std::ops::Deref;

struct DerefExample<T> {
    value: T,
}

impl<T> Deref for DerefExample<T> {
    // Target denotes the resulting type after dereferencing
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    // Deref and DerefMut are traits
    // Rules regarding Deref and DerefMut are designed specifically to
    // accomodate smart pointers.
    // Deref should be implemented only for smart pointer types
    // Deref and DerefMut may get invoked implicily by the compiler.

    // Deref
    // for immutable dereferencing operations ie *v
    // for use on a container (mostly smart pointer types)

    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);
    // TODO How is this different from x.value ?

    // DerefMut
    // for mutable deferencing operations like *v = 1
    // for use on a container (mostly smart pointer types)
}
