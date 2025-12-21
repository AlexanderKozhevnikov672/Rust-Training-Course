// This chapter is dedicated to the common programming concepts, like variables and their
// mutability, data types, functions and control flow stuff

// MUTABILITY
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function that declares a mutable integer variable, assigns it the value 5, then changes
// it to 10, and prints both values.
#[allow(dead_code)]
pub fn simple_mutability() {
    let mut number: i32 = 5;
    println!("{number}");

    number = 10;
    println!("{number}");
}

// DATA TYPES
// ================================================================================================

// ----- 2 --------------------------------------
// Create variables of types `i32``, `f64``, `bool``, and `char``, assign them values, and print
// them.
#[allow(dead_code)]
pub fn simple_data_types() {
    let i32_var: i32 = 1;
    let f64_var: f64 = 1.0;
    let bool_var: bool = true;
    let char_var: char = '1';

    println!("i32: {i32_var}\nf64: {f64_var}\nbool: {bool_var}\nchar_var: {char_var}")
}

// FUNCTIONS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `square` that takes a `u32` integer and returns its square as `u32`.

// IMPLEMENT HERE:
pub fn square(number: u32) -> u32 {
    number * number
}

// ----- 4 --------------------------------------
// Write a recursive function `factorial` that computes the factorial of a number (n!) as `u32`.

// IMPLEMENT HERE:
pub fn factorial(number: u32) -> u32 {
    if number == 0 { 1 } else { number * factorial(number - 1) }
}

// CONTROL FLOW
// ================================================================================================

// ----- 5 --------------------------------------
// Write a program that prints whether a provided signed integer number is positive, negative, or
// zero using `if` statement.
pub fn sign_checker(number: i32) -> &'static str {
    if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    }
}

// ----- 6 --------------------------------------
// Write a program that finds the largest number in an array of 5 integers using a for or while
// loop.
pub fn find_biggest_number(some_array: [u32; 5]) -> u32 {
    let mut max_value: u32 = 0;
    for value in some_array {
        if value > max_value {
            max_value = value;
        }
    }

    max_value
}
