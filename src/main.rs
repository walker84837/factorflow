use anyhow::Result;
use clap::Parser;

/// A Rust port of the `factor` program.
#[derive(Parser)]
struct Args {
    #[arg(name = "numbers", help = "The numbers to factorize")]
    numbers: Vec<i32>,

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

fn factorize(mut n: i32, exponents: bool) -> String {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        let mut count = 0;
        while n % divisor == 0 {
            count += 1;
            n /= divisor;
        }

        if count > 0 {
            if exponents {
                if count == 1 {
                    factors.push(divisor.to_string());
                } else {
                    factors.push(format!("{}^{}", divisor, count));
                }
            } else {
                factors.extend(vec![divisor.to_string(); count as usize]);
            }
        }

        divisor += 1;
    }

    factors.join(" ")
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.lcm_gcd {
        for number in args.numbers {
            let factors = factorize(number, args.exponents);
            println!("{}: {}", number, factors);
        }
    } else {
        let mut result_lcm = args.numbers[0];
        let mut result_gcd = result_lcm;

        for number in args.numbers {
            result_lcm = lcm(result_lcm, number);
            result_gcd = gcd(result_gcd, number);
        }
        println!("LCM: {result_lcm}");
        println!("GCD: {result_gcd}");
    }

    Ok(())
}
