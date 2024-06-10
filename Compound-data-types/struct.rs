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
//   tuple structs are useful when you want to give the whole tuple a name and make the tuple a
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
