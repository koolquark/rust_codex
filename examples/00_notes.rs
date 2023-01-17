fn _what_does_amp_star_mean_in_rust() {
    // * triggers an explicit dereference
    // if the receiver implements ops::Deref overloading,
    // that overloaded behav happens

    // the type of s here is String
    let s = "hi".to_string();

    // here the type of a is &String
    let _a = &s;

    // now lets consider
    let s = "hi".to_string();
    let _b = &*s;
    // here &*s means &(*s)
    // to understand what happens, we have to see if s ie String
    // overloads * or not. * means deref ie
    // std::ops::Deref::deref(&s)
    // Infact string overloads *
    //   impl Deref for String {
    //     type Target = str;
    //     fn deref(&self) -> &str { ... }
    // }
    // So the type of b is &str
}

//----
fn _pass_ref_or_val(s: String) {
    println!("{}", s);
}

fn _passing_ref_or_val() {
    let s1: String = "hello".to_string();
    let _s2: String = "world".to_string();

    _pass_ref_or_val(s1);
    // won't take ref
    // pass_ref_or_val(&s2);
}
// ------

fn _fn_takes_ref(v: &i32) {
    println!("{}", v);
}

fn _pass_ref_to_fn() {
    let v = 300;
    let p = &v;
    _fn_takes_ref(p);
}

//----

fn _clone_mut() {
    let mut _val = "Hello".to_string();
    let mut _cl_1 = _val.clone();
    _cl_1 = " World".to_string();
    println!("{} {}", _val, _cl_1);

    //
    let _amp_str = "its amp str, and not str";
    let _val = "new".to_string();
    // the following is not ok
    // let _dref_ed  = *val;  // size not known
    let _dref_ed = &*_val;
}

fn main() {
    _clone_mut();
}
