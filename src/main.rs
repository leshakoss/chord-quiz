mod get_midi_port;
mod quizzes;

extern crate colored;
extern crate midir;
extern crate rand;
extern crate wmidi;

use std::collections::HashSet;
use std::convert::TryFrom;
use std::error::Error;
use std::io::stdin;

use midir::{Ignore, MidiInput};
use wmidi::MidiMessage::{self, NoteOff, NoteOn};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

#[derive(PartialEq)]
enum QuizState {
    Guessing,
    Pending,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut midi_input = MidiInput::new("midir reading input")?;
    midi_input.ignore(Ignore::None);
    // Get an input port (read from console if multiple are available)

    let in_port = get_midi_port::get_midi_port(&midi_input)?;

    println!("\nOpening connection");
    let in_port_name = midi_input.port_name(&in_port)?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    let quiz: &quizzes::Quiz = quizzes::get_quiz();
    let mut quiz_option = quizzes::random_quiz_option(&quiz);
    let mut base = quizzes::random_base();

    let mut current_notes = HashSet::new();
    quizzes::display_quiz(base, quiz_option);
    let mut state = QuizState::Guessing;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_input.connect(
        &in_port,
        "midir-read-input",
        move |_, message, _| match MidiMessage::try_from(message) {
            Ok(NoteOn(_, note, _)) => {
                current_notes.insert(note);

                if current_notes.len() == quiz_option.signature.len()
                    && state == QuizState::Guessing
                {
                    state = QuizState::Pending;
                    quizzes::display_result(base, &quiz_option.signature, &current_notes)
                }
            }
            Ok(NoteOff(_, note, _)) => {
                current_notes.remove(&note);

                if current_notes.len() == 0 && state == QuizState::Pending {
                    state = QuizState::Guessing;
                    quiz_option = quizzes::random_quiz_option(&quiz);
                    base = quizzes::random_base();
                    quizzes::display_quiz(base, quiz_option)
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
