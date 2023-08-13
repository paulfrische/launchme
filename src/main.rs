mod system;
mod launcher;

fn main() {
    let executables = system::executables();
    let program = launcher::Launcher::new(executables).launch();
    system::run(program);
}
