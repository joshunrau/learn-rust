// If we need to store multiple types in a vector, use an enum
enum MyEnum {
    Int(i32),
    Float(f64)
}

fn vectors() {
    let mut v1: Vec<i32> = Vec::new(); // create new empty vector
    let v2 = vec![1, 2, 3]; // infer type by initial data

    v1.push(4);

    let n1 = &v1[0]; // will panic if out of range
    println!("The first number is {n1}");
    
    // After this, n1 will be invalidated
    // If the value needs to be moved to a new place in memory, n1 cannot point to the old one
    v1.push(5);
    
    // Iterate over items in the vector
    for i in &mut v1 {
        *i += 1;
        println!("{i}");
    }

    // If the item might be out of range
    let n2 = v2.get(1);
    match n2 {
        Some(n2) => println!("The second number is {n2}"),
        None => println!("There is no second number"),
    }

    // Storing multiple types in the same vector
    let _ = vec![MyEnum::Int(1), MyEnum::Float(1.5)];

    
}

pub fn main() {
    vectors();
}
