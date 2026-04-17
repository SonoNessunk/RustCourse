// Ownership, Borrowing and References

// Ownership
// C, C++ -> Memory Managment Control Issue, programmer is responsible
// thx to Garbage Collector it solved this issue, but Slow Performance -> Stopping/Resuming the program

// What is Ownership?
// Every value has a single owner [everu variable has one value, and it is its sole owner].

// Rules
// 1. Each value in Rust has a variable that's its owner.
// 2. There can be only one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    rule_one();
    // rule_two();
    // rule_three();
}

// &, like in C & C++ is a reference, so any change on s will be transmitted to string1. so the owner will still be string1
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn rule_one() {
    // string1 -> Owner
    let string1: String = String::from("RUST");
    let length = calculate_length(&string1);
    println!("Length of '{}' is {}.", string1, length);
}

// fn rule_two() {
//     let string1: String = String::from("RUST");
//     // Now the owner is moved to string2
//     let string2 = string1;
//     println!("{}", string1);
//     println!("{}", string2);
// }

// fn rule_three() {
//     let string1: String = String::from("RUST");
//     let length = calculate_length(&string1);
//     println!("Length of '{}' is {}.", string1, length);
// }

// fn print_lost(s: &String) {
//     println!("{}", &s1);
// }
