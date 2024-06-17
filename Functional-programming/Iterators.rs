//Iterators:
//
//Allows to perform tasks on a sequence of items.
//Iterators are lazy , meaning they have no effect until you call methods that consume the iterator
//to use it up.
//All iterators implement the Iterator trait which has a method next() , which gets called
//automatically when traversing over some data.
//Some methods consume iterator while others produce a new iterator from the provided iterator.
//
//The iterator pattern allows us to perform some tasks on a sequence of items in turn. An iterator is responsible
//for the logic of iterating over each item and determining when the sequence has finished.
//
//eg
//fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}",x)
    }
}

By default , for will apply the into_iter() method to the collection provided to make it an iterator.
as a result the following code is equivalent to the above code:

fn main() {
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}",x)
    }
}

1.
/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for i in 0..arr.len() {// we can just write for i in arr
        //
        println!("{}",arr[i]);
    }
}

2.
🌟 One of the easiest ways to create an iterator is to use the range notion: a..b.

/* Fill in the blank */
fn main() {
    let mut v = Vec::new();
    for n in 0..100 {
       v.push(n);
    }

    assert_eq!(v.len(), 100);

next method
All iterators implement a trait named Iterator that is defined in the standard library:

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Methods with default implementations elided
}
And we can call the next method on iterators directly.

3
/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let  mut v1 = vec![1, 2].into_iter(); // into_iter() consumes the vector and returns an iterator not
    // vector itself

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}


into_iter, iter and iter_mut
In the previous section, we have mentioned that for will apply the into_iter to the collection, and change it into a iterator.
However, this is not the only way to convert collections into iterators.

into_iter, iter, iter_mut, all of them can convert a collection into iterator, but in different ways.

into_iter consumes the collection, once the collection has been consumed, it is no longer available
for reuse, because its ownership has been moved within the loop.
iter, this borrows each element of the collection through each iteration, thus leaving the collection
untouched and available for reuse after the loop
iter_mut, this mutably borrows each element of the collection, allowing for the collection to be modified in place.

4.
/* Make it work */
fn main() {
    let arr: Vec<i32> = vec![0; 10];
    for i in arr,iter() {
        println!("{}", i);
    }

    println!("{:?}",arr);
}

5.
/* Fill in the blank */
fn main() {
    let mut namesi: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut(){
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

6.

/* Fill in the blank */
fn main() {
    let mut values: Vec<i32> = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}

Creating our own iterator
We can not only create iterators from collection's types, but also can create iterators by implementing the
Iterator trait on our own types.

Example:

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

7.struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    /* Implement next method */
    fn next(&mut self) -> Option<Self::Item> {
        let forward = self.current + self.next;
        self.current = self.next;
        self.next = forward;
        Some(self.current)
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}

Methods that Consume the Iterator
The Iterator trait has a number of methods with default implementations provided by the standard library.

Consuming adaptors
Some of these methods call the method next to use up the iterator, so they are called consuming adaptors.

8.

/* Fill in the blank and fix the errors */
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total = v1_iter.sum();  // here sum method takes ownership of the iterator i.e v1_iter

    assert_eq!(total, 6);

    println!("{:?}, {:?}",v1);
}

9.

/* Make it work */
use std::collections::HashMap;
fn main() {
    let names: [(&str, i32);2] = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<&str , i32 > = names.into_iter().collect();

    println!("{:?}",folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.int0_iter().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}


Iterator adaptors
Methods allowing you to change one iterator into another iterator are known as iterator adaptors.
You can chain multiple iterator adaptors to perform complex actions in a readable way.

But because all iterators are lazy, you have to call one of the consuming adapters to get results from
calls to iterator adapters.

10.
/* Fill in the blanks */
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect(); // here map is an iterator adaptor that
// takes a closure and creates a new iterator that calls this closure on each element

    assert_eq!(v2, vec![2, 3, 4]);
}

11.
/* Fill in the blanks */
use std::collections::HashMap;
fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap< &str,i32> =names.into_iter().zip(ages.into_iter()).collect();
        println!("{:?}",folks);

}
