// This chapter is dedicated to the error handling, tests and documentation.

// RESULT
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `first_char(text: &str) -> Result<char, String>` that returns the first
// character of a string or an error message "Empty string" if the string is empty.

pub fn first_char(text: &str) -> Result<char, String> {
    match text.chars().next() {
        Some(c) => Ok(c),
        None => Err("Empty string".to_string()),
    }
}

// ----- 2 --------------------------------------
// Write a function `read_numbers_from_str(line: &str) -> Result<Vec<i32>, String>` that reads a
// line of integers separated by whitespace and parses each integer as i32. In case the value cannot
// be parsed (if it is not an integer) return the `Err("Invalid number")` result.

pub fn read_numbers_from_str(line: &str) -> Result<Vec<i32>, String> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|_| "Invalid number".to_string()))
        .collect()
}

// OPTION
// ================================================================================================

// ----- 3 --------------------------------------
// You have a struct `UserProfile` with fields `username: String` and `email: Option<String>`.
//
// Implement a method `get_email_domain(&self) -> Option<String>` that:
// - If the email exists, extracts the domain (the part after @).
// - If the email is missing, returns `None`.

// IMPLEMENT HERE:
pub struct UserProfile {
    #[allow(dead_code)]
    username: String,
    email: Option<String>,
}

impl UserProfile {
    pub fn new(username: String, email: Option<String>) -> Self {
        UserProfile { username, email }
    }

    pub fn get_email_domain(&self) -> Option<String> {
        match &self.email {
            Some(email) => email.split('@').nth(1).map(|s| s.to_string()),
            None => None,
        }
    }
}

// WRITING TESTS
// ================================================================================================

// ----- 4 --------------------------------------
// Write unit tests for the `factorial(n: u32) -> u64` function.
#[allow(dead_code)]
fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod factorial_tests {
    use super::factorial;

    const TEST_CASES: &[(u32, u64)] = &[
        (0, 1),
        (1, 1),
        (2, 2),
        (3, 6),
        (4, 24),
        (5, 120),
        (6, 720),
        (10, 3628800),
        (12, 479001600),
        (20, 2432902008176640000),
    ];

    #[test]
    fn test_all_cases() {
        for &(input, expected) in TEST_CASES {
            assert_eq!(factorial(input), expected, "Failed for input: {input}");
        }
    }
}

// ----- 5 --------------------------------------
// Write unit tests for the `is_prime(n: u64) -> bool` function checking both prime and non-prime
// numbers.
#[allow(dead_code)]
fn is_prime(number: u64) -> bool {
    if number < 2 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod prime_tests {
    use super::is_prime;

    const TEST_CASES: &[(u64, bool)] = &[
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (6, false),
        (11, true),
        (15, false),
        (17, true),
    ];

    #[test]
    fn test_all_cases() {
        for &(input, expected) in TEST_CASES {
            assert_eq!(is_prime(input), expected, "Failed for input: {input}");
        }
    }
}

// WRITING DOCS
// ================================================================================================

// ----- 6 --------------------------------------
// You have an implemented `TemperatureLog` struct below, which stores a city name and a list of
// daily temperature readings. This struct have a constructor, an `add_reading` method which just
// ads a new value to the `readings` vector and an `average` method which returns an average value
// of the readings of there are some.
//
// Your task is to add doc comments:
// - High-level purpose of the struct.
// - Inline docs for each field and method.
//
// In case you want something more than хор(5):
// - Additionally white the usage example for the `TemperatureLog` in the high-level docs.
// - For the `average` method additionally write an example of its usage.

/// A struct for tracking daily temperature readings for a specific city.
///
/// The `TemperatureLog` maintains a collection of temperature readings (in degrees)
/// and provides methods to add new readings and calculate average temperature.
///
/// # Examples
///
/// ```
/// use rust_learning_course::TemperatureLog;
///
/// let mut log = TemperatureLog::new("Moscow");
/// log.add_reading(1.1);
/// log.add_reading(2.2);
/// log.add_reading(-0.3);
///
/// assert_eq!(log.city, "Moscow");
/// assert_eq!(log.readings.len(), 3);
///
/// if let Some(avg) = log.average() {
///     println!("Average temperature: {:.1} degrees", avg);
/// }
/// ```
#[allow(dead_code)]
pub struct TemperatureLog {
    /// The name of the city being monitored
    pub city: String,
    /// Collection of temperature readings in degrees
    ///
    /// Readings are stored in chronological order as they are added
    pub readings: Vec<f64>,
}

#[allow(dead_code)]
impl TemperatureLog {
    /// Creates a new `TemperatureLog` for the specified city
    ///
    /// # Arguments
    ///
    /// * `city` - The name of the city to monitor
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_learning_course::TemperatureLog;
    ///
    /// let log = TemperatureLog::new("Moscow");
    ///
    /// assert_eq!(log.city, "Moscow");
    /// assert!(log.readings.is_empty());
    /// ```
    pub fn new(city: &str) -> Self {
        Self {
            city: city.to_string(),
            readings: Vec::new(),
        }
    }

    /// Adds a new temperature reading to the log
    ///
    /// # Arguments
    ///
    /// * `value` - The temperature reading to add (in degrees)
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_learning_course::TemperatureLog;
    ///
    /// let mut log = TemperatureLog::new("Moscow");
    ///
    /// log.add_reading(1.1);
    /// log.add_reading(2.2);
    /// log.add_reading(-0.3);
    ///
    /// assert_eq!(log.readings.len(), 3);
    /// ```
    pub fn add_reading(&mut self, value: f64) {
        self.readings.push(value);
    }

    /// Calculates the average of all temperature readings
    ///
    /// Returns `Some(average)` if there are readings, or `None` if the log is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_learning_course::TemperatureLog;
    ///
    /// let mut log = TemperatureLog::new("Moscow");
    /// assert!(log.average().is_none());
    ///
    /// log.add_reading(1.1);
    /// log.add_reading(2.2);
    /// log.add_reading(-0.3);
    ///
    /// let avg = log.average();
    /// assert!(avg.is_some());
    /// assert!((avg.unwrap_or_default() - 1.0).abs() < 1e-9);
    /// ```
    ///
    /// # Notes
    ///
    /// The average is calculated as the arithmetic mean of all readings.
    /// For empty logs, this method returns `None` to avoid division by zero.
    pub fn average(&self) -> Option<f64> {
        if self.readings.is_empty() {
            return None;
        }
        let sum_of_readings: f64 = self.readings.iter().sum();
        Some(sum_of_readings / self.readings.len() as f64)
    }
}
