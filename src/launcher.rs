use sdl2::{
    event::Event,
    keyboard::{Keycode, TextInputUtil},
    pixels::Color,
    rect::Rect,
    ttf, Sdl, VideoSubsystem,
};

pub struct Launcher {
    options: Vec<String>,
    context: Sdl,
    video: VideoSubsystem,
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
        let ttf = ttf::init().expect("failed to create sdl ttf context");
        let text = video.text_input();

        Self {
            options,
            context,
            video,
            ttf,
            text,
        }
    }

    pub fn launch(&self) -> String {
        // NOTE: to render text follow this procedure:
        // - load a font via Sdl2TtfContext
        // - render text to a surface using Font::render
        // - create a target rect (use Surface::width and Surface::height)
        // - create a texture using TextureCreator
        // - copy the texture into the target rect using Canvas::copy

        // TODO: use `font_kit` crate to find font families by name
        // https://docs.rs/font-kit/latest/font_kit/index.html
        // https://docs.rs/font-kit/latest/font_kit/handle/enum.Handle.html

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

        let creator = canvas.texture_creator();

        // TODO: make font configurable
        let font = self
            .ttf
            .load_font("/usr/share/fonts/TTF/IosevkaNerdFont-Regular.ttf", 16)
            .expect("failed to load font");


        let surface = font
            .render("Hello World!")
            .solid(Color::WHITE)
            .expect("failed to render text");

        let text_rect = Rect::new(12, 12, surface.width(), surface.height());

        let texture = creator
            .create_texture_from_surface(&surface)
            .expect("failed to create texture");

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

            let _ = canvas.copy(&texture, None, Some(text_rect));

            canvas.present();
        }

        "".into()
    }
}
