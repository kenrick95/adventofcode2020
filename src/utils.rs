use std::{fs, str::FromStr};

pub fn get_list_of_numbers_from_file<T: FromStr>(path: &str, delimiter: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fs::read_to_string(path)
        .expect("Unable to read file")
        .to_string()
        .trim()
        .split(delimiter)
        .map(|val| val.trim().parse().unwrap())
        .collect()
}
