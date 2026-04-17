// Compound Data Types

// arrays, tuples, slices, strings (slice string)

fn main() {
    //Array
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Number array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];

    println!("Array Fruits: {:?}", fruits);
    println!("Array Fruits 1st: {}", fruits[0]);
    println!("Array Fruits 2nd: {}", fruits[1]);
    println!("Array Fruits 3rd: {}", fruits[2]);

    //Tuple
    let human: (&str, i32, bool) = ("Alice", 30, false);
    println!("Tuple Human: {:?}", human);

    let mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Tuple Mix: {:?}", mix_tuple);

    // Slices
    // [1,2,3,4,5]
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Slice number: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("Slice animal: {:?}", animal_slices);

    let book_slice: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Slice book: {:?}", book_slice);

    // String vs String Slices (&str)
    // String: Growable, mutable, owned string type, heap
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says: {}", stone_cold);

    // String Slices: Ungrowable, unmutable, not owned string type, stack
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("String Slices: {}", slice);
}
