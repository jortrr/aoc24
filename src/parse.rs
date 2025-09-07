use std::{error::Error, str::FromStr};

pub fn parse_each_word_as<T>(
    s: &str,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    s.split_whitespace()
        .map(T::from_str)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}
