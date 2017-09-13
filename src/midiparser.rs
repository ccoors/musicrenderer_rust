use std::cmp;

use ghakuf::messages::*;
use ghakuf::reader::*;

use types::*;

impl MIDIHandlerData {
    pub fn pulse_to_time(&self, pulse: u64) -> f64 {
        assert!(self.tempo_changes.len() > 0);
        let mut acc = 0.0;
        let mut last_change: u64 = 0;
        let mut current_uspp = 0.0;
        let mut last_added = false;
        for t in &self.tempo_changes {
            if pulse >= t.pulse {
                acc += ((t.pulse - last_change) as f64) * current_uspp;
                current_uspp = t.us_per_pulse;
                last_change = t.pulse;
            } else {
                acc += ((pulse - last_change) as f64) * current_uspp;
                last_added = true;
                break;
            }
        }
        if !last_added {
            acc += ((pulse - last_change) as f64) * current_uspp;
        }
        acc
    }

    pub fn max_time(&self) -> f64 {
        self.pulse_to_time(self.max_pulse)
    }

    pub fn add_delta_time(&mut self, delta_time: u32) {
        self.current_pulse += u64::from(delta_time);
        self.max_pulse = cmp::max(self.current_pulse, self.max_pulse);
    }

    pub fn reset_current_pulse(&mut self) {
        self.current_pulse = 0;
    }
}


impl Handler for MIDIHandler {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        let debug_header = (format, track, time_base);
        trace!("SMF header: {:?}", debug_header);
        unsafe { (*self.data).pulses_per_quarter_note = time_base; }
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        let debug_event = (delta_time, event, data);
        trace!("SMF meta event: {:?}", debug_event);
        unsafe { (*self.data).add_delta_time(delta_time); }
        match event {
            &MetaEvent::SetTempo => {
                unsafe { assert_ne!((*self.data).pulses_per_quarter_note, 0); }
                assert_eq!(data.len(), 3);

                let us_per_qn = ((data[0] as u32) << 16) + ((data[1] as u32) << 8) + (data[2] as u32);
                let bpm = 60000000.0 / us_per_qn as f64;
                let uspp = unsafe { (us_per_qn as f64 / (*self.data).pulses_per_quarter_note as f64) };

                debug!("New tempo: {} USPQN / {:.*} BPM / {} USPP", us_per_qn, 0, bpm, uspp);
                unsafe {
                    (*self.data).tempo_changes.push(MIDITempoChange {
                        pulse: (*self.data).current_pulse,
                        us_per_pulse: uspp,
                    });
                    trace!("Current tempo changes: {:?}", (*self.data).tempo_changes);
                }
            }
            _ => {}
        }
    }

    fn midi_event(&mut self, delta_time: u32, _event: &MidiEvent) {
        //        let debug_event = (delta_time, event);
        //        trace!("SMF midi event: {:?}", debug_event);
        unsafe {
            (*self.data).add_delta_time(delta_time);
        }

        // TODO: Schedule event in synth sequencer
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        let debug_event = (delta_time, event, data);
        trace!("SMF sysex event: {:?}", debug_event);
        unsafe { (*self.data).add_delta_time(delta_time); }
    }

    fn track_change(&mut self) {
        trace!("SMF track change");
        unsafe { (*self.data).reset_current_pulse(); }
    }
}
