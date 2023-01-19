use std::{sync::atomic::AtomicUsize, sync::atomic::Ordering};

trait Tr {
    fn default_impl() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "default impl : counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }

    fn types_impl();
}

impl Tr for Ty1 {
    fn types_impl() {
        // this is a generic scope
        // so there is only one COUNTER for all types that implement this function
        // the COUNTER here and COUNTER in default_impl are different static instances
        // ( the scopes are different although both are generic)
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "types impl : counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }
}
impl Tr for Ty2 {
    fn types_impl() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "types impl : counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }
}
struct Ty1 {}
struct Ty2 {}

impl Tr for String {
    fn types_impl() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        println!(
            "types impl : counter was {}",
            COUNTER.fetch_add(1, Ordering::Relaxed)
        );
    }
}

fn main() {
    <Ty1 as Tr>::default_impl(); // 0
    <Ty1 as Tr>::default_impl(); // 1
    <Ty1 as Tr>::default_impl(); // 2
    <Ty2 as Tr>::default_impl(); // 3

    <Ty1 as Tr>::types_impl(); //0
    <Ty1 as Tr>::types_impl(); //1
    <Ty1 as Tr>::types_impl(); //2

    <Ty2 as Tr>::types_impl(); //0
}
