//macro
macro_rules! string_concat {
    () => {
        String::new();
    };
    ($str: expr) => {
        {
            let mut tmp = String::new();
            tmp.push_str($str);
            tmp
        }
    };
}

fn main() {
    let null_str = string_concat!();
    let str1 = string_concat!("First");
    let str1 = string_concat!("Second");
    println!("{} {}", str1, str2);
}
