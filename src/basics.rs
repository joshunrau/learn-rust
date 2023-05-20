use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid guess '{}', must be integer", guess.trim());
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn variables() {
    // Variables are immutable unless otherwise declared
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants
    // Constants are always immutable and must have an explicit type annotation
    // constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn scalar_types() {
    // A scalar type represents a single value. Rust has four primary scalar types:
    // 1) Integers - default is 32 bit integer
    // 2) Floats - default is 64 bit float
    // 3) Booleans
    // 4) Characters

    // Integers
    let my_int: i32 = 2_147_483_647;
    // let my_big_int: i32 = my_int + 1;

    println!("My integer is {my_int}");

    // Floats
    let my_float: f64 = 2.0;
    {
        let my_int: f64 = my_int.into();
        println!("My integer is {my_int}");

        let result = my_int / my_float;
        println!("Result: {result}");
    }

    // Booleans - one byte
    let t = true;
    let f = false;

    println!("{t}, {f}");

    // Characters - four bytes
    let my_char = '😻'; // use single quotes
    println!("{my_char}");
}

fn compound_types() -> () {
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuple - fixed length, may be mixed types
    let my_tuple = (1, 1.0, "Foo");
    let (x, y, z) = my_tuple;
    println!("The values are: {x}, {y}, {z}");
    println!("The value at index 0 is {}", my_tuple.0);

    // The tuple without any values has a special name, unit.
    // This value and its corresponding type are both written () and
    // represent an empty value or an empty return type.

    // Array - every element of an array must have the same type
    // Arrays in Rust have a fixed length.
    let my_array: [i32; 3] = [1, 2, 3];
    println!("The value at index 1 is {}", my_array[0]);
    // Arrays are useful when you want your data allocated on the stack rather than the heap
    // If you need your list to grow in size, you should use a vector

    // You can also initialize an array to contain the same value for each element
    let a: [i32; 2] = [3; 2]; // [value, length] -> [3, 3]
    println!("{}", a[1]);

    println!("Please enter an array index.");

    let mut index = String::new();
    stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("You selected: {element}");
}

fn functions() {
    foo(); // Function can be invoked even if declared later
    println!("2 + 3 = {}", add(2, 3));

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    fn foo() {
        println!("Foo");
    }

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn conditionals() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 }; // both branches must be the same type

    println!("The value of number is: {number}");
}

fn inf_loop() {
    // Infinite loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 20

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    // Over array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn control_flow() {
    conditionals();
    inf_loop();
    while_loop();
    for_loop();
}

pub fn main() {
    control_flow()
}
