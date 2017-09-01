#[macro_use]
extern crate log;
extern crate env_logger;
extern crate fluidsynth_bindgen;
extern crate structopt;
extern crate toml;
extern crate ghakuf;

#[macro_use]
extern crate structopt_derive;
#[macro_use]
extern crate serde_derive;

use std::ffi::CString;
use std::os::raw::c_int;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use structopt::StructOpt;
use fluidsynth_bindgen::*;
use ghakuf::messages::*;
use ghakuf::reader::*;

#[derive(Debug, Deserialize)]
struct OptionalRenderSettings {
    input_file: Option<String>,
    output_file: Option<String>,
    debug_mode: Option<bool>,
    sample_rate: Option<u64>,
}

#[derive(Debug)]
struct RenderSettings {
    input_file: String,
    input_path: PathBuf,
    output_file: String,
    debug_mode: bool,
    sample_rate: u64,
}

fn to_render_settings(r: OptionalRenderSettings, p: PathBuf) -> RenderSettings {
    RenderSettings {
        input_file: r.input_file.unwrap_or(String::from("input.midi")),
        input_path: p,
        output_file: r.output_file.unwrap_or(String::from("output.wav")),
        debug_mode: r.debug_mode.unwrap_or(false),
        sample_rate: r.sample_rate.unwrap_or(48_000),
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "musicrenderer_rust", about = "A simple program to render the music for OpenRCT2-OpenMusic")]
struct Options {
    #[structopt(help = "Input file")]
    input: String,

    #[structopt(help = "Resource directory")]
    resources: String,
}

struct MIDIHandler {}

impl Handler for MIDIHandler {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        let _ = (format, track, time_base);
    }
    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        let _ = (delta_time, event, data);
    }
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        let _ = (delta_time, event);
        //             self.status = HandlerStatus::SkipTrack;
    }
    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        let _ = (delta_time, event, data);
    }
    fn track_change(&mut self) {
        //             self.status = HandlerStatus::Continue;
    }
    //         fn status(&mut self) -> HandlerStatus {
    //             self.status.clone()
    //         }
}

const MAX_POLYPHONY: c_int = 1_024;

fn read_input_file(options: Options) -> RenderSettings {
    let input_file = Path::new(&options.input);
    let mut file = File::open(input_file.to_str().unwrap()).unwrap();
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Could not read input file!");
    }

    let render_settings: OptionalRenderSettings = toml::from_str(&contents).unwrap();
    to_render_settings(render_settings, input_file.parent().unwrap().to_path_buf())
}

fn process_render_settings(render_settings: RenderSettings) {
    let mut midi_file = render_settings.input_path.clone();
    midi_file.push(&render_settings.input_file);
    let mut reader = Reader::new(
        Box::new(MIDIHandler {}),
        &midi_file.to_str().unwrap(),
    ).unwrap();

    let _ = reader.read();

    unsafe {
        let synth_polyphony_string: CString = CString::new("synth.polyphony").unwrap();

        let settings = new_fluid_settings();
        fluid_settings_setint(settings, synth_polyphony_string.as_ptr(), MAX_POLYPHONY);
        let mut set_poly: i32 = 0;
        fluid_settings_getint(settings, synth_polyphony_string.as_ptr(), &mut set_poly as *mut i32);
        assert_eq!(set_poly, MAX_POLYPHONY);
        info!("Same ({} = {})", set_poly, MAX_POLYPHONY);
        delete_fluid_settings(settings);
    }
}

fn main() {
    env_logger::init().unwrap();

    let opt = Options::from_args();
    info!("Options: {:?}", opt);

    let render_settings = read_input_file(opt);
    info!("Render settings: {:?}", render_settings);

    process_render_settings(render_settings);
}
