mod arguments;
mod launcher;
mod system;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let name = args.get(0).expect("failed to retrieve arguments").clone();
    let args = arguments::Args::parse(args);
    match args {
        arguments::Args::Help => {
            println!("Usage: {name} [optional: path to config file]");
            return;
        }
        arguments::Args::Run(r) => {
            let executables = system::executables();
            if let Some(program) = launcher::Launcher::new(executables).launch() {
                system::run(program);
            }
        }
    }
}
