use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: u32,
    ecl: String,
    pid: u64,
    cid: u64,
}

fn valid_passport_data(entries: &HashMap<String, String>) -> bool {
    let need_keys = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
    need_keys.iter().filter(|&k| !entries.contains_key(&k.to_string())).count() == 0
}

fn read_passport_data(path: &str) -> io::Result<Vec<HashMap<String, String>>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();
    let mut data_store: HashMap<String, String> = HashMap::new();  // data is on multi lines

    for line in br.lines() {
        let line = line?;
        let entries: Vec<&str> = line.split_whitespace().collect();
        if entries.len() == 0 {
            // empty line between passports
            if !valid_passport_data(&data_store) {
                eprintln!("Cannot create passport from {:?}", data_store);
            } else {
                rv.push(data_store.clone());
            }
            data_store.clear();
        } else {
            // we have data to parse
            for pair in entries {
                let key_val: Vec<&str> = pair.split(":").collect();
                data_store.insert(key_val[0].to_string(), key_val[1].to_string());
            }
        }
    }
    if !data_store.is_empty() {
        if !valid_passport_data(&data_store) {
            eprintln!("Cannot create passport from {:?}", data_store);
        } else {
            rv.push(data_store);
        }
    }
    Ok(rv)
}

fn field_to_int(entries: &HashMap<String, String>, field: &str) -> Option<u32> {
    let value = &entries[field];
    match value.parse() {
        Ok(v) => Some(v),
        Err(_e) => {
            eprintln!("{} invalid {}: {}", entries["pid"], field, value);
            None
        },
    }
}

impl Passport {
    fn from_hashmap(entries: &HashMap<String, String>) -> Option<Passport> {
        let pid_str = &entries["pid"];
        if pid_str.len() != 9 {
            eprintln!("Invalid pid {}", pid_str);
            return None;
        }
        let pid = match pid_str.parse() {
            Ok(v) => v,
            Err(_e) => {
                eprintln!("Invalid pid {}", pid_str);
                return None;
            }
        };

        let byr = field_to_int(&entries, &"byr")?;
        if byr < 1920 || byr > 2002 {
            eprintln!("{} invalid birth year {}", pid_str, byr);
            return None;
        }

        let iyr = field_to_int(&entries, &"iyr")?;
        if iyr < 2010 || iyr > 2020 {
            eprintln!("{} invalid issue year {}", pid_str, iyr);
            return None;
        }

        let eyr = field_to_int(&entries, &"eyr")?;
        if eyr < 2020 || eyr > 2030 {
            eprintln!("{} invalid expiration year {}", pid_str, eyr);
            return None;
        }

        let hgt_str = &entries["hgt"];
        let hgt = match hgt_str
                    .replace("cm", "")
                    .replace("in", "").parse::<u32>() {
            Ok(v) => v,
            Err(_e) => {
                eprintln!("{} invalid hgt {}", pid_str, hgt_str);
                return None;
            }
        };
        if hgt_str.ends_with("cm") {
            if hgt < 150 || hgt > 193 {
                eprintln!("{} invalid hgt in cm {}", pid_str, hgt);
                return None;
            }
        } else if hgt_str.ends_with("in") {
            if hgt < 59 || hgt > 76 {
                eprintln!("{} invalid hgt in in {}", pid_str, hgt);
                return None;
            }
        } else {
            eprintln!("{} no unit in hgt {}", pid_str, hgt_str);
            return None;
        }

        let hcl_str = &entries["hcl"];
        if !hcl_str.starts_with("#") {
            eprintln!("{} invalid hair colour {}", pid_str, hcl_str);
            return None;
        }
        let hcl = match u32::from_str_radix(&hcl_str.replace("#", ""), 16) {
            Ok(v) => v,
            Err(_e) => {
                eprintln!("{} invalid hcl {}", pid_str, hcl_str);
                return None;
            }
        };

        let valid_eyes = vec!(
            "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
        );
        let ecl = &entries["ecl"];
        // contains does not work with &String :(
        if !valid_eyes.iter().any(|x| x.to_string() == *ecl) {
            eprintln!("{} invalid eye colour {}", pid_str, ecl);
            return None;
        }

        // ignore cid to pass security
        let cid_str = match entries.get("cid") {
            Some(v) => v.clone(),
            None => "0".to_string(),
        };
        let cid = match cid_str.parse::<u64>() {
            Ok(v) => v,
            Err(_e) => {
                eprintln!("{} invalid cid {}", pid_str, cid_str);
                return None;
            },
        };

        Some(Passport {
            byr,
            iyr,
            eyr,
            hgt: hgt_str.clone(),
            hcl,
            ecl: ecl.clone(),
            pid,
            cid,
        })
    }

    fn from_passport_data(passport_data: Vec<HashMap<String, String>>) -> Vec<Passport> {
        let mut rv = Vec::new();
        for passport_entry in passport_data {
            if let Some(passport) = Self::from_hashmap(&passport_entry) {
                rv.push(passport);
            }
        }
        rv
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let passport_data = read_passport_data(&path).expect("no content");
    println!("Found {} passports with all fields", passport_data.len());

    let valid_passports = Passport::from_passport_data(passport_data);
    println!("Found {} passports with valid fields", valid_passports.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn hash_from_string(value: &str) -> HashMap<String, String> {
        let mut data_store: HashMap<String, String> = HashMap::new();  // data is on multi lines
        for pair in value.split_whitespace() {
            let key_val: Vec<&str> = pair.split(":").collect();
            data_store.insert(key_val[0].to_string(), key_val[1].to_string());
        }
        data_store
    }

    #[rstest()]
    fn test_read_multiple_lines() {
        let data = read_passport_data(&"day_04/test_1.txt");
        assert!(data.is_ok());
        assert_eq!(data.unwrap().len(), 2);
    }

    #[rstest(content,
    case(&"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 iyr:2018 byr:1926"),
    case(&"iyr:2019 hcl:#602927 eyr:1967 hgt:170cm pid:012533040 byr:1946"),
    )]
    fn test_missing_entries_passport(content: &str) {
        println!("Checking {}", content);
        assert!(!valid_passport_data(&hash_from_string(content)));
    }

    #[rstest(content,
    case(&"eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
    case(&"iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"),
    case(&"hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
    case(&"hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"),
    )]
    fn test_invalid_passport(content: &str) {
        println!("Checking {}", content);
        let p = Passport::from_hashmap(&hash_from_string(content));
        assert!(p.is_none());
    }

    #[rstest(content,
    case(&"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"),
    case(&"eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
    case(&"hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022"),
    case(&"iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
    )]
    fn test_valid_passport(content: &str) {
        println!("Checking {}", content);
        let p = Passport::from_hashmap(&hash_from_string(content));
        assert!(p.is_some());
    }

    #[rstest()]
    fn test_q1() {
        let passport_data = read_passport_data(&"day_04/input.txt").expect("no content");
        assert_eq!(passport_data.len(), 182);
    }

    #[rstest()]
    fn test_q2() {
        let passport_data = read_passport_data(&"day_04/input.txt").expect("no content");
        let valid_passports = Passport::from_passport_data(passport_data);
        assert_eq!(valid_passports.len(), 109);
    }
}
