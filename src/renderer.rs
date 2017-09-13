extern crate time;

use std::path::PathBuf;

use ghakuf::reader::*;

use types;
use fluidsynthesizer;

pub fn process_render_settings(render_settings: &types::TOMLRenderSettings, resources: &PathBuf) {
    let mut midi_file = render_settings.input_path.clone();
    midi_file.push(&render_settings.input_file);
    let mut handler_data = types::MIDIHandlerData {
        fluid_synthesizers: Vec::new(),
        pulses_per_quarter_note: 0,
        tempo_changes: Vec::new(),
        current_pulse: 0,
        max_pulse: 0,
    };

    {
        let handler = Box::new(types::MIDIHandler {
            data: &mut handler_data as *mut types::MIDIHandlerData,
        });

        info!("Generating FluidSynth synthesizers...");
        let fluid_synthesizers = fluidsynthesizer::generate_fluid_synthesizers(&render_settings, resources);
        let elements = fluid_synthesizers.len();
        info!("Generated {} FluidSynth synthesizer{}", elements, if elements == 1 { "" } else { "s" });

        unsafe { (*handler.data).fluid_synthesizers = fluid_synthesizers; }
        let mut reader = Reader::new(
            handler,
            &midi_file.to_str().unwrap(),
        ).unwrap();

        info!("Parsing MIDI file");
        let _ = reader.read();
    }

    info!("MIDI length: {}", time::Duration::microseconds(handler_data.max_time() as i64));
}
