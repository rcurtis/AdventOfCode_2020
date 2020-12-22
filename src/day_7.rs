use crate::util;
use std::collections::{HashMap, };

#[derive(Eq, PartialEq, Debug, Hash)]
struct Description {
    adverb: String,
    color: String
}

impl Description {
    fn new (adverb: &str, color: &str) -> Self {
        Description { adverb: adverb.to_string(), color: color.to_string() }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Bag {
    description: Description,
    contents: HashMap<Description, i32>
}

impl Bag {
    fn new(input: &str) -> Self {
        todo!()
    }
}

pub fn read_input() -> Vec<String> {
    util::file_to_vec("day_7_input.txt")
}

fn new_bags(input: &[&str]) -> Vec<Bag> {
    input.iter()
        .map(|x| new_bag(x))
        .collect::<Vec<_>>()
}

fn new_bag(input: &str) -> Bag {
    let bag_strs: Vec<&str> = input.split("contain").collect();
    let root_bag_str = bag_strs[0].split(" ").take(2).collect::<Vec<_>>();
    let sub_bags = if bag_strs[1].contains(",") {
        let mut descriptions = HashMap::new();
        let bags = bag_strs[1].split(",").collect::<Vec<_>>();
        for bag in bags {
            let adverb_color = bag.split(" ").collect::<Vec<_>>();
            let amount: i32 = adverb_color[1].parse().unwrap();
            descriptions.insert(Description::new(adverb_color[2], adverb_color[3]), amount);
        }
        descriptions
    } else {
        let adverb_color = bag_strs[1].split(" ").collect::<Vec<_>>();
        let mut descriptions = HashMap::new();
        let amount: i32 = adverb_color[1].parse().unwrap();
        descriptions.insert(Description::new(adverb_color[2], adverb_color[3]), amount);
        descriptions
    };

    let d = Description::new(root_bag_str[0], root_bag_str[1]);
    let b = Bag { description: d, contents: sub_bags};
    b
}

fn can_be_sub_bag(sub_desc: &Description, bags: &Vec<Bag>) -> usize {
    let mut match_count = 0_usize;
    for bag in bags {
        if bag.contents.keys().any(|x| x == sub_desc) {
            match_count += 1;
        }
        for (sub_bag, _) in &bag.contents {
            match_count += can_be_sub_bag(&sub_bag, &bags);
        }
    }
    match_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn new_bag_1() {
        let yes_count = new_bag("light red bags contain 1 bright white bag.");

        let mut hashmap = HashMap::new();
        hashmap.insert(Description::new("bright", "white"), 1);

        assert_eq!(yes_count, Bag {
            description: Description::new("light", "red"),
            contents: hashmap }) ;
    }

    #[test]
    fn new_bag_2() {
        let yes_count = new_bag("posh tomato bags contain 1 faded yellow bag, 1 vibrant blue bag, 1 clear coral bag.");

        let mut hashmap = HashMap::new();
        hashmap.insert(Description::new("faded", "yellow"), 1);
        hashmap.insert(Description::new("vibrant", "blue"), 1);
        hashmap.insert(Description::new("clear", "coral"), 1);

        assert_eq!(yes_count, Bag {
            description: Description::new("posh", "tomato"),
            contents: hashmap }) ;
    }

    #[test]
    fn new_bag_3() {
        let bag = new_bag("clear crimson bags contain 4 muted black bags, 4 posh purple bags, 1 striped black bag, 5 bright black bags.");

        let mut hashmap = HashMap::new();
        hashmap.insert(Description::new("muted", "black"), 4);
        hashmap.insert(Description::new("posh", "purple"), 4);
        hashmap.insert(Description::new("striped", "black"), 1);
        hashmap.insert(Description::new("bright", "black"), 5);

        assert_eq!(bag, Bag {
            description: Description::new("clear", "crimson"),
            contents: hashmap }) ;
    }

    #[test]
    fn contains_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.".split("\r\n").collect::<Vec<_>>();
        let bags = new_bags(&input);
        let sub_desc = Description::new("shiny", "gold");
        let sub_count = can_be_sub_bag(&sub_desc, &bags);
        assert_eq!(4, sub_count);
    }
}