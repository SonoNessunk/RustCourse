// Functions

// an function / variable should be written in snake case

// Entry point
fn main() {
    hello_world();
    tell_height(182);
    human_id("Maxim", 17, 182.2);

    let x: i32 = {
        let price: i32 = 5;
        let quantity: i32 = 10;
        price * quantity
    };

    println!("Result is: {}", x);

    let y: i32 = add(4, 6);
    println!("Value of y: {}", y);
    println!("Value of fn: {}", add(3, 4));

    let weight: f64 = 80.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
}

// Hoisiting: can call function anywhere in your code
fn hello_world() {
    println!("Hello, Rust!");
}

// can insert input values
fn tell_height(height: u32) {
    println!("My height is: {} cm.", height);
}

// more than one input values
fn human_id(name: &str, age: u32, height: f64) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cmd.",
        name, age, height
    )
}

// functions returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

// Expressions and Statements
// Expressions: Anything that returns a value.
// Statements: Anything that does not return a value.

// Expressions
// 5
// true & false
// add(3,5)
// if condition {value1} else {value2}
// ({code})

// Statement
// let y = let x = 10;
// 1. Variable declaration: let x = 5;
// 2. Function definitions: fn add() {}
// 3. Control flow statements: if condition { code } else { code }, while conditions { code }, etc.
