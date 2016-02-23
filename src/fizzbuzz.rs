/// Prints the numbers from 1 to a specified limit, but for multiples
/// of three prints "Fizz", for multiples of five prints "Buzz", and
/// for multiples of both three and five prints "FizzBuzz". Instead of
/// printing to a stream, this function returns a String with the
/// contents that would have been printed to the stream.
///
/// # Examples
///
/// ```
/// assert_eq!(fizz_buzz(1), "1\n");
/// assert_eq!(fizz_buzz(5), "1\n2\nFizz\n4\nBuzz");
/// ```
///
/// # Arguments
///
/// * `limit` - The limit to print the numbers to. This is an
///             inclusive bound.
///
/// # Returns
///
/// A string containing what would have been printed to the command
/// line containing all of the "Fizz", "Buzz", and "FizzBuzz" patterns
/// listed above.
fn fizz_buzz(limit: u32) -> String {
    let mut s = String::new();

    /* limit is inclusive, so add 1 to it */
    for i in 1..(limit + 1) {
        let app = match i {
            _ if (i % 3) == 0 && (i % 5) == 0 => "FizzBuzz".to_string(),
            _ if (i % 3) == 0                 => "Fizz".to_string(),
            _ if (i % 5) == 0                 => "Buzz".to_string(),
            n                                 => n.to_string()
        };

        s = s + &app + "\n";
    }

    return s;
}

#[cfg(test)]
mod tests {
    use super::fizz_buzz;

    #[test]
    fn fizz_buzz_basic_test() {
        assert_eq!(fizz_buzz(1), "1\n");
    }

    #[test]
    fn fizz_buzz_fizz_test() {
        assert_eq!(fizz_buzz(3), "1\n2\nFizz\n");
    }

    #[test]
    fn fizz_buzz_buzz_test() {
        assert_eq!(fizz_buzz(5), "1\n2\nFizz\n4\nBuzz\n");
    }

    #[test]
    fn fizz_buzz_fizz_buzz_test() {
        assert_eq!(fizz_buzz(15),
                   "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\nFizzBuzz\n");
    }
}
