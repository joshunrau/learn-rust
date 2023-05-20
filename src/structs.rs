#[derive(Debug)]
struct Person {
    first_name: String,
    age: u64,
}

fn create_person(first_name: String, age: u64) -> Person {
    Person { first_name, age }
}

// Tuple struct
struct Color(i32, i32, i32);

fn f1() {
    let mut josh = create_person(String::from("Josh"), 23);

    let bob = Person {
        first_name: String::from("Bob"),
        ..josh
    };

    josh.age += 1;

    // Last name is moved
    println!("Name: {} UNKNOWN, Age: {}", josh.first_name, josh.age);
    println!("{:#?}", bob);

    {
        let black = Color(0, 0, 0);
        println!("Color: {}, {}, {}", black.0, black.1, black.2);
    }

    dbg!(&bob);

    println!("{:#?}", bob);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn f2() {
    let rect1: Rectangle = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
