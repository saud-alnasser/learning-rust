use std::{error::Error, fs};

use crate::{parse::Config, query};

pub fn run(config: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    Ok(query::search(&config.query, &content, !config.ignore_case))
}
