
// fn f1(i: &i32) -> &i32 {
//     &i
// }
//
// fn main() {
//     let i = 10;
//     let val = f1(&i);
//     println!("{}", val);
// }


fn main() {
    let s1 = "hello";
    let a;

    {
        let s2 = String::from("world");
        a = f1(s1, s2.as_str());
    }
    println!("{}", a);
}

fn f1<'a, 'b>(str1: &'a str, str2: &'b str) -> &'a str {
    str1
}