fn main() {
    let val = {
        if 5 < 10 {
            1
        } else {
            2
        }
    };

    println!("{}", val);
    
    let val1 = {
        if 10 > 14 {
            1
        } else {
            2
        }
    };

    println!("{}", val1);
    
    let num = 84;
    let grade = match num {
        100 => 'S',
        91..=99 => 'A',
        81..=90 => 'B',
        71..=80 => 'C',
        61..=70 => 'D',
        _ => 'F'
    };

    println!("The grade acheived is {}", grade);
}
