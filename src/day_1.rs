use crate::utils::input_file_path;
use itertools::Itertools;
use std::fs;

pub fn solution_part_1() {
    let file_path = input_file_path("day_1.txt");

    let mut max_calories: usize = 0;
    let mut current_elf_calories: usize = 0;

    for line in fs::read_to_string(file_path).unwrap().lines() {
        if line.is_empty() {
            if current_elf_calories > max_calories {
                max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue;
        }
        let calory: usize = line.parse().unwrap();
        current_elf_calories += calory
    }

    println!("Day 1 pt 1 answer is: {max_calories}");
}

pub fn soluction_iterator_part_1() {
    let file_path = input_file_path("day_1.txt");

    let max_calories = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .coalesce(|prev, curr| match (prev, curr) {
            (None, None) => Ok(None),
            (None, Some(v)) => Ok(Some(v)),
            (Some(v), None) => Err((Some(v), None)),
            (Some(p), Some(c)) => Ok(Some(p + c)),
        })
        .max()
        .flatten()
        .unwrap();
    // .batching(|it| {
    //     let mut elf_calory = None;
    //     while let Some(Some(calory)) = it.next() {
    //         elf_calory = Some(elf_calory.unwrap_or(0) + calory)
    //     }
    //     elf_calory
    // })
    // .max()
    // .unwrap();

    println!("Day 1 pt 1 answer is: {max_calories}");
}

pub fn soluction_iterator_part_2() {
    let file_path = input_file_path("day_1.txt");

    let max_calories: usize = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .coalesce(|prev, curr| match (prev, curr) {
            (None, None) => Ok(None),
            (None, Some(v)) => Ok(Some(v)),
            (Some(v), None) => Err((Some(v), None)),
            (Some(p), Some(c)) => Ok(Some(p + c)),
        })
        .flatten()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    println!("Day 1 pt 1 answer is: {max_calories}");
}
