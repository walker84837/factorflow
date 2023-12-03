use structopt::StructOpt;
use anyhow::Result;

/// A Windows port of the `factor` program.
#[derive(StructOpt)]
#[structopt(name = "factor-win")]
struct Args {
    #[structopt(name = "numbers", help = "The numbers to factorize")]
    numbers: Vec<u64>,

    #[structopt(short = "h", long = "exponents", help = "Print repeated factors in form p^e unless e is 1")]
    exponents: bool,
}

fn factorize(mut n: u64, exponents: bool) -> String {
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
    let args = Args::from_args_safe()?;

    for number in args.numbers {
        let factors = factorize(number, args.exponents);
        println!("{}: {}", number, factors);
    }

    Ok(())
}