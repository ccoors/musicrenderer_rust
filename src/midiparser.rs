use ghakuf::messages::*;
use ghakuf::reader::*;

use types::*;

impl Handler for MIDIHandler {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        let header = (format, track, time_base);
        trace!("SMF header: {:?}", header);
    }

    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        let event = (delta_time, event, data);
        trace!("SMF meta event: {:?}", event);
    }

    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        let event = (delta_time, event);
        trace!("SMF midi event: {:?}", event);

        // TODO: Schedule event in synth sequencer
    }

    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
        let event = (delta_time, event, data);
        trace!("SMF sysex event: {:?}", event);
    }

    fn track_change(&mut self) {
        trace!("SMF track change");
    }
}
