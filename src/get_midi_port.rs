extern crate midir;

use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;

use midir::{MidiInput, MidiInputPort};

pub fn get_midi_port(midi_input: &MidiInput) -> Result<MidiInputPort, Box<dyn Error>> {
    // Get an input port (read from console if multiple are available)
    let in_ports = midi_input.ports();
    match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available MIDI device: {}",
                midi_input.port_name(&in_ports[0]).unwrap()
            );
            Ok(in_ports.into_iter().nth(0).unwrap())
        }
        _ => {
            let port_names: Vec<String> = in_ports
                .iter()
                .map(|x| midi_input.port_name(&x).unwrap())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a MIDI device")
                .default(0)
                .items(&port_names[..])
                .interact()
                .unwrap();
            in_ports
                .into_iter()
                .nth(selection)
                .ok_or("invalid input port selected".into())
        }
    }
}
