use anyhow::Result;
use clap::Parser;

/// A Rust port of the `factor` program.
#[derive(Parser)]
struct Args {
    #[arg(name = "numbers", help = "The numbers to factorize")]
    numbers: Vec<i32>,

    #[arg(short, help = "Print only the prime factors")]
    prime_factors: bool,

    #[arg(
        short = 'h',
        long = "exponents",
        help = "Print repeated factors in form p^e unless e is 1"
    )]
    exponents: bool,

    #[arg(short, help = "Print the LCM and GCD of the numbers")]
    lcm_gcd: bool,
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

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

    if exponents {
        factors.sort();
        let mut result = String::new();
        let mut current_factor = factors[0];
        let mut count = 1;
        for &factor in &factors[1..] {
            if factor == current_factor {
                count += 1;
            } else {
                if count == 1 {
                    result.push_str(&format!("{} ", current_factor));
                } else {
                    result.push_str(&format!("{}^{} ", current_factor, count));
                }
                current_factor = factor;
                count = 1;
            }
        }
        if count == 1 {
            result.push_str(&format!("{} ", current_factor));
        } else {
            result.push_str(&format!("{}^{} ", current_factor, count));
        }
        result
    } else {
        factors.iter().map(|&f| f.to_string()).collect::<Vec<_>>().join(" ")
    }
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
