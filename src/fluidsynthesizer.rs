use std::ffi::CString;
use std::path::PathBuf;

use std::os::raw::c_int;
use std::os::raw::c_double;

use fluidsynth_bindgen::*;

use types::*;

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
}

impl Drop for FluidSynthesizer {
    fn drop(&mut self) {
        unsafe {
            debug!("Dropping FluidSynthesizer");

            if let Some(sequencer) = self.sequencer {
                debug!(" Dropping sequencer");
                delete_fluid_sequencer(sequencer);
            }

            if let Some(synthesizer) = self.synthesizer {
                debug!(" Dropping synthesizer");
                delete_fluid_synth(synthesizer);
            }

            debug!(" Dropping settings");
            delete_fluid_settings(self.settings);
        }
    }
}

fn assert_one_value(setting: &TOMLSynthSetting) {
    let mut i = 0;
    if setting.value_i.is_some() { i += 1 };
    if setting.value_f.is_some() { i += 1 };
    if setting.value_s.is_some() { i += 1 };
    assert!(i == 1, "Expecting exactly one value");
}

pub fn generate_fluid_synthesizers(settings: &TOMLRenderSettings, resources: &PathBuf) -> Vec<FluidSynthesizer> {
    let mut res = vec!();
    for (id, synthsettings) in &settings.synth {
        if synthsettings.synthtype != "fluidsynth" {
            continue;
        }
        info!("Building fluid synthesizer '{}'", id);
        let mut synth = FluidSynthesizer::new();
        synth.set_gain(synthsettings.gain);
        if synthsettings.setting.is_some() {
            for setting in synthsettings.setting.as_ref().unwrap() {
                assert_one_value(setting);
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

        res.push(synth);
    }
    res
}
