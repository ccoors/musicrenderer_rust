use std::ffi::CString;

use std::os::raw::c_int;
use std::os::raw::c_double;

use fluidsynth_bindgen::*;

use types::FluidSynthesizer;

impl FluidSynthesizer {
    pub fn new() -> FluidSynthesizer {
        unsafe {
            FluidSynthesizer {
                settings: new_fluid_settings(),
                synth: None,
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

    pub fn build(&mut self) {
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
