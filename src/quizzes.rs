extern crate colored;
extern crate midir;
extern crate rand;
extern crate wmidi;

use std::collections::HashSet;

use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use rand::seq::SliceRandom;
use rand::Rng;
use wmidi::Note;

pub struct QuizOption {
    name: &'static str,
    pub signature: &'static [i16],
}

pub struct Quiz {
    name: &'static str,
    options: &'static [QuizOption],
}

const QUIZES: &'static [Quiz] = &[
    Quiz {
        name: "Fifths",
        options: &[QuizOption {
            name: " + fifth",
            signature: &[7],
        }],
    },
    Quiz {
        name: "Major and minor chords",
        options: &[
            QuizOption {
                name: " major",
                signature: &[0, 4, 7],
            },
            QuizOption {
                name: " minor",
                signature: &[0, 3, 7],
            },
        ],
    },
    Quiz {
        name: "Root and shell",
        options: &[
            QuizOption {
                name: "maj7",
                signature: &[0, 4, 11],
            },
            QuizOption {
                name: "min7",
                signature: &[0, 3, 10],
            },
            QuizOption {
                name: "7",
                signature: &[0, 4, 10],
            },
        ],
    },
];

pub fn random_quiz_option(quiz: &Quiz) -> &QuizOption {
    quiz.options.choose(&mut rand::thread_rng()).unwrap()
}

pub fn random_base() -> i16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 12)
}

fn get_chord_base_name(chord_base: i16) -> &'static str {
    match chord_base {
        0 => "C",
        1 => "Db",
        2 => "D",
        3 => "Eb",
        4 => "E",
        5 => "F",
        6 => "Gb",
        7 => "G",
        8 => "Ab",
        9 => "A",
        10 => "Bb",
        11 => "B",
        _ => "?",
    }
}

fn get_quiz_result(base: i16, signature: &[i16], notes: &HashSet<Note>) -> bool {
    let current_chord: HashSet<i16> = notes
        .iter()
        .map(|&note| (note as i16 - base) % 12)
        .collect();
    let chord_to_guess = signature.iter().cloned().collect();
    let difference: Vec<&i16> = current_chord.difference(&chord_to_guess).collect();

    match difference.len() {
        0 => true,
        _ => false,
    }
}

fn display_answer(base: i16, signature: &[i16]) {
    let chord: Vec<&str> = signature
        .iter()
        .map(|offset| get_chord_base_name((offset + base) % 12))
        .collect();

    println!("Answer: {}", chord.join(" "))
}

pub fn display_result(base: i16, signature: &[i16], notes: &HashSet<Note>) {
    match get_quiz_result(base, signature, notes) {
        true => println!("{}", "Correct!".green()),
        false => println!("{}", "Incorrect!".red()),
    };
    display_answer(base, signature);
}

pub fn display_quiz(base: i16, quiz_option: &QuizOption) {
    println!(
        "\n{}{}",
        get_chord_base_name(base).bold(),
        quiz_option.name.bold()
    )
}

pub fn get_quiz() -> &'static Quiz {
    let quiz_names: Vec<String> = QUIZES.iter().map(|x| x.name.to_string()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a quiz")
        .default(0)
        .items(&quiz_names[..])
        .interact()
        .unwrap();
    &QUIZES[selection]
}
