/// Formats the prime factors of `numbers` into a string.
/// If `exponents` is true, the factors are printed in form p^e unless e is 1.
/// If `exponents` is false, the factors are printed in form p.
/// Example:
/// ```
/// assert_eq!(format_factors(vec![2, 2, 3, 3, 5], true), "2^2 3^2 5");
/// ```
pub fn format_factors(numbers: Vec<i32>, exponents: bool) -> String {
    if exponents {
        format_factors_with_exponents(numbers)
    } else {
        format_factors_without_exponents(numbers)
    }
}

fn format_factors_with_exponents(numbers: Vec<i32>) -> String {
    let mut factors = numbers;
    factors.sort();
    let mut result = String::new();
    let mut current_factor = factors[0];
    let mut count = 1;
    for &factor in &factors[1..] {
        if factor == current_factor {
            count += 1;
        } else {
            append_factor_to_result(&mut result, current_factor, count);
            current_factor = factor;
            count = 1;
        }
    }
    append_factor_to_result(&mut result, current_factor, count);
    result.trim().to_string()
}

fn format_factors_without_exponents(numbers: Vec<i32>) -> String {
    numbers
        .iter()
        .map(|&f| f.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn append_factor_to_result(result: &mut String, factor: i32, count: i32) {
    if count == 1 {
        result.push_str(&format!("{} ", factor));
    } else {
        result.push_str(&format!("{}^{} ", factor, count));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_factors() {
        let test_cases = vec![
            (vec![2, 2, 3, 3, 5], true, "2^2 3^2 5"),
            (vec![2, 2, 3, 3, 5], false, "2 2 3 3 5"),
            (vec![2, 2, 2, 2, 2], true, "2^5"),
            (vec![2, 2, 2, 2, 2], false, "2 2 2 2 2"),
            (vec![], true, ""),
            (vec![], false, ""),
            (vec![2], true, "2"),
            (vec![2], false, "2"),
            (vec![-2, -2, -3, -3, -5], true, "-2^2 -3^2 -5"),
            (vec![-2, -2, -3, -3, -5], false, "-2 -2 -3 -3 -5"),
        ];

        for (nums, use_exp, expected) in test_cases {
            assert_eq!(format_factors(nums, use_exp), expected);
        }
    }

    #[test]
    #[should_panic]
    fn test_fmt_factors_invalid_input() {
        let nums = vec![0, 0, 0, 0, 0];
        let expected = "2^2 3^2 5";
        assert_eq!(format_factors(nums, true), expected);
    }
}
