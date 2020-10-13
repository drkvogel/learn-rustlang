
// Reference pointers - point to a resource in memory

pub fn run() {
  // primitive array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2)); // Values: ([1, 2, 3], [1, 2, 3])

  // with non-primitives, if you assign another variable to a piece of data, the first
  // variable will no longer hold that value. You'll need to use a reference (&) to
  // point to the resource

  // Vector
  // let vec1 = vec![1, 2, 3]; // error[E0382]: use of moved value: `vec1`
  //     ---- move occurs because `vec1` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
  // let vec2 = vec1;
  //     ---- value moved here

  let vec1 = vec![1, 2, 3]; // 
  let vec2 = &vec1;
          // ----- borrow of `vec1` occurs here
  // println!("Values: {:?}", (vec1, vec2)); // error[E0505]: cannot move out of `vec1` because it is borrowed
  //                           ^^^^  ---- borrow later used here
  //                           |
  //                           move out of `vec1` occurs here

  println!("Values: {:?}", (&vec1, vec2)); // Values: ([1, 2, 3], [1, 2, 3])
}