use std::fs;
use std::io;
use std::path::Path;

use crate::args::Config;

pub fn get_template(path: &Path) -> Result<String, io::Error> {
    let tmpl = fs::read_to_string(&path)?;
    Ok(tmpl)
}

pub fn get_config(path: &Path) -> Result<Config, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(&path)?;
    let data: Config = toml::from_str(&raw)?;

    Ok(data)
}
