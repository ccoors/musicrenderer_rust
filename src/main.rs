#[macro_use]
extern crate log;
extern crate env_logger;
extern crate fluidsynth_bindgen;
extern crate structopt;
extern crate ghakuf;
#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;

use std::path::PathBuf;

use structopt::StructOpt;
use ghakuf::reader::*;

mod types;
mod tomlparser;
mod fluidsynthesizer;
mod midiparser;

fn process_render_settings(render_settings: &types::TOMLRenderSettings, resources: &PathBuf) {
    let mut midi_file = render_settings.input_path.clone();
    midi_file.push(&render_settings.input_file);
    let mut reader = Reader::new(
        Box::new(types::MIDIHandler {}),
        &midi_file.to_str().unwrap(),
    ).unwrap();

    info!("Generating FluidSynth synthesizers...");
    let fluid_synthesizers = fluidsynthesizer::generate_fluid_synthesizers(&render_settings, resources);
    let elements = fluid_synthesizers.len();
    info!("Generated {} FluidSynth synthesizer{}", elements, if elements == 1 { "" } else { "s" });

    info!("Parsing MIDI file");
    let _ = reader.read();
}

fn main() {
    env_logger::init().unwrap();

    let opt = types::Options::from_args();
    debug!("Options: {:?}", opt);

    let render_settings = tomlparser::read_input_file(&opt);
    debug!("Render settings: {:?}", render_settings);

    process_render_settings(&render_settings, &PathBuf::from(opt.resources));
}
