use ghakuf::messages::*;
use ghakuf::reader::*;

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
