use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(author, about)]
pub struct Argument {
    /// Upper limit of integers to which Sieve of Eratosthenes applies
    #[structopt(short, long)]
    number: u64,
}

fn main() {
    let arg = Argument::from_args();
    println!("Prime Number {:?}", prime_serch(arg.number));
}

fn prime_serch(number: u64) -> Vec<u64> {
    // Base candidate number for prime
    let mut candidate = Vec::<u64>::new();
    for i in 2..(number + 1) {
        candidate.push(i);
    }

    // Prime vector
    let mut primes = Vec::<u64>::new();

    // Limit
    let lim = (number as f64).sqrt().floor() as u64;

    // Start
    let mut prime = 2_u64;
    loop {
        // Add
        primes.push(prime);

        // Sieve
        sieve(prime, &mut candidate);

        // Update next candidate prime
        prime = *candidate.iter()
            .find(|x| **x != 0).unwrap();

        if prime > lim {
            break;
        }
    }

    let rest_prime = candidate.iter()
        .filter(|x| **x != 0)
        .collect::<Vec<&u64>>();

    for prime in rest_prime {
        primes.push(*prime);
    }

    primes
}

fn sieve(prime: u64, candidate: &mut [u64]) {
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
        let mut candidate: Vec<u64> = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];

        sieve(2, &mut candidate);
        
        assert_eq!(candidate, vec![0, 3, 0, 5, 0, 7, 0, 9, 0])
    }

    #[test]
    fn test_prime_serch() {
        let primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

        assert_eq!(prime_serch(100), primes);
    }
}
