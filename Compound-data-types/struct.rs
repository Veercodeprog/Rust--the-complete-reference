// Struct
// compound type allowing to store multiple values of different types into a named  data
// structure.
// similar to tuples but each value has the name so values can be accessed through this name.
// have to instantiated with data , think of it like the struct is the template for the instances
// you create from it
//
// instantiating a struct
//
// template for the struct
// struct User {
//  active: bool,
//  username: String,
//  email: String,
//  sign_in_count: u64,
//  }
//
//  creating an instance of the struct user
//  fn main() {
//   let user1 = User {
//   active: true,
//   username: String::from("someusername123"),
//   email: String::from("someuser@example.com"),
//   sign_in_count: 1,
//   };
//   }
//
//   we can also create a mutable instance of the struct.
//   fn main() {
//   let mut user1 = User {
//   active: true,
//   username: String::from("username123"),
//
//   email: String::from("username123@example.com"),
//   sign_in_count: 1,
//   };
//  mutating the instance of the struct
//   user1.email = String::from("anotheremail@example.com");
//
//   function returning a struct
//
//   fn build_user(email: String, username: String) -> User {
//   User {
//   active: true,
//   username: username,
//   email: email,
//   sign_in_count: 1,
//   }
//   }
//
//   Tuple structs
//   tuple structs are a way of creating a struct that looks like a tuple but has a different name
//   and type.
//   tuple structs are useful when you want to give the whole tuple a name but dont care about the
//   fields name and make the tuple a
//   different type from other tuples.
//   instantiated by parenthesis and the values are separated by commas.
//   accessed through point notation.
//
//   struct Color(i32, i32, i32);
//   struct Point(i32, i32, i32);
//   fn main() {
//   let black = Color(0, 0, 0);
//   let origin = Point(0, 0, 0);
//   println!("black: {}, {}, {}", black.0, black.1, black.2);
//   println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
//   }
//
//   Unit-like structs
//   structs that don't have any fields are called unit-like structs because they behave similarly
//   to the unit type, ().
//   useful when you want to implement a trait on a type but don't have any data to store in the
//   type itself
//   does not have any data associated with it.
//
//
//  1. 🌟 We must specify concrete values for each of the fields in struct.

// Fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: String::from("reading")  //answer
//     };
//
//     println!("Success!");
// }
//
// 2. 🌟 Unit type dont have nay fields associated with it. it can be useful when you need to
//    implement a trait on some type but dont have any data to store in the type itself.
//
//    struct Unit;  //unit struct
//    trait SomeTrait {
//    // .. some behaviors defined here
//    }
//
//    impl SomeTrait for Unit { }
//    fn main() {
//let u = Unit;
//     do_something_with_unit(u);
//
//     println!("Success!");
//    }
//    fn do_something_with_unit(u: Unit) {
//
//    }
//
//    3.
//    Strcut Color(i32, i32, i32);
//    struct Point(i32, i32, i32);
//    fn main() {
//    let v:Point  = Point(0,127,255);
//    check_color(v);
//    }
//    fn check_color(p: Point) {
//    let (x, y, z) = p;
//    assert_eq!(x, 0);
//    assert_eq!(p.1, 127);
//    assert_eq!(z, 255);
//    }
//
//4.we can make a whole struct mutable by using the mut keyword in front of the variable name but
//Rust does not allow us to make a single field mutable within a struct.
//
// 5.Using field init shorthand syntax to reduce repetition or redundancy in the code.
//
// struct Person {
//    name: String,
//    age: u8,
//    }
//
//    fn main() {
//    println!("Success!");
//    }
//
//    fn build_person(name: String, age: u8) -> Person {
//    Person {
//    name,
//    age,
//    }
//
//    5. we can use #[derive(Debug)] to print the struct instance.
//    #[derive(Debug)]
//    struct Rectangle {
//    width: u32,
//    height: u32,
//    }
//    fn main() {
//    let scale:u32  = 2;
//    let rect1: Rectangle = Rectangle {
//    width: dbg!(30 * scale), //print debug into stdderr and assign the value to width
//    height: 50,
//    };
//
//    dbg!(&rect1); //print debug into stdderr
//    println!("{:?}"  , rect1); //print debug into stdderr
//    }
//
//
//    Partial move:
//    withing destructuring of a single variabel,both by move and by-reference pattern bindings can
//    be used at the same time.Doing so is called partial move, which means that parts of the
//    variable will be moved while other parts stay in place.   In such a case parent variable
//    cannot be used after the partial move as a whole, however, the parts that are only
//    referenced(and not moved) can still be used
//

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }
//
//     let person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };
//
//     // `name` is moved out of person, but `age` is referenced
//     let Person { name, ref age } = person;
//
//     println!("The person's age is {}", age);
//
//     println!("The person's name is {}", name);
//
//     // Error! borrow of partially moved value: `person` partial move occurs
//     //println!("The person struct is {:?}", person);
//
//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }
