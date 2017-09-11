use std::ffi::CString;

use std::os::raw::c_int;

use fluidsynth_bindgen::*;

use types::FluidSynthesizer;

impl FluidSynthesizer {
    fn new() -> FluidSynthesizer {
        unsafe {
            FluidSynthesizer {
                settings: new_fluid_settings(),
                synth: None,
            }
        }
    }

    fn settings_setint(&self, name: &str, value: c_int) {
        let name: CString = CString::new(name).unwrap();
        unsafe { fluid_settings_setint(self.settings, name.as_ptr(), value); }

        // Verify
        let mut set_value: i32 = 0;
        unsafe { fluid_settings_getint(self.settings, name.as_ptr(), &mut set_value as *mut i32); }
        assert_eq!(set_value, value);
    }

    fn build(&mut self) {
        unsafe { self.synth = Some(new_fluid_synth(self.settings)); }
    }
}

impl Drop for FluidSynthesizer {
    fn drop(&mut self) {
        unsafe {
            if let Some(synth) = self.synth {
                delete_fluid_synth(synth);
            }

            delete_fluid_settings(self.settings);
        }
    }
}
