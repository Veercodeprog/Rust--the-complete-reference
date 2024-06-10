//powerful  construct that allows to compare a value against a set of patterns , then execute
//different code based on which pattern matches.
//
//pattern can be made up of literals, variable names, wildcards, and many other things.
//
//in match expression, the value is compared to the pattern of each arm, in order.
//
//
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire: Direction = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            println!("North or South");
        }
        _ => println!("West"),
    };
}

// Match is an expression, so it returns a value and we can use it in assignements.
// fn main() {
// let boolean: bool = true;
//
// let binary = match boolean {
// true => 1,
// false => 0,
// };
// assert_eq!(binary, 1);
// }
//
//
// using match to get the data an enum variant holds.
//
// enum Message {
// Quit,
// Move { x: i32, y: i32 },
// Write(String),
// ChangeColor(i32, i32, i32),
// }
// fn main() {
// let msgs[Message;3] = [
// Message::Quit,
// Message::Move { x: 3, y: 4 },
// Message::ChangeColor(255,255,0),
// ];
//
// for msg in msgs(){
// show_message(msg);
// }
// }
//
// fn show_message(msg: Message){
// match msg {
//  Message::Move{x:a,y:b}  => {
//   assert_eq!(a,3);
//   assert_eq!(b,4);
//   }
//   Message::ChangeColor(r,g,b) => {
//   assert_eq!(r,255);
//   assert_eq!(g,255);
//   assert_eq!(b,0);
//   },
//
//   __ => println!("no data in these variants")
//   }
//   }
//
//
//   matches looks like match but do something different.
//   fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];
//
//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
//     }
//
//     println!("Success!");
// }
//
//
//
//
// enum MyEnum {
//     Foo,
//     Bar
// }
//
// fn main() {
//     let mut count = 0;
//
//     let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) { // Fix the error by changing only this line
//             count += 1;
//         }
//     }
//
//     assert_eq!(count, 2);
//
//     println!("Success!");
// }
//
// If let:
// for some cases when matching enums , match is too heavy . we can use if let.
//
//
// fn main() {
//     let o = Some(7);
//
//     // Remove the whole `match` block, using `if let` instead
// match o {
//     Some(i) => {
//         println!("This is a really long string and `{:?}`", i);
//
//         println!("Success!");
//     }
//     _ => {}
// };
//
//     if let Some(i) = o {
//     println!("This is a really long string and `{:?}`", i);
//     println!("Success!");
//     }
// }
//
//
//
// Fill in the blank
// enum Foo {
//     Bar(u8)
// }
//
// fn main() {
//     let a = Foo::Bar(1);
//
//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);
//
//         println!("Success!");
//     }
// }
//
//
//
// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }
//
// fn main() {
//     let a: Foo = Foo::Qux(10);
//
//     // Remove the codes below, using `match` instead
//
//     match a{
//     Foo::Bar => println!("match foo::bar"),
//
//     Foo::Baz => println!("match foo::baz"),
//     _ => println!("match others")
//     }

// }
//
//
// SHADOWING:
//
//
// Fix the errors in-place
// fn main() {
//     let age: Option<i32> = Some(30);
//     if let Some(age) = age { // Create a new variable with the same name as previous `age`
//        assert_eq!(age, 30);
//     } // The new variable `age` goes out of scope here
//
//     match age {
//         // Match can also introduce a new shadowed variable
//         Some(age) =>  println!("age is a new variable, it's value is {}",age),
//         _ => ()
//     }
//  }
//
//
//  PATTERNS:
//  1.🌟🌟 Use | to match several values, use ..= to match an inclusive range.
//
// fn main() {}
// fn match_number(n: i32) {
//     match n {
//         // Match a single value
//         1 => println!("One!"),
//         // Fill in the blank with `|`, DON'T use `..` or `..=`
//         2 | 3 |4 | 5 => println!("match 2 -> 5"),
//         // Match an inclusive range
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match -infinite -> 0 or 11 -> +infinite")
//         }
//     }
// }
//
// 2.🌟🌟🌟 The @ operator lets us create a variable that holds a value, at the same time we are
// testing that value to see whether it matches a pattern.
//
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// fn main() {
//     // Fill in the blank to let p match the second arm
//     let p = Point { x: 3, y: 30};
//
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {}", x),
//         // Second arm
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x, y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }
//
//3.
// Fix the errors
// enum Message {
//     Hello { id: i32 },
// }
//
// fn main() {
//     let msg:Message = Message::Hello { id: 5 };
//
//     match msg {
//         Message::Hello {
//             id: id @ 3..=7,
//         } => println!("Found an id in range [3, 7]: {}", id),
//         Message::Hello { id: newid @ (10 | 11 | 1)2 } => {
//             println!("Found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id } => println!("Found some other id: {}", id),
//     }
// }
//
// 4.🌟🌟 A match guard is an additional if condition specified after
// the pattern in a match arm that must also match, along with the pattern matching,
// for that arm to be chosen.
//
// Fill in the blank to make the code work, `split` MUST be used
// fn main() {
//     let num : Option<i32> = Some(4);
//     let split: i32 = 5;
//     match num {
//         Some(x) if x < split  => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }
//
//     println!("Success!");
// }
//
// 5.🌟🌟 Ignoring remaining parts of the value with ..
//
//
// Fill the blank to make the code work
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
//
//     match numbers {
//       (first,.., last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }
//
//     println!("Success!");
// }
//
// 6.🌟🌟 Using pattern &mut V to match a mutable reference requires you to be
// very careful, due to V being a value after matching.
//
// FIX the error with least changing
// DON'T remove any code line
// fn main() {
//     let mut v: String = String::from("hello,");
//     let r: &mut String = &mut v;
//
//     match r {
//         value => value.push_str(" world!")
//     }
// }
