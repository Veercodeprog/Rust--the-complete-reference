// ownership in rust is a system that helps manage memory and prevent memory leaks.
// these rules are enforced at compile time.
// owner: the owner of a value is the variable or data structure  that holds it and is responsible
// for allocating and freeing the memory used to store the value.
// ownership rules:
// 1. each value in rust has a variable that is its owner.
// 2. there can only be one owner at a time.
// 3. when the owner goes out of scope, the value will be dropped.
//
// ownership prevents issues such as :-
// 1.) prevents memory safety issues such as:-
//    a.) dangling pointers.
//    b.) double free errors : trying to free memory that has already been freed.
//    c.) memory leaks: not freeing memory that is no longer being used.
//
//two types of regions in Ram used by a program at runtime:stack and heap memory.
//
//Stack memory:
// 1. stack memory is used for static memory allocation.
// 2. Last in first out.
// 3. all data stored on the stack must have a known, fixed size like integers, floats, bool,char
//    etc.
//4.pushing to the stack is faster than heap because the stack is just a pointer i.e. new data is
//  always at the top of the stack.
//  5. types of the unknown size will get allocated to the heap and a pointer to the heap value
//     will be stored on the stack or is pushed to the stack because the size of the pointer is
//     known and fixed.
//

// in the program below, the variable x,y,z are stored on the stack.and then the function
// add_numbers is called and the values of x and y are passed to the function and the result is
// stored in z. the function add_numbers is also stored on the stack with variables a,b,c.
fn main() {
    let x = 5;
    let y = 10;
    let z = add_numbers(x, y);
    println!("The result is {}", z);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let c = a + b;
    c
}

//Heap memory:
// 1. heap memory is used for dynamic memory allocation.
// 2.data with an unknown size at compile time or a size that might change must be stored on the
//  heap.
//  3. allocating data on the heap will return a pointer to the data( a location in memory where
//   the data is stored or allocated).
//   4. accessing data on the heap is slower than accessing data on the stack because you have to
//   follow a pointer to get to the data.
//   5. accessing data on the heap is slower than accessing data on the stack because you have to
//   follow a pointer to get to the data.
//
//   String type:
//   1. the string type is stored on the heap as it is mutable and can grow in size.
//   2. string is stores on the stack and the data it points to is stored on the heap.
//
//   let s1 = String::from("hello");
//   s1 is a string literal and is stored on the stack and it is actually a pointer to the heap
//   where the actual data is stored.
//s1 in stored on the stack and has the following data:
// 1. a pointer to the heap where the actual data is stored.
// 2. the length of the string.
// 3. the capacity of the string.
//
//the size of s1 is fixed on the stack but the size of the data it points to on the heap can
//change. the size os s1 on the stack is based on three values: the pointer to the heap, the length
//of the string and the capacity of the string.
//so its size is 3 times the size of a pointer i.e 24 bytes on a 64-bit system.(3*8=24)
//
//copy vs move:
// 1. types that are stored on the stack implement the copy trait.
// 2, copying is cheap and does not require a deep copy of the data.
// 3. types that are stored on the heap implement the move trait as copying the data would be
// expensive.
// e.g.
// let x = 5;
// let y = x;
// integer value of x is copied to y and both x and y are stored on the stack.
//
// let s1 = String::from("hello");
// let s2 = s1;
// as s1 is just a pointer to data on the heap, when s2 is assigned the value of s1, the pointer
// to the heap is copied to s2 and s1 is invalidated. this is called a move and not the whole data
// is copied to s2 or in the heap.
// the s1 is invalidated on moving the pointer to s2 as rust does not allow two pointers to the
// same data on the heap as it can lead to data corruption.that is ownership rules.
//
//Deep copy:
// 1. to create a deep copy of data on the heap, the clone method is used.
// let s1 = String::from("hello");
// let s2 = s1.clone();
// now s1 and s2 are two different pointers to the same data on the heap.
//
//
