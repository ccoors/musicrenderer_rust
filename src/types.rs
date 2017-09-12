use std::path::PathBuf;
use std::collections::HashMap;

use fluidsynth_bindgen::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "musicrenderer_rust", about = "A simple program to render the music for OpenRCT2-OpenMusic")]
pub struct Options {
    #[structopt(help = "Input file")]
    pub input: String,

    #[structopt(help = "Resource directory")]
    pub resources: String,

    #[structopt(short = "d", long = "debug", help = "Activate debug mode (save interstage products)")]
    pub debug: bool,
}

pub struct MIDIHandler {}

#[derive(Debug, Deserialize)]
pub struct TOMLOptionalRenderSettings {
    pub input_file: String,
    pub output_file: String,
    pub sample_rate: Option<u64>,

    pub synth: HashMap<String, TOMLSynth>,
}

#[derive(Debug)]
pub struct TOMLRenderSettings {
    pub input_path: PathBuf,

    pub input_file: String,
    pub output_file: String,
    pub sample_rate: u64,

    pub synth: HashMap<String, TOMLSynth>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLMapping {
    pub condition: Vec<TOMLCondition>,
    pub destination: Vec<TOMLDestination>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLCondition {
    pub program: Option<String>,
    pub track: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLDestination {
    pub bank: Option<u32>,
    pub program: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLSynth {
    pub synthtype: String,
    pub gain: f32,
    pub directory: Option<String>,
    pub soundfont: Option<Vec<TOMLSynthSoundfont>>,
    pub setting: Option<Vec<TOMLSynthSetting>>,
    pub mapping: HashMap<String, TOMLMapping>,
}

#[derive(Debug, Deserialize)]
pub struct TOMLSynthSoundfont {
    pub file: String,
    pub offset: i32,
}

#[derive(Debug, Deserialize)]
pub struct TOMLSynthSetting {
    pub name: String,
    pub value_f: Option<f64>,
    pub value_i: Option<i32>,
    pub value_s: Option<String>,
}

#[derive(Debug)]
pub struct FluidSynthesizer {
    pub settings: *mut fluid_settings_t,
    pub synthesizer: Option<*mut fluid_synth_t>,
    pub sequencer: Option<*mut fluid_sequencer_t>,
    pub synthesizer_seq_id: i16,
    pub gain: f32,
    pub last_event: i32,
}

pub fn to_render_settings(r: TOMLOptionalRenderSettings, p: PathBuf) -> TOMLRenderSettings {
    TOMLRenderSettings {
        input_file: r.input_file,
        input_path: p,
        output_file: r.output_file,
        sample_rate: r.sample_rate.unwrap_or(48_000),

        synth: r.synth,
    }
}
