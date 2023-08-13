pub enum Args {
    Help,
    Run(Run),
}

#[derive(Default, Debug)]
pub struct Run {
    pub config_destination: String,
}

impl Args {
    pub fn parse(args: Vec<String>) -> Self {
        if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
            return Self::Help;
        }

        if let Some(destination) = args.get(1) {
            return Self::Run(Run {
                config_destination: destination.clone(),
            });
        }

        Self::Run(Run::default())
    }
}
