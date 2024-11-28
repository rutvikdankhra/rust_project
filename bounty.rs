use std::collections::HashMap;
use std::thread;

// Constants
const PI: f64 = 3.14159;

fn main() {
    // Section 1: Ownership and Borrowing
    ownership_and_borrowing();

    // Section 2: Generics and Traits
    generics_and_traits();

    // Section 3: Enums and Pattern Matching
    enums_and_pattern_matching();

    // Section 4: Error Handling
    error_handling();

    // Section 5: Iterators and Closures
    iterators_and_closures();

    // Section 6: Multithreading
    multithreading_demo();

    // Section 7: Collections
    collections_demo();

    // Section 8: Macros
    macros_demo();

    // Section 9: Smart Pointers
    smart_pointers_demo();
}

// Section 1: Ownership and Borrowing
/// Demonstrates ownership and borrowing concepts in Rust.
fn ownership_and_borrowing() {
    println!("\n--- Ownership and Borrowing ---");
    let my_string = String::from("Hello, Rust!");
    let len = calculate_length(&my_string); // Borrowing
    println!("The length of '{}' is {}.", my_string, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() // Borrowing, does not take ownership
}

// Section 2: Generics and Traits
/// Demonstrates the use of generics and traits in Rust.
fn generics_and_traits() {
    println!("\n--- Generics and Traits ---");
    let point = Point { x: 5, y: 10 };
    println!("Point: ({}, {})", point.x, point.y);

    let circle = Circle { radius: 3.0 };
    println!("Circle area: {:.2}", circle.area());
}

struct Point<T> {
    x: T,
    y: T,
}

trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// Section 3: Enums and Pattern Matching
/// Demonstrates enums and pattern matching in Rust.
fn enums_and_pattern_matching() {
    println!("\n--- Enums and Pattern Matching ---");
    let coin = Coin::Penny;
    println!("Coin value: {}", value_in_cents(&coin));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Section 4: Error Handling
/// Demonstrates error handling in Rust.
fn error_handling() {
    println!("\n--- Error Handling ---");
    let filepath = "nonexistent.txt";

    match std::fs::read_to_string(filepath) {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Failed to read file: {}", e),
    }
}

// Section 5: Iterators and Closures
/// Demonstrates iterators and closures in Rust.
fn iterators_and_closures() {
    println!("\n--- Iterators and Closures ---");
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
}

// Section 6: Multithreading
/// Demonstrates basic multithreading in Rust.
fn multithreading_demo() {
    println!("\n--- Multithreading ---");
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread says: {}", i);
        }
    });

    handle.join().unwrap(); // Wait for thread to finish
    println!("Main thread is done.");
}

// Section 7: Collections
/// Demonstrates collections in Rust using HashMap.
fn collections_demo() {
    println!("\n--- Collections ---");
    let mut scores = HashMap::new();
    scores.insert("Team A", 10);
    scores.insert("Team B", 20);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }
}

// Section 8: Macros
/// Demonstrates creating and using macros in Rust.
macro_rules! my_macro {
    ($msg:expr) => {
        println!("Macro says: {}", $msg);
    };
}

fn macros_demo() {
    println!("\n--- Macros ---");
    my_macro!("Hello from a macro!");
}

// Section 9: Smart Pointers
/// Demonstrates smart pointers like `Box`.
fn smart_pointers_demo() {
    println!("\n--- Smart Pointers ---");
    let boxed_number = Box::new(42);
    println!("Boxed number: {}", boxed_number);
}
