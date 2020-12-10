use std::fs;

pub fn file_to_vec(path: &str) -> Vec<String> {
    let input = fs::read_to_string(path).unwrap();
    input
        .lines()
        .map(|x| x.to_string())
        .collect()
}