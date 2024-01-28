use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;
use std::{env, process, thread};
use std::fs::OpenOptions;
use std::thread::Thread;

/// Determines whether a number is prime. This function is taken from CS 110 factor.py.
///
/// You don't need to read or understand this code.
#[allow(dead_code)]
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for factor in 2..((num as f64).sqrt().floor() as u32) {
        if num % factor == 0 {
            return false;
        }
    }
    true
}

/// Determines the prime factors of a number and prints them to stdout. This function is taken
/// from CS 110 factor.py.
///
/// You don't need to read or understand this code.
#[allow(dead_code)]
fn factor_number(num: u32) {
    let start = Instant::now();

    if num == 1 || is_prime(num) {
        println!("{} = {} [time: {:?}]", num, num, start.elapsed());
        return;
    }

    let mut factors = Vec::new();
    let mut curr_num = num;
    for factor in 2..num {
        while curr_num % factor == 0 {
            factors.push(factor);
            curr_num /= factor;
        }
    }
    factors.sort();
    let factors_str = factors
        .into_iter()
        .map(|f| f.to_string())
        .collect::<Vec<String>>()
        .join(" * ");
    println!("{} = {} [time: {:?}]", num, factors_str, start.elapsed());
}

/// Returns a list of numbers supplied via argv.
#[allow(dead_code)]
fn get_input_numbers() -> VecDeque<u32> {
    let mut numbers = VecDeque::new();
    for arg in env::args().skip(1) {
        if let Ok(val) = arg.parse::<u32>() {
            numbers.push_back(val);
        } else {
            println!("{} is not a valid number", arg);
            process::exit(1);
        }
    }
    numbers
}

fn pop_from_queue(mutex: &Mutex<VecDeque<u32>>) -> Option<u32> {
    let q = mutex.lock();
    q.unwrap().pop_front()
}

fn main() {
    let num_threads = num_cpus::get();
    println!("Farm starting on {} CPUs", num_threads);
    let start = Instant::now();

    let mut threads = Vec::new();
    let numbers = get_input_numbers();
    let mutex_num = Arc::new(Mutex::new(numbers));
    for _ in 0..num_threads {
        let mutex_cloned = mutex_num.clone();
        threads.push(thread::spawn(move || {
            loop {
                if let Some(operand) = pop_from_queue(&mutex_cloned) {
                    factor_number(operand);
                }
                else { break; }
            }
        }))
    }

    for t in threads {
        t.join().expect("thread join fails");
    }

    println!("Total execution time: {:?}", start.elapsed());
}
