use std::{env, fs, process};

pub fn executables() -> Vec<String> {
    let path = match env::var("PATH") {
        Ok(val) => val,
        Err(_e) => "".into(),
    };

    let path: Vec<String> = path.split(":").map(|p| String::from(p)).collect();
    let mut executables: Vec<String> = Vec::new();

    path.iter().for_each(|p| match fs::read_dir(p) {
        Ok(d) => {
            d.for_each(|e| match e {
                Ok(e) => match e.file_name().into_string() {
                    Ok(name) => executables.push(name),
                    Err(_) => return,
                },
                Err(_) => return,
            });
        }
        Err(_e) => return,
    });

    executables
}

pub fn run(program: impl ToString) {
    let _ = process::Command::new::<String>(program.to_string()).output().expect(&format!("failed to run {}", program.to_string()));
}
