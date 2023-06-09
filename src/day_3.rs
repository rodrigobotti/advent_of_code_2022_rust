use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

use itertools::Itertools;

use crate::utils;

const INPUT_FILE_NAME: &str = "day_3.txt";

// **************************
// **        PART 1        **
// **************************

lazy_static! {
    static ref PRIORITIES: HashMap<char, usize> = {
        let lower: Vec<_> = ('a'..='z').collect();
        let upper: Vec<_> = ('A'..='Z').collect();

        let characters = [lower, upper].concat();
        let capacity = characters.len();

        let mut priorities = HashMap::with_capacity(capacity);

        for (i, c) in characters.iter().enumerate() {
            priorities.insert(*c, i + 1);
        }

        priorities
    };
}

fn split_into_compartments(rucksack: &str) -> (&str, &str) {
    let length = rucksack.len();
    if length % 2 != 0 {
        panic!("rucksack {rucksack} does not have an even size")
    }

    rucksack.split_at(length / 2)
}

fn common_item(first_compartment: &str, second_compartment: &str) -> char {
    let f: HashSet<char> = first_compartment.chars().collect();
    let s: HashSet<char> = second_compartment.chars().collect();

    let common: Vec<_> = f.intersection(&s).collect();

    if common.is_empty() {
        panic!(
            "Compartments {first_compartment} and {second_compartment} have no elements in common"
        )
    }
    if common.len() > 1 {
        panic!("Compartments {first_compartment} and {second_compartment} have more than one element in common {common:?}")
    }

    **common.first().unwrap()
}

fn get_priority(c: char) -> usize {
    *PRIORITIES.get(&c).unwrap()
}

pub fn solution_part_1() {
    let file_path = utils::input_file_path(INPUT_FILE_NAME);

    let priorities: usize = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(split_into_compartments)
        .map(|(first, second)| common_item(first, second))
        .map(get_priority)
        .sum();

    println!("Day 3 pt 1 answers is: {priorities}");
}

// **************************
// **        PART 2        **
// **************************

fn intersection<T>(sets: &mut Vec<HashSet<T>>) -> HashSet<T>
where
    T: Eq + Hash,
{
    let mut result = sets.pop().unwrap();
    result.retain(|item| sets.iter().all(|set| set.contains(item)));
    result
}

fn find_badge(elf_1: &str, elf_2: &str, elf_3: &str) -> char {
    let a: HashSet<char> = elf_1.chars().collect();
    let b: HashSet<char> = elf_2.chars().collect();
    let c: HashSet<char> = elf_3.chars().collect();

    let badge: Vec<_> = intersection(&mut vec![a, b, c]).into_iter().collect();

    if badge.is_empty() {
        panic!("Elves have no badge")
    }
    if badge.len() > 1 {
        panic!("Elves have multiple badges")
    }

    *badge.first().unwrap()
}

pub fn solution_part_2() {
    let file_path = utils::input_file_path(INPUT_FILE_NAME);

    let priorities: usize = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| match chunk.collect_tuple() {
            Some((a, b, c)) => (a, b, c),
            None => panic!("Could not group elves in groups of 3"),
        })
        .map(|(a, b, c)| find_badge(a, b, c))
        .map(get_priority)
        .sum();

    println!("Day 3 pt 2 answers is: {priorities}");
}
