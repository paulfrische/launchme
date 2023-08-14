use std::fs;

use clap::Parser;

mod arguments;
mod launcher;
mod system;

fn main() {
    let args = arguments::Arguments::parse();
    let mut config = launcher::config::Config::default();
    if let Some(c) = args.config {
        if let Ok(config_content) = fs::read_to_string(c.clone()) {
            config = launcher::config::Config::load(config_content)
                .unwrap_or(launcher::config::Config::default());
        } else {
            let _ = fs::write(c.clone(), launcher::config::Config::default_content());
        }
    }
    let executables = system::executables();
    if let Some(program) = launcher::Launcher::new(executables, config).launch() {
        system::run(program);
    }
}
