use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "musicrenderer_rust", about = "A simple program to render the music for OpenRCT2-OpenMusic")]
pub struct Options {
    #[structopt(help = "Input file")]
    pub input: String,

    #[structopt(help = "Resource directory")]
    pub resources: String,
}

pub struct MIDIHandler {}

#[derive(Debug, Deserialize)]
pub struct OptionalRenderSettings {
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub debug_mode: Option<bool>,
    pub sample_rate: Option<u64>,
}

#[derive(Debug)]
pub struct RenderSettings {
    pub input_file: String,
    pub input_path: PathBuf,
    pub output_file: String,
    pub debug_mode: bool,
    pub sample_rate: u64,
}

pub fn to_render_settings(r: OptionalRenderSettings, p: PathBuf) -> RenderSettings {
    RenderSettings {
        input_file: r.input_file.unwrap_or(String::from("input.midi")),
        input_path: p,
        output_file: r.output_file.unwrap_or(String::from("output.wav")),
        debug_mode: r.debug_mode.unwrap_or(false),
        sample_rate: r.sample_rate.unwrap_or(48_000),
    }
}

