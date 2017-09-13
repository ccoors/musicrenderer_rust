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

mod types;
mod tomlparser;
mod fluidsynthesizer;
mod midiparser;
mod renderer;

fn main() {
    env_logger::init().unwrap();

    let opt = types::Options::from_args();
    debug!("Options: {:?}", opt);

    let render_settings = tomlparser::read_input_file(&opt);
    debug!("Render settings: {:?}", render_settings);

    renderer::process_render_settings(&render_settings, &PathBuf::from(opt.resources));
}
