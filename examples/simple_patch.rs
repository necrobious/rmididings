extern crate alsa;

use std::error::Error;

use rmididings::*;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {

    let mut md = RMididings::new()?;

    md.config(ConfigArguments {
        client_name: "RMididings Demo",
        in_ports: &[
            ["input", "Virtual Keyboard:Virtual Keyboard"],
        ],
        out_ports: &[
            ["output", "midisnoop:MIDI Input"],
        ],
        ..ConfigArguments::default()
    })?;

    println!("Started");

    let patch = Pass();

    md.run(RunArguments {
        patch: &patch,
        ..RunArguments::default()
    })?;

    Ok(())
}
