
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

fn main() {
    let vec = vec![5,23,45,3,23];
}

fn f1<'a>(v1: &'a [i32], v2:&'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        v1
    } else {
        v2
    }
}