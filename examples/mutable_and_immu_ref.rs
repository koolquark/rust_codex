fn main() {
    // In Rust a value can be immutable or mutable

    // The following let statement creates an immutable value
    let _immutable_value = 10;

    // The following let statement creates a mutable value
    // Note the mut modifier
    let mut _mutable_value = 20;

    // can mutate a mutable value
    _mutable_value = 30;

    // I can't mutate an immutable value
    // Following is not allowed
    // _immutable_value = 40;

    // In Rust we can have references to values.
    // There are two kinds of references
    // Mutable Reference and Immutable Reference

    // Mutable reference to a value means
    // 1. The value which this mutable reference references is a mutable value.
    //    Or in other words, we can have a mutable reference only to a mutable value.
    // 2. This reference can be used to mutate the value
    // See the &mut on RHS
    let mutable_ref = &mut _mutable_value;
    *mutable_ref = 100;

    // the let statement LHS has no mut as in
    // let mut somevar = 10;
    // so mutable reference means
    //   reference to a mutable value
    // and not the mutability of the reference variable itself.
    // ie
    // let mut val = 10;
    // let mr = &mut val;
    // is to be read as mr is a reference to mutable value val.
    // and not to assume that the variable mr itself is mutable
    // mutable here means the mutability of the value.

    // For dereferencing the value ; ie to tell that we want to access or assign the value
    // we have used *

    // The following is not possible
    // Borrow checker will complain cannot borrow as mutable.
    // let this_is_not_ok = &mut _immutable_value;

    // an immutable value
    let ival_a = 10;
    // reference to an immutable value
    let _iref_a = &ival_a;

    // immutable value
    let ival_b = 20;
    let ival_c = 40;
    // a mutable variable holding reference to an immutable value
    let mut _iref_b = &ival_b;
    // mutable variable holding reference to an immutable value being mutated ie made to refer another
    // immutable value
    _iref_b = &ival_c;
}
