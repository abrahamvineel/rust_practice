fn main() {
    let x1 = 'a';
    println!("The value of c1 is {}", x1);
    println!("The max val of i8 is {}", i8::MAX);

    let (first_num, second_num) = (13.5, 14);
    let large_num = 1_000_000;

    println!("vals are {} {} {}", first_num, second_num, large_num);

    let n1 = 14;
    let n2 = 15.6;

    let n3 = n1 as f64 + n2;
    println!("sum is {}", n3);

    //shadowing

    let r = 10;
    let r = 20;

    println!("r val after shadow {}", r);

    let mut p = 1;
    p = 5*5;

    println!("p val after shadow {}", p);

    let q = 32;
    let q = 'A';

    println!("q val after shadow {}", q);

    let r = 65;
    {
        let r = 60;
        println!("r val in statements {}", r);
    }
    println!("r val outside statements {}", r);

    const MAX_SALARY: i32 = 100_000;
    println!("MAX_SALARY is {}", MAX_SALARY);

    let fixed_string = "hello world";
    println!("fixed string {}", fixed_string);

    let mut growable_string = String::from("hello world world hello");
    println!("the string is : \"{}\"", growable_string);

    growable_string.push('s');
    println!("the string is : \"{}\"", growable_string);

    growable_string.pop();
    println!("the string is : \"{}\"", growable_string);

    growable_string.push_str("star");
    println!("the string is : \"{}\"", growable_string);

    println!("the string is : \"{}\", capacity {}, len {}, is empty {}", growable_string, growable_string.capacity(), growable_string.len(), growable_string.is_empty());

    let trim_str_len = growable_string.trim().len();
    println!("the string len after trim is : \"{}\"", trim_str_len);


    let number = 5;
    let num_str = number.to_string();
    println!("did number get converted to string? {}", num_str == "5");

    let some_char = 'a';
    let char_string = some_char.to_string();
    println!("the string is : \"{}\"", char_string);

    let name = "Abraham".to_string();
    println!("the string is : \"{}\"", name);
    let empty_str = String::new();
    println!("len is {}", empty_str);

    let s_1 = "hello".to_string();
    let s_2 = "world".to_string();
    let s_3 = format!("I am printing {} and {}", s_1, s_2);
    println!("{}", s_3);

    let concat = format!("{}{}", s_1, s_2);
    println!("{}", concat);

    let my_information = ("Salary", 40_000);
    println!("My {} is {}", my_information.0, my_information.1);

    let (salary, salary_value) = my_information;

    let mut arr : [i64;5] = [2,4,5,6,1];
    println!("3rd num is {}", arr[2]);

    //Option enum type has none or some
    let val = arr.get(100);
    println!("{:?}", val);

    //Vectors

    let num: Vec<u8> = vec![1, 2, 3, 4];
    let char_vec:Vec<char> = vec!['a';5];
    let (add, mul, sub) = math(10, 5);
    println!("add {}, mul {} , sub {}", add, mul , sub);

    let div = {
        let n1 = 10;
        let n2 = 5;
        n1 / n2
    };
    println!("div {}", div);

    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    let n:f64 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);
}


//functions
fn math(n1: i32, n2: i32) -> (i32, i32, i32) {
    (n1 + n2, n1 * n2, n1 -n2)
}
