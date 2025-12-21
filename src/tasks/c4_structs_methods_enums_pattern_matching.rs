// This chapter is dedicated to the structs, methods, enums, and pattern matching

// STRUCTS
// ================================================================================================

// ----- 1 --------------------------------------
// Define a struct `Point` with fields `x` and `y` (both `u32`). Create a function `new_point(x, y)`
// that returns a `Point` instance.

// IMPLEMENT HERE:
struct Point {
    x: u32,
    y: u32,
}

fn new_point(x: u32, y: u32) -> Point {
    Point { x, y }
}

// uncomment once implemented
pub fn point_checker() {
    let point = new_point(3, 4);
    assert_eq!((3, 4), (point.x, point.y));
}

// ----- 2 --------------------------------------
// Define a struct `Rectangle` with width and height. Implement a function
// `can_hold(r1: &Rectangle, r2: &Rectangle) -> bool` that returns true if `r1` can completely
// contain `r2`.

// IMPLEMENT HERE:
struct Rectangle {
    width: u32,
    height: u32,
}

fn can_hold(r1: &Rectangle, r2: &Rectangle) -> bool {
    r1.width >= r2.width && r1.height >= r2.height
}

// uncomment once implemented
pub fn rectangle_checker() {
    let big = Rectangle { width: 10, height: 8 };
    let small = Rectangle { width: 5, height: 4 };

    assert!(can_hold(&big, &small));
    assert!(!can_hold(&small, &big));
}

// METHODS
// ================================================================================================

// ----- 3 --------------------------------------
// Create a `Company` struct with `name: String`, `date_of_origin: u32` and `annual_income: u64`
// fields. Implement `new(...) -> Self` constructor for it and a `total_income(...) -> u64`
// method that would calculate how much money this company earned since it was established
// (excluding taxes).

// IMPLEMENT HERE:
pub struct Company {
    #[allow(dead_code)] // for name field
    name: String,
    date_of_origin: u32,
    annual_income: u64,
}

impl Company {
    pub fn new(name: String, date_of_origin: u32, annual_income: u64) -> Company {
        Company { name, date_of_origin, annual_income }
    }

    pub fn total_income(&self) -> u64 {
        u64::from(2025 - self.date_of_origin) * self.annual_income
    }
}

// ----- 4 --------------------------------------
// Define a struct BankAccount with `owner: String` and `balance: u64` fields.
// Implement basic `new(...) -> Self` constructor.
// Implement methods:
// - `deposit(&mut self, amount: u64)` which adds the specified amount to the balance.
// - `withdraw(&mut self, amount: u64) -> bool` which removes the specified amount from the balance
//   and returns `true`, or just returns `false` if there are insufficient funds.
// - `balance(&self) -> u64` which returns the current balance.

// IMPLEMENT HERE:
pub struct BankAccount {
    #[allow(dead_code)] // for owner field
    owner: String,
    balance: u64,
}

impl BankAccount {
    pub fn new(owner: String, balance: u64) -> BankAccount {
        BankAccount { owner, balance }
    }

    pub fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: u64) -> bool {
        if self.balance < amount {
            false
        } else {
            self.balance -= amount;
            true
        }
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}

// ENUMS
// ================================================================================================

// ----- 5 --------------------------------------
// Define an enum `TrafficLight` with variants `Red`, `Yellow`, and `Green`. Implement a
// `next(light: &TrafficLight) -> TrafficLight` method for it that returns the next light in
// sequence.

// IMPLEMENT HERE:
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

// ----- 6 --------------------------------------
// Define an enum `Operation` with variants `Add(i32, i32)`, `Subtract(i32, i32)`,
// `Multiply(i32, i32)` and `Divide(i32, i32)`. Implement a method
// `apply(self) -> Option<i32>` for it that computes the result and returns `None` if
// dividing by zero (you can use `match` for convenience)

// IMPLEMENT HERE:
pub enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

impl Operation {
    pub fn apply(self) -> Option<i32> {
        match self {
            Operation::Add(x, y) => Some(x + y),
            Operation::Subtract(x, y) => Some(x - y),
            Operation::Multiply(x, y) => Some(x * y),
            Operation::Divide(x, y) => {
                if y == 0 {
                    None
                } else {
                    Some(x / y)
                }
            },
        }
    }
}

// PATTERN MATCHING
// ================================================================================================

// ----- 7 --------------------------------------
// Write a enum `WeirdLengthMeasures`, containing `Inch`, `Foot`, `Yard` and `Mile` variants.
// Implement a `convert_to_human_format(&self) -> f64` method, which returns the length of of the
// provided weirdo lengths in meters using pattern matching (with `match`).
// Use provided values:
// - Inch -> 0.0254 m
// - Foot -> 0.3048 m
// - Yard -> 0.9144 m
// - Mile -> 1609.344 m

// IMPLEMENT HERE:
pub enum WeirdLengthMeasures {
    Inch,
    Foot,
    Yard,
    Mile,
}

impl WeirdLengthMeasures {
    pub fn convert_to_human_format(&self) -> f64 {
        match self {
            WeirdLengthMeasures::Inch => 0.0254,
            WeirdLengthMeasures::Foot => 0.3048,
            WeirdLengthMeasures::Yard => 0.9144,
            WeirdLengthMeasures::Mile => 1609.344,
        }
    }
}

// ----- 8 --------------------------------------
// Write a function `fizzbuzz(n: u32) -> Vec<String>` that returns a vector of strings from 1 to n
// where:
// - Multiples of 2 are "Fizz",
// - Multiples of 3 are "Buzz",
// - Multiples of both 2 and 3 are "FizzBuzz",
// - Otherwise the number itself.

pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut v: Vec<String> = vec!["".to_string(); n as usize];
    for (i, s) in v.iter_mut().enumerate() {
        if (i + 1) % 2 == 0 {
            s.push_str("Fizz");
        }
        if (i + 1) % 3 == 0 {
            s.push_str("Buzz");
        }
        if s.is_empty() {
            *s = format!("{}", i + 1);
        }
    }
    v
}
