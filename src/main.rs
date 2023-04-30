//! Sieve of Eratosthenes Program
//!
//! This program uses the famous **Sieve of Eratosthenes** to discover prime numbers.
//!
//! The rules are as follows
//! - Prepare an array [2, 3, 4, ..... n] // n is input value
//! - (1) Choose the first non-zero number, and add it to the list of prime numbers
//! - (2) Replace all multiples of the chosen number with zeros.
//! - (3) Repeat (1) until the square root of the input value is less than the number chosen in (1).
//! - (4) Add all remaining non-zero numbers to the prime number list

mod numable;

use numable::Numable;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
struct Argument {
    /// Upper limit of integers to which Sieve of Eratosthenes applies
    #[structopt(short, long)]
    number: u64,
}

fn main() {
    let arg = Argument::from_args();
    println!("Prime Number {:?}", prime_serch(arg.number));
}

/// Prime Search Function
///
/// Finds prime numbers up to the input value. Only `Numable` data types are allowed as input values.
///
/// ## Example
/// ```
/// let prime: Vec<usize> = [2, 3, 5, 7, 11, 13, 17, 19]
///
/// assert_eq!(prime_serch(20), prime);
/// ```
fn prime_serch<N>(number: N) -> Vec<usize>
where
    N: Numable,
{
    let number = number.to_num();
    // Base candidate number for prime
    let mut candidate = Vec::<usize>::new();
    for i in 2..(number + 1) {
        candidate.push(i);
    }

    // Prime vector
    let mut primes = Vec::<usize>::new();

    // Limit
    let lim = (number as f64).sqrt().floor() as usize;

    // Start
    let mut prime = 2_usize;
    loop {
        // Add
        primes.push(prime);

        // Sieve
        sieve(prime, &mut candidate);

        // Update next candidate prime
        prime = *candidate.iter().find(|x| **x != 0).unwrap();

        if prime > lim {
            break;
        }
    }

    let rest_prime = candidate
        .iter()
        .filter(|x| **x != 0)
        .collect::<Vec<&usize>>();

    for prime in rest_prime {
        primes.push(*prime);
    }

    primes
}

/// No Prime Detection
///
/// Screens out through the candidate prime numbers by the specific prime number.
///
/// ## Example
/// ```
/// let prime = 3;
/// let candidate: Vec<usize> = [0, 3, 0, 5, 0, 7, 0, 9, 0]; // After already being Screensed out by 2.
///
/// let rest_candidate: Vec<usize> = [0, 0, 0, 5, 0, 7, 0, 0, 0];
///
/// assert_eq!(prime_serch(prime ,candidate), rest_candidate);
/// ```
fn sieve(prime: usize, candidate: &mut [usize]) {
    for slot in candidate.iter_mut() {
        if *slot % prime == 0 {
            *slot = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let mut candidate: Vec<usize> = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];

        sieve(2, &mut candidate);

        assert_eq!(candidate, vec![0, 3, 0, 5, 0, 7, 0, 9, 0])
    }

    #[test]
    fn test_prime_serch() {
        let primes: Vec<usize> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(prime_serch(100), primes);
    }

    #[test]
    fn test_generic() {
        let primes: Vec<usize> = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];

        assert_eq!(prime_serch(100_u8), primes);
        assert_eq!(prime_serch(100_u16), primes);
        assert_eq!(prime_serch(100_u32), primes);
        assert_eq!(prime_serch(100_u64), primes);
        assert_eq!(prime_serch(100_u128), primes);
        assert_eq!(prime_serch(100_i8), primes);
        assert_eq!(prime_serch(100_i16), primes);
        assert_eq!(prime_serch(100_i32), primes);
        assert_eq!(prime_serch(100_i64), primes);
        assert_eq!(prime_serch(100_i128), primes);
    }
}
