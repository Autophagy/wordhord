use clap::{arg, command};
use std::env;
use std::error::Error;

use config::Config;

mod app;
mod config;
mod templates;

fn main() -> Result<(), Box<dyn Error>> {
    let m = command!().arg(arg!([CONFIG]).required(true)).get_matches();

    let config = Config::new(m.get_one::<String>("CONFIG").unwrap())?;

    app::create_build_dir(&config)?;
    app::build_wordhord(&config)?;

    Ok(())
}
