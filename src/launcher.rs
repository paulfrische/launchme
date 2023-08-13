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

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 700;
const PADDING: i32 = 12;
const LINE_SPACING: i32 = 6;

pub struct Launcher {
    options: Vec<String>,
    query: String,
    context: Sdl,
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
            .window("", WIDTH, HEIGHT) // TODO: calculate dimensions or get them via config
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
            canvas,
            ttf,
            text,
        }
    }

    // TODO: better error handling
    pub fn launch(&mut self) -> Option<String> {
        let mut result: Option<String>;
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

            // filter all options
            let mut filtered_options: Vec<String> = self
                .options
                .iter()
                .filter(|s| s.starts_with(&self.query))
                .map(|s| String::from(s))
                .collect();

            filtered_options.sort_by(|a, b| {
                a.len()
                    .partial_cmp(&b.len())
                    .expect("couldn't order strings")
            });

            for event in event_pump.poll_iter() {
                // TODO: handle input
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        result = None;
                        break 'main_loop;
                    }
                    Event::KeyDown { keycode, .. } => {
                        if let Some(key) = keycode {
                            match key {
                                Keycode::Backspace => {
                                    if !self.query.is_empty() {
                                        let _ = self.query.pop();
                                    }
                                }
                                Keycode::Return => {
                                    result = Some(
                                        filtered_options.get(0).unwrap_or(&self.query).to_string(),
                                    );
                                    break 'main_loop;
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

            // TODO: make cursor configurable
            let mut cursor = Rect::new(PADDING, PADDING, 5, font.height() as u32);
            if !self.query.is_empty() {
                // `blended` means something like antialiasing
                let surface = font
                    .render(&self.query)
                    .blended(Color::CYAN) // TODO: theming
                    .expect("failed to render text");

                // TODO: make padding configurable
                let rect = Rect::new(PADDING, PADDING, surface.width(), surface.height());

                let texture = creator
                    .create_texture_from_surface(&surface)
                    .expect("failed to create texture");

                cursor = Rect::new(rect.right(), rect.y, 5, rect.height());

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }

            let mut options_to_render =
                ((HEIGHT as i32 - 2 * PADDING) / (font.height() + LINE_SPACING) - 1) as usize;

            options_to_render = options_to_render.min(filtered_options.len());

            for i in 0..options_to_render {
                let offset = PADDING + (font.height() + LINE_SPACING) * (i + 1) as i32;
                let surface = font
                    .render(
                        filtered_options
                            .get(i as usize)
                            .expect("failed to calculate valid index"),
                    )
                    .blended(Color::GRAY)
                    .expect("failed to render options");

                let rect = Rect::new(PADDING, offset, surface.width(), surface.height());
                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("failed to create texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }
            self.canvas.set_draw_color(Color::WHITE);
            let _ = self.canvas.fill_rect(cursor);
            self.canvas.present();
        }

        self.canvas.window_mut().hide();

        result
    }
}
