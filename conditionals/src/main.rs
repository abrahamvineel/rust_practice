fn main() {
    let val = {
        if 5 < 10 {
            1
        } else {
            2
        }
    };

    println!("{}", val);

    let num = 84;
    let grade = match num {
        91..=100 => 'A',
        81..=90 => 'B',
        71..=80 => 'C',
        61..=70 => 'D',
        _ => 'F'
    };

    println!("The grade acheived is {}", grade);
}
