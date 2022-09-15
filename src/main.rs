use clap::Parser;
use num_bigint::BigInt;
use num_traits::cast::FromPrimitive;

/// Calculate the factorial of a number and then output the number of zeroes it contains
#[derive(Parser)]
struct Cli {
    /// The number to calculate the factorial of
    n: i64
}

// Calculate number of zeroes in factorial
fn count_zeroes(s: String) -> usize {
    return s.matches('0').count();
}

// Calculate Factorial
fn calc_fact(n: i64) -> BigInt {
    let mut total = BigInt::from_i64(n).unwrap();
    for x in 1..n {
        total *=x;
    }
    return total;
}


fn main() {
    let args = Cli::parse();
    println!("Calculating number of zeroes in {}!", args.n);
    let fact = calc_fact(args.n);
    let num_zeroes = count_zeroes(fact.to_str_radix(10));
    println!("{}! has {} zeroes", args.n, num_zeroes);
}
