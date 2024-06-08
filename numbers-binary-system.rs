// Length
// i represents signed integer, u represents unsigned integer
// 1. i8: 8 bits i.e. 1 byte , -128 to 127, u8: 0 to 255
// 2. i16: 16 bits i.e. 2 bytes, -32768 to 32767, u16: 0 to 65535
// 3. i32: 32 bits i.e. 4 bytes, -2147483648 to 2147483647, u32: 0 to 4294967295
// 4. i64: 64 bits i.e. 8 bytes, -9223372036854775808 to 9223372036854775807, u64: 0 to
//    18446744073709551615
//    5. i128: 128 bits i.e. 16 bytes, -170141183460469231731687303715884105728 to
//    170141183460469231731687303715884105727, u128: 0 to 340282366920938463463374607431768211455
//    6. arch:  isize: depends on the architecture of the system, 32 bits on 32-bit systems and 64 bits on
//       64-bit systems, usize: same as isize .this is also called the pointer size integer type
//
// Number - default types
// 1. i32 is the default type for integers
// 2. f64 is the default type for floating-point numbers
//
// Numbers - what is a word?
// 1. A word is the number of bits that can be handled or read or access by the CPU in one go
// 2. For example, a 32-bit CPU can handle 32 bits in one go
// 3. A 64-bit CPU can handle 64 bits in one go
// we can assign a variable of a type to a variable of another type
// eg fn main() {
//    let x: i32 = 10;
//    let mut y: i64 = 0;
//    y = x ; // this isn't allowed
//
//    fn main()
//   {
//   let v1:u8 = 251_u8 + 8_u8;
//   let v2 = i8::checked_add(251, 8).unwrap();
//   println!("v1: {}, v2: {}", v1, v2);
//   }
//
//
fn main() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("v1: {}, v2: {}", v1, v2);

    //in  rust we can perform operations in different number systems
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1_024 + 255 + 63 + 255);
    println!("v: {}", v);

    let x: f64 = 1.0;
    let y: f32 = 1.0;
    let z: f64 = 0.01_f64;
    assert_eq!(type_of(&x), "f64".to_string());
    //for loops
    //
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -3 + -2 + -1 + 0 + 1);

    for c in 'a'..='z' {
        println!("{}", c);
    }
}
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
