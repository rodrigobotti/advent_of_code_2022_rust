use crate::utils;
use itertools::Itertools;

const INPUT_FILE_NAME: &str = "day_1.txt";

// **************************
// **        PART 1        **
// **************************

pub fn solution_part_1() {
    let mut max_calories: usize = 0;
    let mut current_elf_calories: usize = 0;

    let lines = utils::read_input_lines(INPUT_FILE_NAME);
    for line in lines {
        let line = line.expect("failed to read line from buffered file");
        if line.is_empty() {
            if current_elf_calories > max_calories {
                max_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue;
        }
        let calory: usize = line.parse().expect("{line} is not a number");
        current_elf_calories += calory
    }

    println!("Day 1 pt 1 answer is: {max_calories}");
}

pub fn soluction_iterator_part_1() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let max_calories = lines
        .map(|line| line.ok()?.parse::<usize>().ok())
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

// **************************
// **        PART 2        **
// **************************

pub fn soluction_iterator_part_2() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let max_calories: usize = lines
        .map(|line| line.ok()?.parse::<usize>().ok())
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

    println!("Day 1 pt 2 answer is: {max_calories}");
}
