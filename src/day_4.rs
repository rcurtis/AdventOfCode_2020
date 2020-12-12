use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

pub fn read_input() -> Vec<String> {
    let input = fs::read_to_string("day_4_input.txt").unwrap();
    input.split("\r\n\r\n")
        .map(|x| x.to_string().replace("\r\n", " "))
        .collect::<Vec<String>>()
}

fn map_has_all_keys_fn(map: &HashMap<&str, &str>, tokens: &[&str]) -> bool {
    tokens.iter().any(|token| !map.contains_key(token))
}

fn map_has_all_keys(map: &HashMap<&str, &str>, tokens: &[&str]) -> bool {
    for token in tokens {
        if !map.contains_key(token) { return false; }
    }
    true
}

fn tokenize(input: &str) -> HashMap<&str, &str> {
    let tokens: Vec<_> = input
        .lines()
        .flat_map(|x| x.split_whitespace().collect::<Vec<_>>())
        .collect();

    let mut map = HashMap::new();
    for token in tokens {
        let (k, v) = token.split_at(token.find(":").unwrap());
        map.insert(k, &v[1..]); // strip off the leading colon from the value
    }
    map
}

fn parse_passport_part_1(input: &str) -> bool {
    let map = tokenize(input);
    map_has_all_keys(&map, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
}

fn parse_passport_part_2(input: &str) -> bool {
    let map = tokenize(input);

    if map_has_all_keys(&map, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]) {
        let valid_byr = is_valid_year(map["byr"], 1920, 2002);
        let valid_iyr = is_valid_year(map["iyr"], 2010, 2020);
        let valid_eyr = is_valid_year(map["eyr"], 2020, 2030);
        let valid_hgt = is_valid_height(map["hgt"]);
        let valid_hcl = is_valid_hair_color(map["hcl"]);
        let valid_ecl = is_valid_eye_color(map["ecl"]);
        let valid_pid = is_valid_pid(map["pid"]);
        return valid_byr && valid_iyr && valid_eyr && valid_hgt && valid_hcl && valid_ecl && valid_pid;
    }

    false
}

fn is_valid_year(input: &str, min: i32, max: i32) -> bool {
    let year = input.parse::<i32>();
    match year {
        Ok(year) => year >= min && year <= max,
        Err(_) => false
    }
}

fn is_valid_height(height: &str) -> bool {
    let numerics: Vec<char> = height.chars().take_while(|x| x.is_digit(10)).collect();
    if numerics.len() == 0 { return false; }

    let num = String::from_iter(numerics.iter()).parse::<i32>();
    if num.is_err() { return false; }
    let num = num.unwrap();

    let measure_str = height.chars().skip_while(|x| x.is_digit(10)).collect::<Vec<_>>();
    if measure_str.len() == 0 { return false; }
    let measure_str = String::from_iter(measure_str.iter());

    match measure_str.as_str() {
        "cm" => num >= 150 && num <= 193,
        "in" => num >= 59 && num <= 76,
        _ => false
    }
}

fn is_valid_hair_color(color: &str) -> bool {
    if color.len() != 7 { return false; }
    if color.chars().next().unwrap() != '#' { return false; }

    let digits = color.chars().skip(1).collect::<Vec<_>>();

    let found_invalid = digits.iter().any(|x| !x.is_digit(16)); // What a hack
    !found_invalid
}

fn is_valid_eye_color(input: &str) -> bool {
    match input {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false
    }
}

fn is_valid_pid(input: &str) -> bool {
    if input.len() != 9 { return false; }
    !input.chars().any(|x| !x.is_digit(10))
}

pub fn count_valid_passports_part_1(input: &Vec<String>) -> usize {
    input.iter()
        .filter(|x| parse_passport_part_1(x))
        .count()
}

pub fn count_valid_passports_part_2(input: &Vec<String>) -> usize {
    input.iter()
        .filter(|x| parse_passport_part_2(x))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_with_all_fields() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let result = parse_passport_part_1(input);
        assert!(result)
    }

    #[test]
    fn parse_with_all_except_cid() {
        let input = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";
        let result = parse_passport_part_1(input);
        assert!(result)
    }

    #[test]
    fn parse_missing_hgt() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";
        let result = parse_passport_part_1(input);
        assert!(!result)
    }

    #[test]
    fn parse_missing_cid_and_byr() {
        let input = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let result = parse_passport_part_1(input);
        assert!(!result)
    }

    #[test]
    fn valid_year() {
        let input = "2002";
        let result = is_valid_year(input, 1920, 2002);
        assert!(result)
    }

    #[test]
    fn invalid_year() {
        let input = "2003";
        let result = is_valid_year(input, 1920, 2002);
        assert!(!result)
    }

    #[test]
    fn valid_height_in() {
        let input = "60in";
        let result = is_valid_height(input);
        assert!(result)
    }

    #[test]
    fn valid_height_cm() {
        let input = "190cm";
        let result = is_valid_height(input);
        assert!(result)
    }

    #[test]
    fn invalid_height_in() {
        let input = "190in";
        let result = is_valid_height(input);
        assert!(!result)
    }

    #[test]
    fn invalid_height_cm() {
        let input = "190";
        let result = is_valid_height(input);
        assert!(!result)
    }

    #[test]
    fn valid_hair_color() {
        let input = "#123abc";
        let result = is_valid_hair_color(input);
        assert!(result)
    }

    #[test]
    fn invalid_hair_color1() {
        let input = "#123abz";
        let result = is_valid_hair_color(input);
        assert!(!result)
    }

    #[test]
    fn is_digit_is_fucking_weird() {
        let v = 'f'.is_digit(16); // base 16 stops at 'f'
        assert!(v)
    }

    #[test]
    fn valid_eye_colors() {
        let result = is_valid_eye_color("brn");
        assert!(result);
    }

    #[test]
    fn invalid_eye_colors() {
        let result = is_valid_eye_color("wat");
        assert!(!result);
    }

    #[test]
    fn valid_pid() {
        let result = is_valid_pid("000000001");
        assert!(result);
    }

    #[test]
    fn invalid_pid() {
        let result = is_valid_pid("0123456789");
        assert!(!result);
    }

    #[test]
    fn invalid_part2_1()
    {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
        let result = parse_passport_part_2(input);
        assert!(!result);
    }

    #[test]
    fn invalid_part2_2()
    {
        let input = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
        let result = parse_passport_part_2(input);
        assert!(!result);
    }

    #[test]
    fn invalid_part2_3()
    {
        let input = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
        let result = parse_passport_part_2(input);
        assert!(!result);
    }

    #[test]
    fn invalid_part2_4()
    {
        let input = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let result = parse_passport_part_2(input);
        assert!(!result);
    }

    #[test]
    fn valid_part2_2()
    {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
        let result = parse_passport_part_2(input);
        assert!(result);
    }

    #[test]
    fn valid_part2_3()
    {
        let input = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
        let result = parse_passport_part_2(input);
        assert!(result);
    }

    #[test]
    fn valid_part2_4()
    {
        let input = "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
        let result = parse_passport_part_2(input);
        assert!(result);
    }

    #[test]
    fn valid_part2_5()
    {
        let input = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let result = parse_passport_part_2(input);
        assert!(result);
    }
}