use std::path::PathBuf;
use std::collections::HashMap;

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
pub struct OptionalRenderSettings {
    pub input_file: String,
    pub output_file: String,
    pub sample_rate: Option<u64>,

    pub synth: HashMap<String, Synth>,
}

#[derive(Debug)]
pub struct RenderSettings {
    pub input_path: PathBuf,

    pub input_file: String,
    pub output_file: String,
    pub sample_rate: u64,

    pub synth: HashMap<String, Synth>,
}

#[derive(Debug, Deserialize)]
pub struct Mapping {
    pub condition: Vec<Condition>,
    pub destination: Vec<Destination>,
}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub program: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Destination {
    pub patch: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Synth {
    pub synthtype: String,
    pub gain: Option<f32>,
    pub directory: Option<String>,
    pub soundfont: Option<Vec<SynthSoundfont>>,
    pub setting: Option<Vec<SynthSetting>>,
    pub mapping: HashMap<String, Mapping>,
}

#[derive(Debug, Deserialize)]
pub struct SynthSoundfont {
    pub file: String,
    pub offset: u32,
}

#[derive(Debug, Deserialize)]
pub struct SynthSetting {
    pub name: String,
    pub value_f: Option<f32>,
    pub value_i: Option<u32>,
}

pub fn to_render_settings(r: OptionalRenderSettings, p: PathBuf) -> RenderSettings {
    RenderSettings {
        input_file: r.input_file,
        input_path: p,
        output_file: r.output_file,
        sample_rate: r.sample_rate.unwrap_or(48_000),

        synth: r.synth,
    }
}
