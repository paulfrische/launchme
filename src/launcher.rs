pub struct Launcher {
    options: Vec<String>,
}

impl Launcher {
    pub fn new(options: Vec<impl ToString>) -> Self {
        let options = options.iter().map(|s| s.to_string()).collect();

        Self { options }
    }

    pub fn launch(&self) -> String {
        todo!()
    }
}
