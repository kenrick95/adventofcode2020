use regex::Regex;
use std::collections::HashMap;
pub fn main() {
    let raw_inputs = super::utils::get_list_of_strings_from_file("./src/day04.in", "\n");
    let mut raw_ids: Vec<String> = vec![];
    let mut passports: Vec<HashMap<String, String>> = vec![];

    // Process `raw_input` into one `id`
    {
        let mut i = 0;
        let mut temp: Vec<String> = vec![];
        while i < raw_inputs.len() {
            let line = raw_inputs[i].clone().trim().to_string();
            // println!("line '{}'", line);
            if line == "" {
                // Empty line, push all temps to `ids`
                raw_ids.push(temp.join(" "));
                temp = vec![];
                i += 1;
                continue;
            }
            temp.push(line.to_string());
            i += 1;
        }
        if temp.len() > 0 {
            raw_ids.push(temp.join(" "));
        }
    }

    let required_fields: Vec<String> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    // let optional_fields: Vec<String> = vec!["cid"].iter().map(|s| s.to_string()).collect();

    // Process each `id`
    for id in raw_ids {
        // println!("id: {:?}", id);
        let mut temp_id: HashMap<String, String> = HashMap::new();
        let props: Vec<String> = id.split(" ").map(|s| s.to_string()).collect();
        for prop in props {
            let kv: Vec<String> = prop.split(":").map(|s| s.to_string()).collect();
            // println!("kv: {:?}", kv);
            let key = kv[0].clone();
            let value = kv[1].clone();
            temp_id.insert(key, value);
        }

        let mut is_id_valid_as_passport = true;
        for field in &required_fields {
            if temp_id.get(field).is_none() {
                // println!("field: {:?}", field);
                is_id_valid_as_passport = false;
                break;
            }
        }

        if is_id_valid_as_passport {
            passports.push(temp_id);
        }
    }

    println!("Part 1: {}", passports.len());

    /*

     byr (Birth Year) - four digits; at least 1920 and at most 2002.
     iyr (Issue Year) - four digits; at least 2010 and at most 2020.
     eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
     hgt (Height) - a number followed by either cm or in:
         If cm, the number must be at least 150 and at most 193.
         If in, the number must be at least 59 and at most 76.
     hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
     ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
     pid (Passport ID) - a nine-digit number, including leading zeroes.
     cid (Country ID) - ignored, missing or not.

    */
    let mut valid_passports: Vec<HashMap<String, String>> = vec![];
    for passport in passports {
        if is_valid_passport(&passport) {
            valid_passports.push(passport);
        }
    }
    println!("Part 2: {}", valid_passports.len()); // 179 too low; 195 too high
}

fn is_valid_passport(passport: &HashMap<String, String>) -> bool {
   
    // println!("passport {:?}", passport);
    if !is_valid_byr(&passport) {
        // println!("invalid byr");
        return false;
    }
    if !is_valid_iyr(&passport) {
        // println!("invalid iyr");
        return false;
    }
    if !is_valid_eyr(&passport) {
        // println!("invalid eyr");
        return false;
    }
    if !is_valid_hgt(&passport) {
        // println!("invalid hgt");
        return false;
    }
    if !is_valid_hcl(&passport) {
        // println!("invalid hcl");
        return false;
    }
    if !is_valid_ecl(&passport) {
        // println!("invalid ecl");
        return false;
    }
    if !is_valid_pid(&passport) {
        // println!("invalid pid");
        return false;
    }
    // println!("passport {:?}", passport);

    true
}

fn is_valid_byr(passport: &HashMap<String, String>) -> bool {
    let byr: usize = passport.get(&"byr".to_string()).unwrap().parse().unwrap();
    byr >= 1920 && byr <= 2002
}
fn is_valid_iyr(passport: &HashMap<String, String>) -> bool {
    let iyr: usize = passport.get(&"iyr".to_string()).unwrap().parse().unwrap();
    iyr >= 2010 && iyr <= 2020
}
fn is_valid_eyr(passport: &HashMap<String, String>) -> bool {
    let eyr: usize = passport.get(&"eyr".to_string()).unwrap().parse().unwrap();
    eyr >= 2020 && eyr <= 2030
}
fn is_valid_hgt(passport: &HashMap<String, String>) -> bool {
    let hgt: String = passport.get(&"hgt".to_string()).unwrap().clone();

    let x: &[_] = &['c', 'm', 'i', 'n'];
    let numeric_hgt: usize = hgt.trim_end_matches(x).parse().unwrap();
    if hgt.ends_with("cm") {
        numeric_hgt >= 150 && numeric_hgt <= 193
    } else if hgt.ends_with("in") {
        numeric_hgt >= 59 && numeric_hgt <= 76
    } else {
        false
    }
}
fn is_valid_hcl(passport: &HashMap<String, String>) -> bool {
    let hcl: String = passport.get(&"hcl".to_string()).unwrap().clone();
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(&hcl)
}

fn is_valid_ecl(passport: &HashMap<String, String>) -> bool {
    let ecl: String = passport.get(&"ecl".to_string()).unwrap().clone();
    let valid_ecl: Vec<String> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let found = valid_ecl.iter().find(|s| ecl == **s).is_some();
    found
}
fn is_valid_pid(passport: &HashMap<String, String>) -> bool {
    let pid: String = passport.get(&"pid".to_string()).unwrap().clone();
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(&pid)
}
