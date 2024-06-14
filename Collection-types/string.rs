// std::string::String is UTf-8 encoded ,growable string.it is a most common string type we use in
// daily programming.
//
// 1.🌟🌟
//
//
// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
fn main() {
    let mut s: String = String::from("hello, )";
    s.push_str("world");
    s.push("!");

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

2.🌟🌟
String and &str
A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence.
String is heap allocated, growable and not null terminated.

&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a
String, just like &[T] is a view into Vec<T>.

// FILL in the blanks
fn main() {
   let mut s: String = String::from("hello, world");

   let slice1: &str = &s; // In two ways  second way is to write let slice1: &str = s.as_str();
   assert_eq!(slice1, "hello, world");

   let slice2: &str = &s[..5];
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; //or let mut slice3: String =  s; as we are not using s after
//this line.so we can move it and give ownership to slice3.
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!");
}

3.🌟🌟

// Question: how many heap allocations are happening here?
// Your answer:
fn main() {
    // Create a String type based on `&str`
    // The type of string literals is `&str`
   let s: String = String::from("hello, world!");//allocating in heap memory ,1 times

   // Create a slice point to String `s`
   let slice: &str = &s;// just a view into the data,no allocation

   // Create a String type based on the recently created slice
   let s: String = slice.to_string(); //allocating in heap memory, 2 times

   assert_eq!(s, "hello, world!");

   println!("Success!");
}

4.🌟🌟🌟 You can't use index to access a char in a string, but you can use slice &s1[start..end].

// FILL in the blank and FIX errors
fn main() {
    let s = String::from("hello, 世界");
    let slice1: &str = &s[..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2:&str  = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format  so 7..10, 10..13 is
// for the character '界'
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!");
}

// we can use utf8_slice to slice UTF-8 string , it can index chars instead of bytes
// use utf8_slice;
fn main() {
   let s = "The 🚀 goes to the 🌑!";

   let rocket = utf8_slice::slice(s, 4, 5); //if we dont use utf8_slice the 4..7
   // Will equal "🚀"
}

5.🌟🌟🌟

// FILL in the blanks
fn main() {
    let mut s:String = String::new();
    s.push_str("hello");

    // Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111]; //vector holding u8 integers

    // Turn a byte's vector into a String
    let s1:String = String::from_utf8(v).unwrap(); //converting vector to string;


    assert_eq!(s, s1);

    println!("Success!");
}


Representation
A String is made up of three components: a pointer to some bytes, a length, and a capacity.

The pointer points to an internal buffer String uses to store its data. The length is the number of
bytes currently stored in the buffer( always stored on the heap ), and the capacity is the size of
the buffer in bytes or total amount of memory recieved from allocator. As such, the length will always be less than or equal to the capacity.

6.🌟🌟🌟
🌟🌟 If a String has enough capacity, adding elements to it will not re-allocate


// Modify the code below to print out:
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
fn main() {
    let mut s:String = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");//we only need 5 capacity but rust provides 8 capacity by default. also
        //capacity increases by 2 times when it is full and new memory is allocated.this can be
        //expensive . so to resolve this we can use with_capacity method to allocate memory
        println!("{}", s.capacity()); // if lenght > capacity then rust will have to reallocate the
        // memory for string i.e. find a new memory location and copy the data to the new location.
    }

    println!("Success!");
}
