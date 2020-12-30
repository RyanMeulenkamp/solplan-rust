use crate::model::state::State;
use wasm_bindgen::__rt::std::env::current_dir;
use wasm_bindgen::__rt::std::fs::{File, rename};
use wasm_bindgen::__rt::std::io::{BufReader, BufWriter};

pub fn read_config() -> State {
    match current_dir() {
        Ok(dir) => log::info!("Current working directory: {}", dir.as_path().display()),
        Err(_) => log::info!("Unable to garner current directory.")
    }
    match File::open("conf/state.conf") {
        Ok(file) => {
            match serde_json::from_reader(BufReader::new(file)) {
                Ok(state) => state,
                Err(_) => {
                    log::warn!("File conf/state.conf could not be parsed!");
                    State::fallback()
                }
            }
        },
        Err(_) => {
            log::warn!("File conf/state.conf not found!");
            match File::open("conf/default.conf") {
                Ok(file) => {
                    match serde_json::from_reader(BufReader::new(file)) {
                        Ok(state) => state,
                        Err(_) => {
                            log::warn!("File conf/default.conf could not be parsed!");
                            State::fallback()
                        }
                    }
                }
                Err(_) => {
                    log::warn!("File conf/default.conf not found!");
                    State::fallback()
                }
            }
        }
    }
}

pub fn write_config(state: &State) {
    match File::create("conf/state.conf~") {
        Ok(file) => {
            match serde_json::to_writer_pretty(BufWriter::new(file), state) {
                Ok(_) => {
                    match rename("conf/state.conf~", "conf/state.conf") {
                        Ok(_) => log::info!("Written settings to conf/state.conf successfully"),
                        Err(_) => log::warn!("Failed to write settings to conf/state.conf")
                    };
                },
                Err(_) => log::warn!("Failed to write settings to conf/state.conf~"),
            }
        }
        Err(_) => log::error!("Failed to create new config file.")
    }
}
