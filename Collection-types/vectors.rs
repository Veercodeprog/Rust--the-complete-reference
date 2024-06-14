//Vectors:
//Like arrays but dynamically sized
//allocation on the heap as contiguous memory
//all elements must be of the same type
//special macro: vec!.
//
//eg: String type is just a vector of UTF-8 bytes that is allocated in heap memory
//String = vec<u8>
//
//vectors are resizable arrays.Like slices there size is not known at compile time , but they can
//grow or shrink in size.
//
//1. 🌟🌟
//
fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v: Vec<u8> = Vec::from(arr);
    is_vec(v); // v is Vec<u8>

    let v: Vec<u8> = vec![1, 2, 3]; // shadowing
    is_vec(v);

    // vec!(..) and vec![..] are same macros, so
    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(v.clone());

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    // let v1: Vec<u8; 3> = vec!(arr);
    let mut v1: Vec<u8> = Vec::new(); // create new empty vector
    is_vec(v1.clone());
    for i in &v {
        v1.push(*i);
    }
    assert_eq!(v, v1);

    println!("Success!");
}

fn is_vec(v: &Vec<u8>) {}

2. 🌟🌟
A vec can be extended with extend method


// FILL in the blank
fn main() {
    let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
    v1.pop(); // remove last element
    v1.push(3); // add 3 to the end

    let mut v2: Vec<i32> = Vec::new();
    v2.extend(&v1.extend());

    assert_eq!(v1, v2);

    println!("Success!");
}

3. 🌟🌟
Turn x into Vec

// FILL in the blanks
fn main() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr: [i32;3] = [1, 2, 3];
    let v1: Vec <i32> = Vec::from(arr);
    let v2: Vec<i32> = arr.into();

    assert_eq!(v1, v2);


    // String -> Vec
    // impl From<String> for Vec
    let s: String = "hello".to_string();
    let v1: Vec<u8> = s.into();

    let s = "hello".to_string();
    let v2: Vec<u8> = s.into_bytes(); // into_bytes() is a method of String
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s: &str = "hello";
    let v3: Vec<u8> = Vec::from(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    println!("Success!");
 }

4. 🌟🌟

// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i)) //Option<i32>
    }

    for i in 0..5 {

        // IMPLEMENT the code here...
   match v.get(i) {
        Some(e) => v[i] = e + 1,
        None => v.push(i+2)
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}

Slicing:
A Vec can be mutable .On the other hand , slices are read only objects.To get a slice use & operator.
In rust its most common to pass slices as arguments to functions rather than passing the entire vectors when you just want to
provide read access to the data.

5. 🌟🌟
// FIX the errors
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    let slice1: &[i32]= &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3: &[i32] = &v[0..];
   // slice3.push(4);  /// slices are immutable references only , so we can not mutate a slice.

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}

Capcity:
The capacity of a vector is the amount of space allocated for any future elements that will be added
onto the vector. This is not to be confused with the length of a vector, which specifies the number of
actual elements within the vector. If a vector’s length exceeds its capacity, its capacity will automatically
be increased, but its elements will have to be reallocated.

For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10 more elements.
Pushing 10 or fewer elements onto the vector will not change its capacity or cause reallocation to occur.
However, if the vector’s length is increased to 11, it will have to reallocate, which can be slow.
For this reason, it is recommended to use Vec::with_capacity whenever possible to specify how big the
vector is expected to get.

// FIX the errors
fn main() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}


6. 🌟🌟
Store distinct types in Vector
The elements in a vector must be the same type, for example , the code below will cause an error:

fn main() {
   let v = vec![1, 2.0, 3]; // error: mismatched types
}
But we can use enums or trait objects to store distinct types.

#[derive(Debug, PartialEq)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // FILL in the blank
    let v : Vec<IpAddr>=
    vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}

8. 🌟🌟
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    // FILL in the blank
    let v: vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
