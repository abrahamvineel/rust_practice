fn main() {
    let mut a_number;
    let a_word = "Ten";

    a_number = 10;
    println!("The number is {}.", a_number);
    a_number = 15;
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);


    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 5;

    println!("The number is {}.", shadow_num);

    let number: u32 = 14;
    println!("The number is {}.", number);

    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);
}
