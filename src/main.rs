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

mod types;
mod tomlparser;
mod fluidsynthesizer;

use types::*;

use std::ffi::CString;
use std::os::raw::c_int;

use structopt::StructOpt;
use fluidsynth_bindgen::*;
use ghakuf::messages::*;
use ghakuf::reader::*;

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

fn process_render_settings(render_settings: TOMLRenderSettings) {
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

    let render_settings = tomlparser::read_input_file(opt);
    info!("Render settings: {:?}", render_settings);

    process_render_settings(render_settings);
}
