// Entry point
fn main() {
    // Section 1: Basic Printing and Variables
    basic_demo();

    // Section 2: Float Demonstration
    float_demo();

    // Section 3: Boolean and Comparison
    boolean_demo();

    // Section 4: Character Demonstration
    char_demo();

    // Section 5: Constants
    constants_demo();

    // Section 6: String Manipulation
    string_demo();

    // Section 7: Vector (Array) Demonstration
    vector_demo();

    // Section 8: Struct Demonstration
    struct_demo();

    // Section 9: String Referencing
    string_reference_demo();

    // Section 10: Arithmetic Operations
    arithmetic_demo();
}

// Function Groups
fn basic_demo() {
    println!("Hello, world!");
    println!("\n");

    let mut a = 100;
    a = 110;

    let b: i32 = 200;
    let c: i32 = 300;

    println!("The number is: {}, {}, {}", a, b, c);
    println!("\n");
}

fn float_demo() {
    let f: f64 = 3.14;
    println!("The float number is: {}", f);
    println!("\n");
}

fn boolean_demo() {
    let is_active: bool = true;
    let is_finished: bool = false;

    // Uncomment to print boolean variables
    // println!("Is active? {}", is_active);
    // println!("Is finished? {}", is_finished);

    let comparison: bool = 10 > 5;
    println!("Is 10 greater than 5? {}", comparison);
    println!("\n");
}

fn char_demo() {
    let ch: char = 'R';
    println!("Character is: {}", ch);
    println!("\n");
}

fn constants_demo() {
    const X: i32 = 100;
    println!("The constant number is: {}", X);
    println!("\n");
}

fn string_demo() {
    let mut name: String = String::from("Ruchit");
    let name2: String = "Ruchit".to_string();

    println!("Name: {}, Name2: {}", name, name2);
    println!("\n");
}

fn vector_demo() {
    let mut vec: Vec<u64> = Vec::new();
    vec.push(10);
    vec.push(15);

    let length_vec: usize = vec.len();
    println!("Vector/Array is: {:?} and its length is: {}", vec, length_vec);
    println!("\n");
}

fn struct_demo() {
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let my_rectangle = Rectangle {
        length: 10,
        width: 20,
    };

    println!(
        "Length and Width of my rectangle is: {} and {}",
        my_rectangle.length, my_rectangle.width
    );
    println!("\n");
}

fn string_reference_demo() {
    let my_string: String = "Ruchit".to_string();
    let other_string: &String = &my_string;

    println!("My String: {}", my_string);
    println!("Other String: {}", other_string);
    println!("\n");
}

fn arithmetic_demo() {
    let sum_result: u32 = sum(10, 20);
    println!("Sum of two numbers is: {}", sum_result);

    let sub_result: u32 = sub(20, 10);
    println!("Subtraction of two numbers is: {}", sub_result);

    let mul_result: u32 = mul(2, 2);
    println!("Multiplication of two numbers is: {}", mul_result);

    let div_result: u32 = div(10, 3);
    println!("Division of two numbers is: {:.2}", div_result);

    let modulo_result: u32 = modulo(10, 3);
    println!("Modulo of two numbers is: {}", modulo_result);
    println!("\n");
}

// Arithmetic Operations
fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn mul(a: u32, b: u32) -> u32 {
    a * b
}

fn div(a: u32, b: u32) -> u32 {
    a / b
}

fn modulo(a: u32, b: u32) -> u32 {
    a % b
}
