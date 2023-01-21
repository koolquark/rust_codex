use std::mem;

fn main() {
    println!("align_of_i32 : {}", mem::align_of::<i32>());
    println!("align_of_1u8 : {}", mem::align_of_val(&1u8));
    println!("align_of_1u16 : {}", mem::align_of_val(&1u16));

    // comparing enums carrying data disregarding data

    enum HasData {
        A(String),
        B(i32),
    }

    assert_eq!(
        mem::discriminant(&HasData::A("Hello".to_string())),
        mem::discriminant(&HasData::A("World".to_string()))
    );

    assert_eq!(
        mem::discriminant(&HasData::B(20i32)),
        mem::discriminant(&HasData::B(30i32))
    );

    let p = "Hello World";
    mem::drop(p);

    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(vec![1, 2], old_v);
    assert_eq!(vec![3, 4, 5], v);

    struct Buffer<T> {
        buf: Vec<T>,
    }

    impl<T> Buffer<T> {
        fn replace_index(&mut self, i: usize, v: T) -> T {
            mem::replace(&mut self.buf[i], v)
        }
    }

    let mut buffer = Buffer { buf: vec![1, 10] };
    assert_eq!(buffer.replace_index(0, 9), 1);
    assert_eq!(buffer.buf[0], 9);

    let mut x = 5;
    let mut y = 7;
    mem::swap(&mut x, &mut y);

    assert_eq!(7, x);
    assert_eq!(5, y);

    let mut v = vec![1, 2];
    let old_v = mem::take(&mut v); // v gets default value
    assert_eq!(vec![1, 2], old_v);
    assert!(v.is_empty());

    // @TODO transmute example

    let _x: i32 = unsafe { mem::zeroed() };
}
