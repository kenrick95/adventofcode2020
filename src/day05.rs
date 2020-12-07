pub fn main() {
    let inputs = super::utils::get_list_of_strings_from_file("./src/day05.in", "\n");

    let mut seat_ids = vec![];
    for input in inputs {
        let rows: Vec<char> = input
            .trim()
            .to_string()
            .chars()
            .into_iter()
            .collect::<Vec<char>>()[0..7]
            .to_vec();
        let cols: Vec<char> = input
            .trim()
            .to_string()
            .chars()
            .into_iter()
            .collect::<Vec<char>>()[7..10]
            .to_vec();
        // println!("rows {:?}", rows);
        let row;
        let col;
        {
            let mut l = 0;
            let mut r = 128;
            for ch in rows {
                let mid = (l + r) / 2;
                if ch == 'F' {
                    r = mid;
                } else if ch == 'B' {
                    l = mid;
                }
                // println!("l {}, r {}, mid {}", l, r, mid);
            }
            row = l;
        }
        {
            let mut l = 0;
            let mut r = 8;
            for ch in cols {
                let mid = (l + r) / 2;
                if ch == 'L' {
                    r = mid;
                } else if ch == 'R' {
                    l = mid;
                }
                // println!("l {}, r {}, mid {}", l, r, mid);
            }
            col = l;
        }
        let seat_id = row * 8 + col;
        // println!("row {}, col {}, seat_id {}", row, col, seat_id);
        seat_ids.push(seat_id);
    }

    {
        let answer_pt1 = seat_ids.iter().max().unwrap();
        println!("Part 1: {:?}", answer_pt1);
    }
    {
        seat_ids.sort();
        // let max_seat_id = *seat_ids.iter().max().unwrap();
        let min_seat_id = *seat_ids.iter().min().unwrap();
        let mut answer_pt2 = -1;
        {
            let mut i = 0;
            let mut i_value = min_seat_id;
            while i < seat_ids.len() {
                let val = seat_ids[i];
                if val != i_value {
                    answer_pt2 = i_value;
                    break;
                }

                i += 1;
                i_value += 1;
            }
        }

        // println!("seat_ids: {:?}", seat_ids);
        println!("Part 2: {:?}", answer_pt2); // 624 is too high
    }
}
