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
}
