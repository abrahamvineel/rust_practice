fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_val: Option<u32> = stack.pop();
    println!("The popped value is {:?} ", popped_val);
    popped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Stack size full");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn stack_size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    let n = n.trim().parse().expect("invalid input");
    n
}

fn main() {
    let size = input();

    let mut stack = new_stack(size as usize);
    loop {
        let option = input();
        match option {
            1 => {
                let item = input();
                push(&mut stack, item, size as usize);
            }
            2 => println!("popped val {:?}", pop(&mut stack)),
            3 => println!("The elements are {:?}", stack),
            4 => println!("The size of stack {}", stack_size(&stack)),
            5 => break,
            _ => println!("Invalid option")
        }
    }
}
