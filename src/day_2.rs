use std::collections::HashMap;

use itertools::Itertools;

use crate::utils;

const INPUT_FILE_NAME: &str = "day_2.txt";

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
    let player_move_points = PT1_PLAYER_MOVE_POINTS
        .get(player)
        .expect("cannot reolve points for player move {player}");

    let outcome_points = PT1_ROUND_MOVES_POINTS
        .get(&(opponent, player))
        .expect("cannot resolve round points given opponent={opponent} + player={player}");

    player_move_points + outcome_points
}

pub fn solution_part_1() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let points: usize = lines
        .map(
            |line| match utils::read_line(line).split_whitespace().collect_tuple() {
                Some((o, p)) => (o.to_owned(), p.to_owned()),
                None => panic!("failed to parse line into opponent and player"),
            },
        )
        .map(|(o, p)| pt1_round_points(o.as_str(), p.as_str()))
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
    let outcome_points = PT2_OUTCOME_POINTS
        .get(outcome)
        .expect("cannot calculate outcome points for {outcome}");

    let player_move = PT2_ROUND_PLAYER_MOVE
        .get(&(opponent, outcome))
        .expect("cannot resolve player move from outcome={outcome} + opponent={opponent}");

    let player_move_points = PT2_PLAYER_MOVE_POINTS.get(player_move).unwrap();

    outcome_points + player_move_points
}

pub fn solution_part_2() {
    let lines = utils::read_input_lines(INPUT_FILE_NAME);

    let points: usize = lines
        .map(
            |line| match utils::read_line(line).split_whitespace().collect_tuple() {
                Some((o, p)) => (o.to_owned(), p.to_owned()),
                None => panic!("failed to parse line into opponent and outcome"),
            },
        )
        .map(|(opponent, outcome)| pt2_round_points(opponent.as_str(), outcome.as_str()))
        .sum();

    println!("Day 2 pt 2 answers is: {points}");
}
