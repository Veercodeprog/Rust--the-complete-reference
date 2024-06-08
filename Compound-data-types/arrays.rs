// 1. fixed size collection of elements of same data type stored as
// coniguous block in stack memory.
// 2. signature of array is [T; Length] where T is the type of elements.this indicates the length
// of the array is fixed and cannot be changed at compile time.
// 3.arrays can neither grow nor shrink in size,they must retain their original size.
//
//4. we cant initialize an array like below:
//
//fn init_array(n: i32) {
//let arr = [1; n]; // this will not work as the length of the array must be known at compile time.
//println!("{:?}", arr);
//}
//this wont work as the length of the array must be known at compile time.if we do it like this
//the length of the array will be known at runtime and the compiler will not be able to allocate
//the memory for the array.
//
//out of bounds indexing causes program to panic in RUST
