use std::{collections::HashMap, fs};

use itertools::Itertools;

use crate::utils;

// **************************
// **        PART 1        **
// **************************

// ---
// Rock    = A = X = 1
// Paper   = B = Y = 2
// Scissor = C = Z = 3
// ---
// Lost    = 0
// Draw    = 3
// Win     = 5
// ---

lazy_static! {
    static ref PT1_PLAYER_MOVE_POINTS: HashMap<&'static str, usize> = {
        utils::hashmap!(
            "X" => 1,
            "Y" => 2,
            "Z" => 3
        )
    };
    static ref PT1_ROUND_MOVES_POINTS: HashMap<(&'static str, &'static str), usize> = {
        utils::hashmap!(
            ("A", "X") =>   3,
            ("A", "Y") =>   6,
            ("A", "Z") =>   0,

            ("B", "X") =>   0,
            ("B", "Y") =>   3,
            ("B", "Z") =>   6,

            ("C", "X") =>   6,
            ("C", "Y") =>   0,
            ("C", "Z") =>   3
        )
    };
}

fn pt1_round_points(opponent: &str, player: &str) -> usize {
    let player_move_points = PT1_PLAYER_MOVE_POINTS.get(player).unwrap();
    let outcome_points = PT1_ROUND_MOVES_POINTS.get(&(opponent, player)).unwrap();

    player_move_points + outcome_points
}

pub fn solution_part_1() {
    let file_path = utils::input_file_path("day_2.txt");

    let points: usize = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| match line.split_whitespace().collect_tuple() {
            Some((o, p)) => (o, p),
            None => panic!("failed to parse line into opponent and player"),
        })
        .map(|(o, p)| pt1_round_points(o, p))
        .sum();

    println!("Day 2 pt 1 answers is: {points}");
}

// **************************
// **        PART 2        **
// **************************

#[derive(Debug, Eq, PartialEq, Hash)]
enum PlayerMoves {
    Rock,
    Paper,
    Scissors,
}

lazy_static! {
    static ref PT2_OUTCOME_POINTS: HashMap<&'static str, usize> = {
        utils::hashmap!(
            "X" => 0,
            "Y" => 3,
            "Z" => 6
        )
    };
    static ref PT2_ROUND_PLAYER_MOVE: HashMap<(&'static str, &'static str), PlayerMoves> = {
        utils::hashmap!(
            ("A", "X") =>   PlayerMoves::Scissors,
            ("A", "Y") =>   PlayerMoves::Rock,
            ("A", "Z") =>   PlayerMoves::Paper,

            ("B", "X") =>   PlayerMoves::Rock,
            ("B", "Y") =>   PlayerMoves::Paper,
            ("B", "Z") =>   PlayerMoves::Scissors,

            ("C", "X") =>   PlayerMoves::Paper,
            ("C", "Y") =>   PlayerMoves::Scissors,
            ("C", "Z") =>   PlayerMoves::Rock
        )
    };
    static ref PT2_PLAYER_MOVE_POINTS: HashMap<PlayerMoves, usize> = {
        utils::hashmap!(
            PlayerMoves::Rock       => 1,
            PlayerMoves::Paper      => 2,
            PlayerMoves::Scissors   => 3
        )
    };
}

fn pt2_round_points(opponent: &str, outcome: &str) -> usize {
    let outcome_points = PT2_OUTCOME_POINTS.get(outcome).unwrap();
    let player_move = PT2_ROUND_PLAYER_MOVE.get(&(opponent, outcome)).unwrap();
    let player_move_points = PT2_PLAYER_MOVE_POINTS.get(player_move).unwrap();

    outcome_points + player_move_points
}

pub fn solution_part_2() {
    let file_path = utils::input_file_path("day_2.txt");

    let points: usize = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| match line.split_whitespace().collect_tuple() {
            Some((o, p)) => (o, p),
            None => panic!("failed to parse line into opponent and outcome"),
        })
        .map(|(opponent, outcome)| pt2_round_points(opponent, outcome))
        .sum();

    println!("Day 2 pt 2 answers is: {points}");
}
