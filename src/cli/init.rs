use std::{io::Write, path::Path};

use anyhow::Result;

const INDEX_TS: &str = include_str!("../initial_files/index.ts");
const INDEX_D_TS: &str = include_str!("../initial_files/index.d.ts");

pub fn initialize_config(path: &Path) -> Result<()> {
    let index_ts_file_path = path.join("index.ts");
    let mut index_ts_file = match std::fs::File::create_new(index_ts_file_path) {
        Ok(file) => file,
        Err(_error) => {
            log::error!("The file 'index.ts' already exits.");
            std::process::exit(1);
        }
    };
    index_ts_file.write_all(INDEX_TS.as_bytes())?;

    let index_d_ts_path = path.join("index.d.ts");
    let mut index_d_ts_file = match std::fs::File::create_new(index_d_ts_path) {
        Ok(file) => file,
        Err(_error) => {
            log::error!("The file 'index.d.ts' already exits.");
            std::process::exit(1);
        }
    };
    index_d_ts_file.write_all(INDEX_D_TS.as_bytes())?;

    Ok(())
}
