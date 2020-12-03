fn get_tree_count(map: &Vec<Vec<char>>, dy: usize, dx: usize) -> usize {
    let num_rows: usize = map.len();
    let num_columns: usize = map[0].len();
    let mut point = (0, 0);

    let mut tree_count = 0;

    let mut iteration = 0;
    while iteration < num_rows - 1 {
        let next_point = (point.0 + dy, point.1 + dx);
        // println!(
        //     "iteration: {:?}: {:?} -> {:?}",
        //     iteration, point, next_point
        // );
        if next_point.0 >= num_rows {
            break;
        }

        let next_point_char = map[next_point.0][next_point.1 % num_columns];

        if next_point_char == '#' {
            tree_count += 1;
        }

        point = next_point;

        iteration += 1;
    }

    tree_count
}

pub fn main() {
    /*
        . --> Open square
        # --> Tree
    */

    let input = super::utils::get_list_of_strings_from_file("./src/day03.in", "\n");

    // Note that map is infinite column-wise (i.e. repeating column); but limited in rows
    let map: Vec<Vec<char>> = input
        .iter()
        .map(|line| {
            let column: Vec<char> = line.trim().chars().collect();
            column
        })
        .collect();
    // println!("num_rows: {:?}", num_rows);
    // println!("num_columns: {:?}", num_columns);
    // println!("map: {:?}", map);

    let answer_part_1 = get_tree_count(&map, 1, 3);
    println!("Part 1: {}", answer_part_1);

    let deltas = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let results: Vec<usize> = deltas
        .iter()
        .map(|delta| get_tree_count(&map, delta.0, delta.1))
        .collect();
    let mut answer_part_2 = 1;
    for res in results {
        answer_part_2 *= res;
    }
    println!("Part 2: {}", answer_part_2);
}
