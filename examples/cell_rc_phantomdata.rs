// TODO - Explain why phantomdata is used in the Rc defintion

#![allow(unused)]
fn main() {
    use std::cell::Cell;
    use std::marker::PhantomData;
    use std::process::abort;
    use std::ptr::NonNull;

    struct Rc<T: ?Sized> {
        ptr: NonNull<RcBox<T>>,
        // phantom: PhantomData<RcBox<T>>,
    }

    struct RcBox<T: ?Sized> {
        strong: Cell<usize>,
        refcount: Cell<usize>,
        value: T,
    }

    impl<T: ?Sized> Clone for Rc<T> {
        fn clone(&self) -> Rc<T> {
            self.inc_strong();
            Rc {
                ptr: self.ptr,
                // phantom: PhantomData,
            }
        }
    }

    trait RcBoxPtr<T: ?Sized> {
        fn inner(&self) -> &RcBox<T>;

        fn strong(&self) -> usize {
            self.inner().strong.get()
        }

        fn inc_strong(&self) {
            self.inner()
                .strong
                .set(self.strong().checked_add(1).unwrap_or_else(|| abort()));
        }
    }

    impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
        fn inner(&self) -> &RcBox<T> {
            unsafe { self.ptr.as_ref() }
        }
    }
}
