use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::str::FromStr;

/// Extract numbers from string.
///
/// # Examples
///
/// ```
/// # use failure::Error;
/// # fn main() -> Result<(), Error> {
/// use aoc2019::input::get_numbers;
/// assert_eq!(get_numbers::<i32>("<1, 3*-4>")?, vec![1, 3, -4]);
/// # Ok(())
/// # }
/// ```
pub fn get_numbers<N: Copy + Clone + Debug + FromStr>(input: &str) -> Result<Vec<N>, Error>
where
    <N as std::str::FromStr>::Err: StdError,
{
    lazy_static! {
        static ref Number: Regex = Regex::new(r"-?\d+").unwrap();
    }

    Number
        .find_iter(input)
        .map(|m| match m.as_str().parse::<N>() {
            Ok(number) => Ok(number),
            Err(err) => {
                let message = format!(
                    "Could not convert \"{}\" from \"{}\", reason is {:?}",
                    m.as_str(),
                    input,
                    err.description()
                );
                Err(failure::err_msg(message))
            }
        })
        .collect::<Result<Vec<N>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() -> Result<(), Error> {
        assert_eq!(get_numbers::<i32>("1")?, vec![1]);
        assert_eq!(get_numbers::<i32>("-1")?, vec![-1]);
        assert_eq!(get_numbers::<i32>("23")?, vec![23]);
        assert_eq!(get_numbers::<i32>("-23")?, vec![-23]);
        assert_eq!(get_numbers::<i32>("0")?, vec![0]);

        Ok(())
    }

    #[test]
    fn test_space_separated_numbers() -> Result<(), Error> {
        assert_eq!(get_numbers::<i32>("  1 2 3  ")?, vec![1, 2, 3]);
        assert_eq!(get_numbers::<i32>("-1 32 7")?, vec![-1, 32, 7]);

        Ok(())
    }

    #[test]
    fn test_many_mixed_characters() -> Result<(), Error> {
        assert_eq!(
            get_numbers::<i32>(
                "sdaf1-asdfäö©@£$∞§|[]≈2sdafÖköoi
         3dsifguyhijksöldfmb,"
            )?,
            vec![1, 2, 3]
        );
        assert_eq!(get_numbers::<i32>("--1,--32,--7")?, vec![-1, -32, -7]);

        Ok(())
    }

    #[test]
    fn test_failure() {
        assert!(get_numbers::<u8>("1000").is_err());
        assert!(get_numbers::<usize>("-1").is_err());
    }
}
