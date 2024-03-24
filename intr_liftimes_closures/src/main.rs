
// fn f1(i: &i32) -> &i32 {
//     &i
// }
//
// fn main() {
//     let i = 10;
//     let val = f1(&i);
//     println!("{}", val);
// }


// fn main() {
//     let s1 = "hello";
//     let a;
//
//     {
//         let s2 = String::from("world");
//         a = f1(s1, s2.as_str());
//     }
//     println!("{}", a);
// }
//
// fn f1<'a, 'b>(str1: &'a str, str2: &'b str) -> &'a str {
//     str1
// }

// fn main() {
//     let i1 = 5;
//     let i2 = 20;
//     println!("{}", greater(&i1, i2));
// }
//
// fn greater<'a>(i: &'a i32, j:i32) -> &'a i32 {
//     i
// }
//


// fn main() {
//     let i1 = 5;
//     let i2 = 20;
//     println!("{}", crate::greater(&i1, &i2));
// }
//
// fn greater<'a, 'b>(i: &'a i32, j:&'a i32) -> &'a i32 {
//     if i > j {
//         i
//     } else {
//         j
//     }
// }

// struct Person<'a> {
//     name: &'a str,
//     age: i32,
// }
//
// fn main() {
//     let name = "hello";
//     let mut p = Person{
//         name: &name,
//         age: 40
//     };
//
//     {
//         let other_name = String::from("world");
//         p.name = &other_name
//     }
//     println!("{} {}", p.name, p.age);
// }

use std::path::Component::ParentDir;

// fn main() {
//     let vec = vec![5,23,45,3,23];
// }
//
// fn f1<'a>(v1: &'a [i32], v2:&'a [i32]) -> &'a [i32] {
//     if 3 > 5 {
//         v1
//     } else {
//         v2
//     }
// }

//closures

fn main() {
    // let x =7;
    // let sq = |num: i32| println!("the square is {}", num * num);
    // sq(x);
    //
    // let y = 34;
    //
    // sq(y);

    // let a = |general_info: String, name: &str, age| println!("{} {} {}", general_info, name, age);
    // let gen = String::from("details are");
    // let (name, age) = (String::from("hello"), 23);
    // a(gen, &name, &age);

    // let sq = |num| num * num;
    // let x = 5;
    // // sq(x);
    //
    // let y = 105.5;
    // sq(y);

//     let status  = |y: f32| { if y != 0.0 {true} else {false}};
//     div(5.0, 0.0, status);
// }
//
// fn div<F: Fn(f32) -> bool>(x: f32, y: f32, f:F) {
//     if f(y) == true {
//         println!("the div res {}", x / y);
//     } else {
//         println!("error")
//     }

    // let s1 = |x: u32| -> u32 {x + 1};
    // let s2 = |x| {x + 1};
    // let s3 = |x|x + 1;
    //

    // let mut v1 = vec![1,2,3];
    // let mut s = || {
    //     v1.push(3245);
    // };
    // // println!("{:?}", v1);
    // s();

    //function types
    // let mut f = min;
    // println!("{} ", f(3,2));
    // let (n, a) = (String::from("hello"), 10);
    // info(name, &n, a);

    println!("{}", do_twice(add_one, 10));

}

fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// fn name(name: &str) {
//     println!("{}", name);
// }
//
// fn info(f: fn(&str), a: &str, age: i32) {
//     f(a);
//     println!("{} ", age);
// }

// fn max(x: i32, y: i32) -> i32 {
//     if x > y {x}
//     else {y}
// }
//
// fn min(x: i32, y: i32) -> i32 {
//     if x < y {x}
//     else {y}
// }