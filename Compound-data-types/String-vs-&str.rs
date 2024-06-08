// A string is a heap allocated string type that owns its content and is mutable.
// A &str is a string slice, a reference to a string. It cannot be mutated and is not owned by the
// caller.  it does not own the underlying data.it is basically an immutable sequence of UTF-8
// bytes in memory.
// think of &str as a view on a sequence of characters (stored as UTF-8 bytes) in memory
//
// str and &str:
// 1.) we cant use str type in normal ways but we can use &str type.
//
// fn main() {
// let s: &str = "hello"; // this is a string slice and is stored on the stack.
// println!("{}", s);
// }
//
// 2.) we can only use str by boxing it or boxed it , & can be used to convert Box<str> to &str.
// fn main() {
// let s: Box<str> = "hello, world".into(); // into method will convert the string slice to a
// Box<str> type.
// greetings(&s);
// }
// fn greetings(s: &str) {
//
// println!("{}", s);
// }
//
//
// String:
// String type is defined in std and stored as a vector of bytes in the heap. but guaranteed to
// alwasy be a valid UTF-8 sequence of bytes. it is a growable, mutable, owned, heap-allocated.
//fn main() {
//let mut s :String = String::from("hello");
//s.push_str(", world"); // push_str method is used to append a string slice to the string.
//s.push('!'); // push method is used to append a single character to the string.
//
//assert_eq!(s, "hello, world!");
//println!("{}", s);
//}
//we can only concat a String with &str , and String's ownership will be moved to the another variable
//or &str.
//fn main() {
//let s1:String = String::from("hello");
//let s2:String = String::from(", world");
//let s3: String = s1 + s2.as_str(); // String -> &str.
//assert_eq!(s3, "hello, world");
//println!("{}", s3);
//}
//
//opposite to the seldom using of str, &str(string slice) and String are used frequently.
//&str can be converted to String in two ways:
//
//fn main() {
//let s: &str = "hello";
//greeting(s.to_string()); //using to_string() method or we can write String::from(s) also in place
//of s.to_string().
//}
//fn greeting(s: String) {
//println!("{}", s);
//}
//
//we can use String::from() or to_string() method to convert &str to String.
//
//String escapes
//
//fn main() {
//we can use escape characters in the string to write the bytes by their hexadecimal values.
//let byte_escape = "I'm writing Ru\x73\x74!";
//println!("what are you doing\x3F (\\x3F means ?) {}", byte_escape);
//
// let unicode_codepoint = "\u{211D}";
// let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
//
// println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);
// let long_string = "String literals
// can span multiple lines.
// The linebreak and indentation here ->\
// <- can be escaped too!";
// println!("{}", long_string);
// }
//
// in raw strings, escapes dont work.
//
// we cant use index to access a char in a string, but you can use slice &str[start..end] to get a
// substring.
//
// fn main() {
// let s:String = String::from("hello");
// let h = &s[0..1]; // for first character access
// assert_eq!(h, "h");
//
// let h1 = &s[3..5]; // for 4th and 5th character access
// assert_eq!(h1, "lo");
// println!("{}", h);
// }
//
// we can use chars() method to get the characters of a string ,that operates on UTF-8 characters.
