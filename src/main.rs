use anyhow::Result;
use clap::Parser;

mod maths;
mod utilities;
use crate::{
    maths::{gcd, lcm},
    utilities::format_factors,
};

/// A Rust port of the `factor` program.
#[derive(Parser)]
struct Args {
    #[clap(name = "numbers", help = "The numbers to factorize")]
    numbers: Vec<i32>,

    #[clap(short, help = "Print only the prime factors")]
    prime_factors: bool,

    #[clap(
        short = 'h',
        long = "exponents",
        help = "Print repeated factors in form p^e unless e is 1"
    )]
    exponents: bool,

    #[clap(short, help = "Print the LCM and GCD of the numbers")]
    lcm_gcd: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.lcm_gcd {
        for number in &args.numbers {
            let factors = factorize(*number, args.exponents, args.prime_factors);
            println!("{}: {}", number, factors);
        }
    } else {
        let mut result_lcm = args.numbers[0];
        let mut result_gcd = result_lcm;

        for number in &args.numbers {
            result_lcm = lcm(result_lcm, *number);
            result_gcd = gcd(result_gcd, *number);
        }
        println!("LCM: {result_lcm}");
        println!("GCD: {result_gcd}");
    }

    Ok(())
}

/// Factorize a number into its prime factors.
/// Example:
/// ```
/// assert_eq!(factorize(12, true, false), "2^2 3");
/// ```
fn factorize(mut n: i32, exponents: bool, prime_factors: bool) -> String {
    let is_prime = |n: i32| -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    };

    let mut factors = Vec::new();
    let mut divisor: i32 = 2;

    // Check for divisibility by 2
    while n % 2 == 0 {
        n /= 2;
        factors.push(2);
    }

    // Check for divisibility by odd numbers starting from 3
    while n > 1 && divisor.pow(2) <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1; // Try the next number
        while !is_prime(divisor) {
            divisor += 1;
        }
    }

    // If the remaining number is greater than 1, it's also a factor
    if n > 1 {
        factors.push(n);
    }

    if prime_factors {
        factors.dedup();
    }

    format_factors(factors, exponents)
}
