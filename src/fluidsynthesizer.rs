use std::ffi::CString;
use std::path::PathBuf;

use std::os::raw::c_int;
use std::os::raw::c_double;

use fluidsynth_bindgen::*;

use types::*;
use gm_instruments;

impl FluidSynthesizer {
    pub fn new() -> FluidSynthesizer {
        unsafe {
            FluidSynthesizer {
                settings: new_fluid_settings(),
                synthesizer: None,
                sequencer: None,
                synthesizer_seq_id: 0,
                gain: 1.0,
                last_event: 0,
                used_channels: 0,
                mapping: Vec::new(),
            }
        }
    }

    pub fn settings_setint(&self, name: &str, value: c_int) {
        let name: CString = CString::new(name).unwrap();
        unsafe { assert_eq!(fluid_settings_setint(self.settings, name.as_ptr(), value), 1); }

        // Verify
        let mut set_value: i32 = 0;
        unsafe { fluid_settings_getint(self.settings, name.as_ptr(), &mut set_value as *mut i32); }
        assert_eq!(set_value, value);
    }

    pub fn settings_setfloat(&self, name: &str, value: c_double) {
        let name: CString = CString::new(name).unwrap();
        unsafe { assert_eq!(fluid_settings_setnum(self.settings, name.as_ptr(), value), 1); }

        // Verify
        let mut set_value: f64 = 0.0;
        unsafe { fluid_settings_getnum(self.settings, name.as_ptr(), &mut set_value as *mut f64); }
        assert_eq!(set_value, value);
    }

    pub fn settings_setstring(&self, name: &str, value: String) {
        let name: CString = CString::new(name).unwrap();
        let value: CString = CString::new(value).unwrap();
        unsafe { assert_eq!(fluid_settings_setstr(self.settings, name.as_ptr(), value.as_ptr()), 1); }

        // Verify TODO
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    pub fn set_mapping(&mut self, mapping: Vec<FluidSynthesizerMapping>) {
        self.mapping = mapping;
    }

    pub fn build(&mut self) {
        unsafe {
            self.synthesizer = Some(new_fluid_synth(self.settings));
            self.sequencer = Some(new_fluid_sequencer2(0));
            self.synthesizer_seq_id = fluid_sequencer_register_fluidsynth(self.sequencer.unwrap(), self.synthesizer.unwrap());
            debug!("Sequencer seq ID {}", self.synthesizer_seq_id);
            assert_eq!(fluid_sequencer_get_use_system_timer(self.sequencer.unwrap()), 0);

            // Unfortunately, sequencer precision is limited to 1ms/tick by FluidSynth
            fluid_sequencer_set_time_scale(self.sequencer.unwrap(), 1000.0);
            assert_eq!(fluid_sequencer_get_time_scale(self.sequencer.unwrap()), 1000.0);
        }
    }

    pub fn load_soundfont(&self, file: &str, offset: i32) {
        assert!(self.synthesizer.is_some());
        let file: CString = CString::new(file).unwrap();
        info!("Loading SoundFont...");
        let result = unsafe { fluid_synth_sfload(self.synthesizer.unwrap(), file.as_ptr(), 0) };
        assert_ne!(result, FLUID_FAILED, "Could not load SoundFont");
        info!("SoundFont loaded. Got ID {}", result);
        debug!("Setting bank offset");
        unsafe { fluid_synth_set_bank_offset(self.synthesizer.unwrap(), result, offset); }
    }

    pub fn debug_programs(&self) {
        for channel in 0..self.used_channels {
            let mut sfont_id: u32 = 0;
            let mut bank_num: u32 = 0;
            let mut preset_num: u32 = 0;
            unsafe { fluid_synth_get_program(self.synthesizer.unwrap(), channel as i32, &mut sfont_id as *mut u32, &mut bank_num as *mut u32, &mut preset_num as *mut u32); }
            debug!("Channel {}: {} - {}:{}", channel, sfont_id, bank_num, preset_num);
        }

    }
}

impl Drop for FluidSynthesizer {
    fn drop(&mut self) {
        unsafe {
            trace!("Dropping FluidSynthesizer");

            if let Some(sequencer) = self.sequencer {
                trace!(" => Dropping sequencer");
                delete_fluid_sequencer(sequencer);
            }

            if let Some(synthesizer) = self.synthesizer {
                trace!(" => Dropping synthesizer");
                delete_fluid_synth(synthesizer);
            }

            trace!(" => Dropping settings");
            delete_fluid_settings(self.settings);
        }
    }
}

fn assert_one_value_in_synth_setting(setting: &TOMLSynthSetting) {
    let mut i = 0;
    if setting.value_i.is_some() { i += 1 };
    if setting.value_f.is_some() { i += 1 };
    if setting.value_s.is_some() { i += 1 };
    assert_eq!(i, 1, "Expecting exactly one value");
}

fn assert_one_value_in_condition(condition: &TOMLCondition) {
    let mut i = 0;
    if condition.channel.is_some() { i += 1 };
    if condition.program.is_some() { i += 1 };
    assert_eq!(i, 1, "Expecting exactly one value");
}

fn generate_single_mapping(synthsettings: &TOMLSynth, synth: &mut FluidSynthesizer, condition: &TOMLCondition, destinations: &Vec<TOMLDestination>) -> FluidSynthesizerMapping {
    assert_one_value_in_condition(condition);
    let channel = condition.channel;
    let program = if condition.program.is_some() {
        Some(gm_instruments::program_nr_of(condition.program.as_ref().unwrap()))
    } else {
        None
    };

    let mut fluid_destinations = Vec::new();
    for destination in destinations {
        let destination_bank = if destination.bank.is_some() {
            destination.bank.unwrap()
        } else {
            0
        };
        let destination_program = if destination.program.is_some() {
            gm_instruments::program_nr_of(destination.program.as_ref().unwrap())
        } else {
            destination.program_nr.expect("Destination must contain program or program_nr") as u8
        };
        unsafe { fluid_synth_program_select(synth.synthesizer.unwrap(), synth.used_channels as i32, destination.soundfont, destination_bank, destination_program as u32); }

        fluid_destinations.push(synth.used_channels);
        synth.used_channels += 1;
    }

    let res = FluidSynthesizerMapping {
        condition: FluidSynthesizerCondition {
            channel,
            program,
        },
        destinations: fluid_destinations,
    };
    res
}

fn generate_mapping(synthsettings: &TOMLSynth, synth: &mut FluidSynthesizer) -> Vec<FluidSynthesizerMapping> {
    let mut res = Vec::new();
    for (_id, mapping) in &synthsettings.mapping {
        for condition in &mapping.condition {
            res.push(generate_single_mapping(synthsettings, synth, &condition, &mapping.destination));
        }
    }
    res
}

pub fn generate_fluid_synthesizers(settings: &TOMLRenderSettings, resources: &PathBuf) -> Vec<FluidSynthesizer> {
    let mut res = Vec::new();
    for (id, synthsettings) in &settings.synth {
        if synthsettings.synthtype != "fluidsynth" {
            continue;
        }
        info!("Building fluid synthesizer '{}'", id);
        let mut synth = FluidSynthesizer::new();
        synth.set_gain(synthsettings.gain);
        if synthsettings.setting.is_some() {
            for setting in synthsettings.setting.as_ref().unwrap() {
                assert_one_value_in_synth_setting(setting);
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
        if synthsettings.soundfont.is_some() {
            for soundfont in synthsettings.soundfont.as_ref().unwrap() {
                let mut soundfont_file = resources.clone();
                soundfont_file.push(&soundfont.file);
                let soundfont_file = soundfont_file.to_str().unwrap();
                info!("Loading soundfont '{}' with offset {}", soundfont_file, soundfont.offset);
                synth.load_soundfont(&soundfont_file, soundfont.offset);
            }
        }

        let mapping = generate_mapping(synthsettings, &mut synth);
        synth.set_mapping(mapping);
        synth.debug_programs();

        res.push(synth);
    }
    res
}
