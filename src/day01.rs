pub fn main() {
    let input: Vec<usize> = super::utils::get_list_of_numbers_from_file("./src/day01.in", "\n");
    let sum_target: usize = 2020;

    // Part 1
    {
        let mut answers: Vec<(usize, usize)> = vec![];
        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                if (input[i] + input[j] == sum_target) {
                    answers.push((input[i], input[j]));
                }
            }
        }

        for answer in answers {
            println!("Part 01: {}", answer.0 * answer.1);
        }
    }

    // Part 2
    // Can be optimized to O(N^2 log N) ... but lazy...
    {
        let mut answers: Vec<(usize, usize, usize)> = vec![];
        for i in 0..input.len() {
            for j in (i + 1)..input.len() {
                for k in (j + 1)..input.len() {
                    if (input[i] + input[j] + input[k] == sum_target) {
                        answers.push((input[i], input[j], input[k]));
                    }
                }
            }
        }

        for answer in answers {
            println!("Part 02: {}", answer.0 * answer.1 * answer.2);
        }
    }
}
