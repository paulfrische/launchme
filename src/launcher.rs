use sdl2::{
    event::Event,
    keyboard::{Keycode, TextInputUtil},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    ttf,
    video::Window,
    Sdl, VideoSubsystem,
};

pub struct Launcher {
    options: Vec<String>,
    query: String,
    context: Sdl,
    video: VideoSubsystem,
    canvas: Canvas<Window>,
    ttf: ttf::Sdl2TtfContext,
    text: TextInputUtil,
}

impl Launcher {
    pub fn new(options: Vec<impl ToString>) -> Self {
        let options = options.iter().map(|s| s.to_string()).collect();

        let context = sdl2::init().expect("failed to create sdl context");

        let video = context
            .video()
            .expect("failed to initialize sdl video subsystem");

        let window = video
            .window("", 1000, 700) // TODO: calculate dimensions or get them via config
            .position_centered()
            .borderless()
            .build()
            .expect("failed to create window");

        let canvas = window
            .into_canvas()
            .build()
            .expect("failed to create canvas");

        let ttf = ttf::init().expect("failed to create sdl ttf context");

        let text = video.text_input();

        Self {
            options,
            query: String::from(""),
            context,
            video,
            canvas,
            ttf,
            text,
        }
    }

    // TODO: better error handling
    pub fn launch(&mut self) -> String {
        let creator = self.canvas.texture_creator();

        // TODO: use `font_kit` crate to find font families by name
        // https://docs.rs/font-kit/latest/font_kit/index.html
        // https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html

        // TODO: make font configurable
        let font = self
            .ttf
            .load_font("/usr/share/fonts/TTF/IosevkaNerdFont-Regular.ttf", 16)
            .expect("failed to load font");

        // TODO: handle theming
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self
            .context
            .event_pump()
            .expect("failed to create event pump");

        self.text.start();
        'main_loop: loop {
            self.canvas.set_draw_color(Color::BLACK);
            self.canvas.clear();

            for event in event_pump.poll_iter() {
                // TODO: handle input
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main_loop,
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Backspace => {
                                    if !self.query.is_empty() {
                                        let _ = self.query.pop();
                                    }
                                }
                                Keycode::Return => {
                                    break 'main_loop
                                }
                                _ => (),
                            }
                        }
                    }
                    Event::TextInput { text, .. } => {
                        self.query += &text;
                    }
                    _ => {}
                }
            }

            // NOTE: to render text follow this procedure:
            // - load a font via Sdl2TtfContext
            // - render text to a surface using Font::render
            // - create a target rect (use Surface::width and Surface::height)
            // - create a texture using TextureCreator
            // - copy the texture into the target rect using Canvas::copy

            if !self.query.is_empty() {
                // `blended` means something like antialiasing
                let surface = font
                    .render(&self.query)
                    .blended(Color::WHITE) // TODO: theming
                    .expect("failed to render text");

                let text_rect = Rect::new(12, 12, surface.width(), surface.height());

                let texture = creator
                    .create_texture_from_surface(&surface)
                    .expect("failed to create texture");

                let _ = self.canvas.copy(&texture, None, Some(text_rect));
            }

            self.canvas.present();
        }

        self.query.clone()
    }
}
