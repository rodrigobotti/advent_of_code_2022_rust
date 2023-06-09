use std::collections::HashSet;

use itertools::Itertools;

use crate::utils;

const INPUT_FILE_NAME: &str = "day_4.txt";

// **************************
// **        PART 1        **
// **************************

fn to_ids_set(s: &str) -> HashSet<usize> {
    match s.split('-').collect_tuple() {
        Some((start, end)) => {
            let start: usize = start.parse().expect("start {start} is not a number");
            let end: usize = end.parse().expect("end {end} is not a number");
            HashSet::from_iter(start..=end)
        }
        None => panic!("could parse {s} as a range of ids"),
    }
}

fn to_elf_pair(s: &str) -> (HashSet<usize>, HashSet<usize>) {
    match s.split(',').collect_tuple() {
        Some((e1, e2)) => (to_ids_set(e1), to_ids_set(e2)),
        None => panic!("could not parse {s} as two elves assignments"),
    }
}

fn is_redundant_assignment(s: &str) -> bool {
    let (elf_1, elf_2) = to_elf_pair(s);

    elf_1.is_subset(&elf_2) || elf_2.is_subset(&elf_1)
}

pub fn solution_part_1() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let redundant_count: usize = lines
        .map(utils::read_line)
        .filter(|line| is_redundant_assignment(line.as_str()))
        .map(|_| 1)
        .sum();

    println!("Day 4 pt 1 answers is: {redundant_count}");
}

// **************************
// **        PART 2        **
// **************************

fn has_overlaping_assigment(s: &str) -> bool {
    let (elf_1, elf_2) = to_elf_pair(s);

    !elf_1.intersection(&elf_2).collect::<Vec<_>>().is_empty()
}

pub fn solution_part_2() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let overlap_count: usize = lines
        .map(utils::read_line)
        .filter(|line| has_overlaping_assigment(line.as_str()))
        .map(|_| 1)
        .sum();

    println!("Day 4 pt 2 answers is: {overlap_count}");
}
