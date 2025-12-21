// This chapter is dedicated to the concurrency.

use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// THREADS & JOIN
// ================================================================================================

// ----- 1 --------------------------------------
// Spawn multiple threads to calculate squares of the provided numbers and collect the results.

pub fn calculate_squares(input_numbers: Vec<i32>) -> Vec<i32> {
    let mut children = Vec::with_capacity(input_numbers.len());
    for number in input_numbers {
        children.push(thread::spawn(move || number * number));
    }

    let mut results = Vec::with_capacity(children.len());
    for child in children {
        results.push(child.join().unwrap());
    }

    results
}

// ----- 2 --------------------------------------
// Implement a `parallel_prime_check` function that splits work across multiple threads.

fn is_prime(number: u64) -> bool {
    if number <= 1 {
        return false;
    }
    for divisor in 2..=((number as f64).sqrt() as u64) {
        if number % divisor == 0 {
            return false;
        }
    }
    true
}

/// Inputs:
/// - `numbers` - a `u64` vector of values which should be checked.
/// - `number_of_threads` - a number of threads you must use to *efficiently* distribute the values
///   from the numbers vector.
///
/// Outputs:
/// - `Vec<(u64, bool)>` is a vector of the provided values along with the boolean flag whether this
///   value is prime.
pub fn parallel_prime_check(numbers: Vec<u64>, number_of_threads: usize) -> Vec<(u64, bool)> {
    let numbers = Arc::new(numbers);

    let results: Vec<Option<(u64, bool)>> = vec![None; numbers.len()];
    let results = Arc::new(Mutex::new(results));

    let next_index = Arc::new(Mutex::new(0));

    let mut children = Vec::with_capacity(number_of_threads);

    for _ in 0..number_of_threads {
        let numbers = Arc::clone(&numbers);
        let next_index = Arc::clone(&next_index);
        let results = Arc::clone(&results);

        let child = thread::spawn(move || {
            loop {
                let current_index = {
                    let mut index_guard = next_index.lock().unwrap();
                    if *index_guard >= numbers.len() {
                        break;
                    }

                    let idx = *index_guard;
                    *index_guard += 1;
                    idx
                };

                let num = numbers[current_index];
                let is_prime_result = is_prime(num);

                let mut results_guard = results.lock().unwrap();
                results_guard[current_index] = Some((num, is_prime_result));
            }
        });

        children.push(child);
    }

    for child in children {
        child.join().unwrap();
    }

    let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
    results.into_iter().map(|opt| opt.unwrap()).collect()
}

// MPSC CHANNELS
// ================================================================================================

// ----- 3 --------------------------------------
// Compute the factorial for each value in the provided vector.
// Use a separate thread for each computation.
// Send the factorial results to the main thread using a channel transmitter.
// Using a channel receiver, collect the resulting factorial values into a vector and return it from
// the function.

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

pub fn parallel_factorials(numbers: Vec<u32>) -> Vec<u32> {
    let length = numbers.len();
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::with_capacity(length);

    for num in numbers {
        let tx = tx.clone();

        children.push(thread::spawn(move || {
            let result = factorial(num);
            tx.send(result).unwrap();
        }));
    }

    let mut results = Vec::with_capacity(length);
    for _ in 0..length {
        results.push(rx.recv().unwrap());
    }

    for child in children {
        child.join().unwrap();
    }

    results
}

// MUTEX + ARC
// ================================================================================================

// ----- 4 --------------------------------------
// Implement a `SharedCounter` struct with one `value: ?<i32>` field and methods:
// - `pub fn new(initial_value: i32) -> Self`, which creates a new instance of the `SharedCounter`.
// - `pub fn increment(&self)` which will increment the internal value.
// - `pub fn get_value(&self) -> i32` which will return the internal value.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct SharedCounter {
    value: Arc<Mutex<i32>>,
}

impl SharedCounter {
    pub fn new(initial_value: i32) -> Self {
        SharedCounter {
            value: Arc::new(Mutex::new(initial_value)),
        }
    }

    pub fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    pub fn get_value(&self) -> i32 {
        let value = self.value.lock().unwrap();
        *value
    }
}

// ----- 5 --------------------------------------
// Simulate a bank account system with concurrent deposits and withdrawals.
//
// Implement a `BankAccount` struct with one `balance: ?<i32>` field and methods:
// - `pub fn new(initial_balance: i32) -> Self`, which creates a new instance of the `BankAccount`.
// - `pub fn deposit(&self, amount: i32)` which adds the provided amount to the balance.
// - `pub fn withdraw(&self, amount: i32) -> bool` which attempts to remove the provided amount from
//   the balance. If the balance have sufficient funds, it removes the provided amount and returns
//   `true`, otherwise returns `false`.
// - `pub fn get_balance(&self)` which returns the current balance.
//
// Notice that these methods could be called from the several threads at the same time. Use `Arc`
// and `Mutex` where needed.

#[derive(Clone)]
pub struct BankAccount {
    balance: Arc<Mutex<i32>>,
}

impl BankAccount {
    pub fn new(initial_balance: i32) -> Self {
        BankAccount {
            balance: Arc::new(Mutex::new(initial_balance)),
        }
    }

    pub fn deposit(&self, amount: i32) {
        let mut balance = self.balance.lock().unwrap();
        *balance += amount;
    }

    pub fn withdraw(&self, amount: i32) -> bool {
        let mut balance = self.balance.lock().unwrap();
        if *balance >= amount {
            *balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_balance(&self) -> i32 {
        let balance = self.balance.lock().unwrap();
        *balance
    }
}

// FINAL BOSS: CHANNELS + MUTEX + ARC
// ================================================================================================

// ----- 6 --------------------------------------
// Implement a work queue where multiple workers consume tasks and send results back (a simple task
// distribution system).
//
// You will need to implement two procedures:
// - `worker(id: usize, task_receiver: ?<Receiver<i32>>, result_sender: ?<Sender<(usize, i32)>>)`,
//   which has the ID of the worker, the task receiver, which waits for the task to be provided to
//   this worker, and the result sender, which sends the computed result back to the main thread.
//   This procedure should:
//   - Loop over all incoming tasks from `task_receiver`.
//   - For each task, compute the square of the value this task provided.
//   - Send the result back via `result_sender` along with the worker’s ID:
//    `(worker_id, result_value)`.
//   - Decide by your own whether this procedure should return something or not, and if should --
//    what exactly.
//   - Use `Arc` or `Mutex` if needed.
// - `run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)>` which has the
//   vector of tasks (just values, which square we should compute) and the total number of workers
//   which should be spawned. It returns the vector of worker IDs (`usize`) and the resulting value
//   computed by this worker (`i32`). This procedure should:
//   - Create two channels: for sending tasks to workers and for collecting results from workers.
//   - For each worker spawn a thread which runs the worker function, consuming tasks and sending
//     results.
//   - Send each task from the input list into the task_sender.
//   - Collect all results from the result_receiver into a vector and return it.

fn worker(
    worker_id: usize,
    task_receiver: Arc<Mutex<Receiver<i32>>>,
    result_sender: Sender<(usize, i32)>,
) {
    loop {
        let task = {
            let receiver = task_receiver.lock().unwrap();
            match receiver.recv() {
                Ok(task) => task,
                Err(_) => break,
            }
        };

        result_sender.send((worker_id, task * task)).unwrap();
    }
}

pub fn run_work_queue(tasks: Vec<i32>, number_of_workers: usize) -> Vec<(usize, i32)> {
    let (task_sender, task_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();

    let task_receiver = Arc::new(Mutex::new(task_receiver));

    let mut children = Vec::with_capacity(number_of_workers);

    for worker_id in 0..number_of_workers {
        let task_receiver = Arc::clone(&task_receiver);
        let result_sender = result_sender.clone();

        children.push(thread::spawn(move || {
            worker(worker_id, task_receiver, result_sender);
        }));
    }

    let tasks_length = tasks.len();
    for task in tasks {
        task_sender.send(task).unwrap();
    }

    let mut results = Vec::with_capacity(tasks_length);
    for _ in 0..tasks_length {
        results.push(result_receiver.recv().unwrap());
    }

    drop(task_sender);

    for child in children {
        child.join().unwrap();
    }

    results
}
