fn new_stack(maxsize: usize) -> Vec<String> {
    let vec: Vec<String> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    let popped_val: Option<String> = stack.pop();
    println!("The popped value is {:?} ", popped_val);
    popped_val
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) {
    if stack.len() == maxsize {
        println!("Stack size full");
    } else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn stack_size(stack: &Vec<String>) -> usize {
    stack.len()
}

fn input() -> String {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    let n = n.trim().parse().expect("invalid input");
    n
}

fn individual_symbols(input_expr: String) -> Vec<String> {
    // let mut tokenized_input:Vec<String> = Vec::new();
}

fn main() {
    let input_expr = String::from("(33+45/3*(2+9)-50)");
    println!("The original expr is {:?}", input_expr);
}
