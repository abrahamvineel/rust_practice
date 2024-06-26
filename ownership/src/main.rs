fn heap_fn(var1: &Vec<i32>) {
    println!("var: {:?}", var1);
}

fn main() {
    let vec1 = vec![5,6,7,8];
    let vec2 = vec1.clone();
    println!("{:?} {:?}", vec1, vec2);

    let heap_vec: Vec<i32> = vec![10, 20, 30];

    heap_fn(&heap_vec);
    println!("values in heap {:?}", heap_vec);
}
