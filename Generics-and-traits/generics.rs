// Generics:
// Placeholders for concrete types in functions, structs, enums, and methods.
// enables writings more reusable and flexible code.
// avoids having to write duplicate code for different types.
// zero-cost abstractions. rust compiler will at compile time fill out generics with concrete
// types.
//
// const generics:
// Type parameter that represents a compile time constant value.
// allows to write generic code that operated with values that are known at compile time.
// used for array sizes , bit widths and other constants.
//
//1.) Generic functions:
// Fill in the blanks to make it work
// struct A; // Concrete type `A`.
// struct S(A); // Concrete type `S`.
// struct SGen<T>(T); // Generic type `SGen`.
//
// fn reg_fn(_s: S) {}
//
// fn gen_spec_t(_s: SGen<A>) {}
//
// fn gen_spec_i32(_s: SGen<i32>) {}
//
// fn generic<T>(_s: SGen<T>) {}  //only generic function
//
// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A)); // Concrete type.
//     gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.
//
//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('A'));
//
//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen('Z')); // we can even pass any type here as we are not using explicit
//     definition here like generic::<char>(SGen('A'));, so even if we wrirt generic(SGen(6.7)); it
//     will give no error.
//
//     println!("Success!");
// }
//
// 2.) a function type with explicitly specified type parameters looks like this:
// fn::<A, B, .. >(args)
//
// fn sum<T: std::ops::Add<Output = T > >(a: T, b: T) -> T {
//    a + b
//    }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
//
//     println!("Success!");
// }
//
// 3.🌟

// Implement struct Point to make it work.
// struct Point<T> {
//    x: T,
//    y: T,
//    }
// fn main() {
//     let integer : Point = Point { x: 5, y: 10 };
//     let float: Point = Point { x: 1.0, y: 4.0 };
//
//     println!("Success!");
// }
//
// 4.🌟🌟
//
// Modify this struct to make the code work
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// fn main() {
//     // DON'T modify this code.
//     let p: Point<i32,String> = Point{x: 5, y : "hello".to_string()};
//
//     println!("Success!");
// }
//
// 5.🌟🌟
//
// Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: <T>,
// }
//
// impl <T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }
//
// fn main() {
//     let x: Val<f64> = Val { val: 3.0 };
//     let y: Val<String> = Val {
//         val: "hello".to_string(),
//     };
//     println!("{}, {}", x.value(), y.value());
// }
//
// 6.)🌟🌟🌟
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<V,W>(self,other: Point<V,W>) -> Point<T,W> {
//     Point {
//     x: self.x,
//     y: other.y,
//     }
// }
//
// fn main() {
//     let p1: Point<i32, i32> = Point { x: 5, y: 10 };
//     let p2: Point<&str, char> = Point { x: "Hello", y: '中'};
//
//     let p3: Point <i32,char> = p1.mixup(p2);
//
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
//
//     println!("Success!");
// }
//
// 7.)🌟🌟🌟
//
// Fix the errors to make the code work.
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p: Point <f64> = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin());
// }
//
// Const Generics:
// Const Generics are generic arguments that range over constant values rather than types or
// lifetimes. This allows for instance types to be generic over integers or other constants.
// In fact there has been one example of const generic types since early on in Rust's development:
// the array size in the array type [T; N], for some type T and N: usize.However there has
// previously been no way to abstract over arrays of arbitrary size:if you wanted to implement a
// trait for arrays of any size, you would have to do so for each possible size value.For a long
// time, even the standard library methods for arrays were limited to arrays of length at most 32
// due to this problem.
//
// Here's an example of a type and implementation making use of const generics: a type wrapping
// a pair of arrays of the same size.
//
// struct ArrayPair<T, const N: usize> {
//     left: [T; N],
//     right: [T; N],
// }
//
// impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
//     // ...
// }
//
// 1.)🌟🌟 <T, const N: usize> is part of the struct type, it means Array<i32, 3> and Array<i32, 4>
// are different types.
//
// struct Array<T, const N: usize> {
//     data : [T; N]
// }
//
// fn main() {
//     let arrays:[Array<i32, 3>; 3] = [
//         Array{
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [1, 2, 3],
//         },
//         Array {
//             data: [4,5,6]
//         }
//     ];
//
//    let floats: [Array<f64,2>;3] = [
//    Array{ data: [1.1, 2.2]},
//    Array{ data: [3.3, 4.4]},
//    Array{ data: [5.5, 6.6]}
//    ];
//     println!("Success!");
// }
//
//
// // reamining 2 exercises do later.
