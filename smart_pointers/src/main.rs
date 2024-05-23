use std::cell::RefCell;

fn main() {

    // let mut x = 50;
    // let x1= &x;
    // let x2 = &x;
    // let x3 = &mut x;
    // println!("{} {}", x1, x2);
    //
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    let d = a.borrow_mut();
    println!("{}", d.unwrap());
}
