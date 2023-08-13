mod system;
mod launcher;

fn main() {
    let executables = system::executables();
    if let Some(program) = launcher::Launcher::new(executables).launch() {
        system::run(program);
    }
}
