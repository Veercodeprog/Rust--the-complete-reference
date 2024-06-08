// borrowing: way of temporarily accessing a value without taking ownership of it.
// when borrowing , we are taking a reference pointer to the value and not the value itself.
// helps in prevention of dangling pointers and data races.
// data can be moved or borrowed mutably or immutably.
// Rules of referencing:
// 1. At any given time, you can have either one mutable reference or any number of immutable
// references to a particular piece of data.
// 2. References must always be valid.
//
// fn main() {
//    let s1 = String::from("hello");
//    let y = x; // solution is to use the clone method // let y = x.clone();
//    println!("{}", x,y); // this will not work as x has been moved and is no longer valid.
//    }
//    to avoid this we can use borrowing using the clone method.
//
//    fn main() {
//    let s: String = gives_ownership(); // gives_ownership moves its return value into s1.
//    println!("{}", s);
//    }
//
//    fn gives_ownership() -> String {
//    let s:String = String::from("hello");
//    let _s = s.as_bytes();    // let _s = s.into_bytes(); // if we use into_bytes() here, s will be moved and we will not be
//                             // able to use it again after this line. so we use the as_bytes() method which takes the
//                             // reference to the value and not the value itself.
//    s
//    }
//
//    fn main() {
//    let x: (i32, i32 , (), String) = (1, 2, (), String::from("hello"));
//    let y : (i32, i32, (), String) = x.clone() ; // x is copied to y as it is a tuple and all the values
//    are stored on the stack.
//    println!("{:?},{:?}  ", x, y);
//    }
//
//    we can write the above program correctly as:
//    fn main() {
//    let x: (i32, i32 , (), &str) = (1, 2, (), "hello");
//    let y : (i32, i32, (), &str) = x; // x is copied to y as it is a tuple and all the values
//    println!("{:?},{:?}  ", x, y);
//    }
//
//    Mutability can be changes when ownership is transferred or borrowed.
//
//    fn main() {
//    let x : Box<i32> = Box::new(5); //here x is a box pointer to the heap where the value 5 is
//    stored.
//    let y : Box<i32> = Box::new(1); // here y is a box pointer to the heap where the value 1 is
//    *y =4; // this will not work as y is immutable. to make it work make y mutable.
//    assert_eq!(*x, 4);
//    println!("success");
//    }

fn main() {
    #[derive(Debug)]
    struct Person {
        // struct point is defined here.it is a tuple struct.
        name: String,
        age: Box<u8>,
    }
    let person: Person = Person {
        name: String::from("John"),
        age: Box::new(25), // Box is a smart pointer that points to the heap where the value is
                           // stored
    };
    //name is moved out of person and is no longer valid.whereas age is referenced and thus only
    //borrowed but not moved.
    let Person { name, ref age } = person; // destructuring the person struct into name and age.
                                           // ref is used to borrow the value and not take
                                           // ownership of it.
    println!("the person's name is {} and age is {}", name, age);
    println!("the person's name is {} and age is {}", name, person.age);

    let mut s = String::from("hello");
    let r1 = &mut s;
    //    let r2 = &mut s; // this will not work as we cannot have two mutable references to the same
    // value at the same time.
    println!("{},{}", r1, r2);
}
