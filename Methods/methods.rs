// Methods in rust:
// function that is associated with a partiucular type or struct.
// Takes a parameter and returns a value , but defined as a member of a struct or enum.
// called using dot notation.
// implemented through a impl block.
//
// eg
// #[derive(Debug)]
// struct Rectangle {
//    width: u32,
//    height: u32,
//    }
//
//    impl Rectangle {
//     fn area(&self) -> u32 {
//     self.width * self.height
//     }
//     }
//
//     self here is replaced with the instance of the struct.
//
//     let rect1 = Rectangle { width: 30, height: 50 };
//     println!("The area of the rectangle is {} square pixels.", rect1.area());
//
//
//     Associated functions:
//   functions that are associated with the struct or an enum but doesnt take an instance as it's
//   first parameter.
//   called using the name of the type not an instance of it.
//   often used as a constructor that will return a new instance of the struct or enum.
//
//   #[derive(Debug)]
//   struct Rectangle {
//   width: u32,
//   height: u32,
//   }
//
//   impl Rectangle {
//     fn new (width: u32, height: u32) -> Rectangle {
//     Rectangle { width, height }
//     }
//     }
//
//     new() is an associated function becasue it doesnt take self as it's first parameter.so to
//     call a associated function we dont need an instance of the struct.
//
//     we can call the associated function by using the name of the struct and the method name
//     separated by "::" operator.
//     let rect1 = Rectangle::new(30, 50);
//     println!("rect1 is {:?}", rect1);
//
//
//     1.)Methods are similar to functions: Declare with fn , have parameters and return values.
//    unlike functions, methods are defined within the context of a struct or an enum or a trait
//    object and their first parameter is always self, which represents the instance of the struct
//    the method is being called on.
//
//    2.)🌟🌟 self will take the ownership of current struct instance, however, &self will only
//    borrow a reference from the instance.
//
//    // Only fill in the blanks, DON'T remove any line!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     pub fn show_state(&self) {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // Don't take the ownership of `light` here.
//     light.show_state();
//     // ... Otherwise, there will be an error below
//     println!("{:?}", light);
// }
// 3.🌟🌟 The &self is actually short for self: &Self. Within an impl block, the type Self is an
// alias for the type that the impl block is for. Methods must have a parameter named self of type
// Self for their first parameter, so Rust lets you abbreviate this with only the name self in the
// first parameter spot.
//
// &self == self: &Self
//
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // Using `Self` to fill in the blank.
//     pub fn show_state(self: &Self) {
//         println!("the current state is {}", self.color);
//     }
//
//     // Fill in the blank, DON'T use any variants of `Self`.
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string()
//     }
// }
// fn main() {
//     println!("Success!");
// }
//
// 4.🌟🌟🌟 All functions defined within an impl block are called associated functions,because they
// are associated with the type named after the impl keyword.wecan define associated functions that
// dont have self as first parameter(and thus are not methods) because they dont need an instance
// of the type (struct or enum) to work with.
//
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 1. Implement an associated function `new`,
//     // 2. It will return a TrafficLight contains color "red"
//     // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
//     pub fn new()->Self{
//     Self{
//     color: String::from("red")
//     }
//
//
//     }
//
//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }
//
// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
//
//     println!("Success!");
// }
//
// 5. 🌟 Each struct is allowed to have multiple impl blocks.
//
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // Using multiple `impl` blocks to rewrite the code below.
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//
// }
// impl Rectangle {
// fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
//
//     }
//
//
// fn main() {
//     println!("Success!");
// }
//
// 6.🌟🌟🌟 We can also implement methods for enums.
//
// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }
//
// // Implement TrafficLightColor with a method.
// impl TrafficLightColor {
//   fn color(&self) -> &str {
//      match self {
//      Self::Yellow => "yellow",
//     Self::Red => "red",
//     Self::Green => "green",
//     }
//   }
//
//
// fn main() {
//     let c:TrafficLightColor = TrafficLightColor::Yellow;
//
//     assert_eq!(c.color(), "yellow");
//
//     println!("{:?}", c);
// }
