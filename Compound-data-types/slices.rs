// Slice
//
// reference to contiguous sequence of elements in a collection
// provide a way of borrow part of a collection without taking ownership of the entire collection.
// can be created from array, vector, string, and other collections implementing the Deref trait.
//
// eg of slice syntax:
// let a = [1, 2, 3, 4, 5];
// let slice = &a[1..3]; // slice is &[2, 3]
// assert_eq!(slice, &[2, 3]);
// slice has &[i32] type, which is a reference to a slice of i32 values.
//
// slice actually works like string slices do, by storing a reference to the starting element and a
// length.
//
// slices are similar to arrays, but they are dynamically sized and their size and length are not
// known at compile time.
//
// fn main() {
// let arr: [i32; 5] = [1, 2, 3, 4, 5];
// let s1: &[i32] = &arr[1..3];
// let s2: &str = "Hello, world!"; //a slice reference is a two word object , for simplicity
// reasons, from now on we will use slice  instead of slice reference.the first word is a pointer
// to the first element of the slice, and the second word is the length of the slice.
// println!("Success!");
// }
//
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];
    let slice: &[char] = &arr[..2]; // slice is a reference to the first two elements of the array.
                                    // a slice reference is a two word object , the first word is a pointer to the first element of
                                    // the slice, and the second word is the length of the slice.And pointer and length field both
                                    // occupies sizes of usize which is 8 bytes on 64-bit systems and 4 bytes on 32-bit systems.
                                    // if the slice would have been an array ,then the size of the
                                    // array (slice ) would have have been 8(2*4 i.e. each char of 4 bytes) bytes on 64-bit systems and 4(2*2) bytes on
                                    // 32-bit systems.
    assert!(std::mem::size_of_val(&slice) == 16);
    println!("Success!");
}

// fn main() {
// let arr :[i32; 5] = [1, 2, 3, 4, 5];
// let s1: &[i32] = &arr[1..3];
// assert_eq!(s1, &[2, 3]);
// }
//
// fn main() {
//     let s:&str = "你好，世界";
//     // Modify this line to make the code work
//
// let slice = &s[0..3];
// assert_eq!(slice, "你好");
//println!("Success!");
// }
