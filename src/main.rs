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

use std::collections::HashMap;

use structopt::StructOpt;
use ghakuf::messages::*;
use ghakuf::reader::*;

mod types;
mod tomlparser;
mod fluidsynthesizer;

use types::*;

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

fn generate_fluid_synthesizers(settings: &HashMap<String, TOMLSynth>, options: &Options) -> Vec<FluidSynthesizer> {
    let mut res = vec!();
    for (id, settings) in settings {
        info!("Building fluid synthesizer '{}'", id);
        let mut synth = FluidSynthesizer::new();
        if settings.setting.is_some() {
            for setting in settings.setting.as_ref().unwrap() {
                if setting.value_i.is_some() {
                    let set = *setting.value_i.as_ref().unwrap();
                    debug!("Setting '{}' to {}", setting.name, set);
                    synth.settings_setint(&setting.name, set);
                }

                if setting.value_f.is_some() {
                    let set = *setting.value_f.as_ref().unwrap();
                    debug!("Setting '{}' to {}", setting.name, set);
                    synth.settings_setfloat(&setting.name, set);
                }

                if setting.value_s.is_some() {
                    let set = setting.value_s.as_ref().unwrap().clone();
                    debug!("Setting '{}' to '{}'", setting.name, set);
                    synth.settings_setstring(&setting.name, set);
                }
            }
        }

        synth.build();

        if settings.soundfont.is_some() {
            for soundfont in settings.soundfont.as_ref().unwrap() {
                info!("Loading soundfont '{}' with offset {}", soundfont.file, soundfont.offset);
                let soundfont_file = format!("{}{}", options.resources, soundfont.file);
                synth.load_soundfont(&soundfont_file, soundfont.offset);
            }
        }

        res.push(synth);
    }
    res
}


fn process_render_settings(render_settings: TOMLRenderSettings, options: &Options) {
    let mut midi_file = render_settings.input_path.clone();
    midi_file.push(&render_settings.input_file);
    let mut reader = Reader::new(
        Box::new(MIDIHandler {}),
        &midi_file.to_str().unwrap(),
    ).unwrap();

    let fluid_synthesizers = generate_fluid_synthesizers(&render_settings.synth, options);

    let _ = reader.read();
}

fn main() {
    env_logger::init().unwrap();

    let opt = Options::from_args();
    debug!("Options: {:?}", opt);

    let render_settings = tomlparser::read_input_file(&opt);
    debug!("Render settings: {:?}", render_settings);

    process_render_settings(render_settings, &opt);
}
