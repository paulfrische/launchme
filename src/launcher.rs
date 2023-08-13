use sdl2::{event::Event, keyboard::Keycode, pixels::Color, Sdl, VideoSubsystem};

pub struct Launcher {
    options: Vec<String>,
    context: Sdl,
    video: VideoSubsystem,
}

impl Launcher {
    pub fn new(options: Vec<impl ToString>) -> Self {
        let options = options.iter().map(|s| s.to_string()).collect();
        let context = sdl2::init().expect("failed to create sdl context");
        let video = context
            .video()
            .expect("failed to initialize sdl video subsystem");

        Self {
            options,
            context,
            video,
        }
    }

    pub fn launch(&self) -> String {
        // TODO: better error handling
        let window = self
            .video
            .window("", 1000, 700) // TODO: calculate dimensions or get them via config
            .position_centered()
            .borderless()
            .build()
            .expect("failed to create window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("failed to create canvas");

        // TODO: handle theming
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        let mut event_pump = self
            .context
            .event_pump()
            .expect("failed to create event pump");

        'main_loop: loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main_loop,
                    _ => {}
                }
            }

            // TODO: handle input

            canvas.present();
        }

        "".into()
    }
}
