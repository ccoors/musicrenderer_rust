extern crate toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use types::*;

pub fn read_input_file(options: Options) -> RenderSettings {
    let input_file = Path::new(&options.input);
    let mut file = File::open(input_file.to_str().unwrap()).unwrap();
    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Could not read input file!");
    }

    let render_settings: OptionalRenderSettings = toml::from_str(&contents).unwrap();
    info!("Optional Render settings: {:?}", render_settings);
    to_render_settings(render_settings, input_file.parent().unwrap().to_path_buf())
}
