// See https://doc.rust-lang.org/nomicon/lifetimes.html
fn main() {
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    // data.push(4);
    println!("{}", x);
}
// The ownership and scope are the factors that decide lifetime
// The lifetime of a reference can't be larger than the life
// time of the item it refers to.
// Here we can see that the var data is the owner of the value.
// We have taken a shared reference x to a subpart of the value
// owned by data. On next line we are trying to push to the value
// owned by data, this is not possible since a subpart is already borrowed
// as shared reference, we can't have a mutable reference anymore
// Also if we consider lifetimes, lifetime is decided by owner's
// scope , here the owner is data and is out of scope at the line
// data.push ( data is mutable , push takes onwnership of value
// under data's ownerhsip ). Thus x's lifetime has ended before
// the println statement

// See Rustnomicon for lifetime information

// 'a: {
//   let mut data: Vec<i32> = vec![1, 2, 3];
//   'b: {
//       // 'b is as big as we need this borrow to be
//       // (just need to get to `println!`)
//       let x: &'b i32 = Index::index::<'b>(&'b data, 0);
//       'c: {
//           // Temporary scope because we don't need the
//           // &mut to last any longer.
//           Vec::push(&'c mut data, 4);
//       }
//       println!("{}", x);
//   }
// }
