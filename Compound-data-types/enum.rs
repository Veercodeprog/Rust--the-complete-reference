//Enum
//way of defining a type with only one of the a possible set of values.
//we can only access one variant of enum at a time.
//can hold additional information using tuples or structs.
//especially useful when used in match statement.
//
//eg
//enum IpAddr{
//   V4(String),
//   V6(String),
//   }
//
//   let home = IpAddr::V4(String::from("127.0.0.1"));
//
//   let loopback = IpAddr::V6(String::from("::1"));
//
//enum vs structs
//in a struct, we would have provided concrete values for each field.
//in an enum,we instantiate the enum with one single variant.
//
//enum Number{
//   One, //1
//   Two, //2
//   Three, //3
//   }
//
//   enum Number1{
//   zero=0,//0
//   one, //1
//   two, //2
//    }
//
//    enum Number2{
//    zero= 1 ,
//    one =2 , //2
//    two=3
//    }
//
// /   fn main() {
//         // An enum variant can be converted to a integer by `as`
//         assert_eq!(Number::One as u8, Number1::One as u8);
//         assert_eq!(Number1::One as u8, Number2::One as u8);
//         println!("{}" , Number::One as u8 );
//
//         println!("Success!");
//     }
//
//
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg);
    }
}
fn show_message(msg: Message) {
    println!("{:?}", msg);
}

//🌟🌟 Since there is no null in Rust, we have to use enum Option<T> to deal with the cases when the value is absent.
//🌟🌟 Option<T> is an enum defined in the standard library.
//1.) Option is an enum that represents a value that may or may not be present.
//2.) Known in other languages as "null" , referring to absence of values .
//3.) Option<T> is generic, so it can hold values of any type.
//4.) USed in cases where a function or method might fail to return a value.
//template for Option<T> enum:
// enum Option<T> {  // T is a generic type parameter meaning the Some variant of the Option enum
// can hold one piece of data of any type.
// None,
// Some(T),
// }
//
///////////////////////////
// fn main() {
//     let five: Option<i32> = Some(5);
//     let six: Option<i32> = plus_one(five);
//     let none: Option<i32> = plus_one(None);
//
//     if let Some(n) = six {
//         println!("{}", n);
//
//         println!("Success!");
//     }
//
//     panic!("NEVER LET THIS RUN！");
// }
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//        None  => None,
//         Some(i) => Some(i + 1),
//     }
// }
