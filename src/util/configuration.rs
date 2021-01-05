use crate::model::state::State;
use wasm_bindgen::__rt::std::fs::{File, rename};
use wasm_bindgen::__rt::std::io::{BufReader, BufWriter};

const STATE_FILE: &str = "conf/state.conf";
const DEFAULT_FILE: &str = "conf/default.conf";

pub fn read_config() -> State {
    File::open(STATE_FILE)
        .or(File::open(DEFAULT_FILE))
        .and_then(|file|
            serde_json::from_reader(BufReader::new(file))
                .map_err(|err| std::io::Error::from(err) )
        )
        .unwrap_or(State::fallback())
}

pub fn write_config(state: &State) {
    let temp_file = format!("{}~", STATE_FILE);
    File::create(&temp_file)
        .and_then(|file|
            serde_json::to_writer_pretty(BufWriter::new(file), state)
                .map_err(|err| std::io::Error::from(err))
        )
        .and_then(|_| rename(temp_file.as_str(), STATE_FILE))
        .expect(format!("Couldn't write state to {}", STATE_FILE).as_str())
}
