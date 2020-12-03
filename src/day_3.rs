use std::fs;

pub fn read_input() -> Vec<String> {
    let input = fs::read_to_string("day_3_input.txt").unwrap();
    input.lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

pub fn part_1(terrain: &Vec<String>) -> u64 {
    walk_count_trees(&terrain, 3, 1)
}

pub fn part_2(terrain: &Vec<String>) -> u64 {
    walk_count_trees(&terrain, 1, 1)
        * walk_count_trees(&terrain, 3, 1)
        * walk_count_trees(&terrain, 5, 1)
        * walk_count_trees(&terrain, 7, 1)
        * walk_count_trees(&terrain, 1, 2)
}

fn walk_count_trees(terrain: &Vec<String>, x_step: usize, y_step: usize) -> u64 {
    let width = terrain.get(0).unwrap().len();
    let mut x = 0;
    let mut y = 0;
    let mut tree_count = 0;

    while y < terrain.len() {
        let row = terrain.get(y);
        let row = row.unwrap();
        let tile = row.chars().nth(x).unwrap();

        if tile == '#' { tree_count += 1; }

        x = (x + x_step) % width;
        y += y_step ;
    }
    tree_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_correct_answer() {
        let input = read_input();
        let tree_count = walk_count_trees(&input, 3, 1);
        assert_eq!(214, tree_count)
    }

    #[test]
    fn part_2_correct_answer() {
        let input = read_input();
        let tree_count = part_2(&input);
        assert_eq!(8336352024, tree_count)
    }
}