extern crate colored;
extern crate midir;
extern crate rand;
extern crate wmidi;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{stdin, stdout, Write};

use colored::*;
use midir::{Ignore, MidiInput};
use rand::Rng;
use wmidi::MidiMessage::{self, NoteOff, NoteOn};
use wmidi::Note;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

const MAJOR: [i16; 3] = [0, 4, 7];
const MINOR: [i16; 3] = [0, 3, 7];

enum ChordType {
    Major,
    Minor,
}

struct Quiz {
    chord_base: i16,
    chord_type: ChordType,
}

fn random_quiz() -> Quiz {
    let mut rng = rand::thread_rng();
    Quiz {
        chord_base: rng.gen_range(0, 12),
        chord_type: if rng.gen_range(0, 3) == 0 {
            ChordType::Major
        } else {
            ChordType::Minor
        },
    }
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

fn get_quiz_result(quiz: &Quiz, notes: &HashSet<Note>) -> bool {
    let current_chord: HashSet<i16> = notes
        .iter()
        .map(|&note| (note as i16 - quiz.chord_base) % 12)
        .collect();
    let chord_to_guess = match quiz.chord_type {
        ChordType::Major => MAJOR,
        ChordType::Minor => MINOR,
    }
    .iter()
    .cloned()
    .collect();
    let difference: Vec<&i16> = current_chord.difference(&chord_to_guess).collect();

    match difference.len() {
        0 => true,
        _ => false,
    }
}

fn display_answer(quiz: &Quiz) {
    let chord: Vec<&str> = match quiz.chord_type {
        ChordType::Major => MAJOR,
        ChordType::Minor => MINOR,
    }
    .iter()
    .map(|offset| get_chord_base_name((offset + quiz.chord_base) % 12))
    .collect();

    println!("Answer: {}", chord.join(" "))
}

fn display_result(quiz: &Quiz, notes: &HashSet<Note>) {
    match get_quiz_result(quiz, notes) {
        true => println!("{}", "Correct!".green()),
        false => println!("{}", "Incorrect!".red()),
    };
    display_answer(&quiz);
}

fn display_quiz(quiz: &Quiz) {
    println!(
        "\n{} {}",
        get_chord_base_name(quiz.chord_base).bold(),
        match quiz.chord_type {
            ChordType::Major => "major",
            ChordType::Minor => "minor",
        }
        .bold()
    )
}

#[derive(PartialEq)]
enum QuizState {
    Guessing,
    Pending,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };
    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    let mut current_notes = HashSet::new();
    let mut quiz = random_quiz();
    display_quiz(&quiz);
    let mut state = QuizState::Guessing;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message, _| match MidiMessage::try_from(message) {
            Ok(NoteOn(_, note, _)) => {
                current_notes.insert(note);

                if current_notes.len() == 3 && state == QuizState::Guessing {
                    state = QuizState::Pending;
                    display_result(&quiz, &current_notes)
                }
            }
            Ok(NoteOff(_, note, _)) => {
                current_notes.remove(&note);

                if current_notes.len() == 0 && state == QuizState::Pending {
                    state = QuizState::Guessing;
                    quiz = random_quiz();
                    display_quiz(&quiz)
                }
            }
            _ => {}
        },
        (),
    )?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
