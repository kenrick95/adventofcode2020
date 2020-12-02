use regex::Regex;

use std::collections::HashMap;

pub fn main() {
    let lines = super::utils::get_list_of_strings_from_file("./src/day02.in", "\n");
    let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<ch>[a-z]{1}): (?P<test>[a-z]+)$").unwrap();

    let mut answer_part_1 = 0;
    let mut answer_part_2 = 0;
    // println!("{:?}", lines);
    for line in lines {
        // println!("line '{}';", line.trim());
        let trimmed_line = line.trim();
        let captures = re.captures(&trimmed_line).unwrap();
        let min: usize = captures["min"].to_string().parse().unwrap();
        let max: usize = captures["max"].to_string().parse().unwrap();
        let ch: char = captures["ch"]
            .to_string()
            .chars()
            .into_iter()
            .next()
            .unwrap();
        let test: String = captures["test"].to_string();

        let mut test_char_count: HashMap<char, usize> = HashMap::new();
        for test_ch in test.chars() {
            let ch_count = test_char_count.entry(test_ch).or_insert(0);
            *ch_count += 1;
        }

        let count = *test_char_count.get(&ch).unwrap_or(&0);
        if min <= count && count <= max {
            answer_part_1 += 1;
        }

        let test_chars: Vec<char> = test.chars().collect();
        if (min > 0 && min <= test_chars.len() && test_chars[min - 1] == ch)
            ^ (max > 0 && max <= test_chars.len() && test_chars[max - 1] == ch)
        {
            answer_part_2 += 1;
        }
    }
    println!("Part 1: {}", answer_part_1);
    println!("Part 2: {}", answer_part_2);
}
