use font_kit::{handle::Handle, source::SystemSource};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use sdl2::{
    event::Event,
    keyboard::{Keycode, TextInputUtil},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    ttf,
    video::Window,
    Sdl,
};

use super::config;

impl From<config::Color> for Color {
    fn from(value: config::Color) -> Self {
        let config::Color::RGB(r, g, b) = value;
        return Self::RGB(r, g, b);
    }
}

pub struct Launcher {
    options: Vec<String>,
    config: config::Config,
    query: String,
    context: Sdl,
    canvas: Canvas<Window>,
    ttf: ttf::Sdl2TtfContext,
    text: TextInputUtil,
}

impl Launcher {
    pub fn new(options: Vec<impl ToString>, config: config::Config) -> Self {
        let options = options.iter().map(|s| s.to_string()).collect();

        let context = sdl2::init().expect("failed to create sdl context");

        let video = context
            .video()
            .expect("failed to initialize sdl video subsystem");

        let window = video
            .window("", config.width as u32, config.height as u32)
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
            config,
            query: String::from(""),
            context,
            canvas,
            ttf,
            text,
        }
    }

    // TODO: better error handling
    pub fn launch(&mut self) -> Option<String> {
        let result: Option<String>;
        let creator = self.canvas.texture_creator();

        // TODO: make font configurable
        let mut font_path = String::from("");
        let source = SystemSource::new();
        let fonts = source
            .all_fonts()
            .expect("failed to get a list of all fonts");
        let selected_font = fonts.get(0).expect("no fonts found");
        if let Handle::Path { path, .. } = selected_font {
            font_path = path.to_str().expect("failed to get font path").to_string();
        }

        let font = self
            .ttf
            .load_font(font_path, 16)
            .expect("failed to load font");

        self.canvas.set_draw_color(self.config.background);
        self.canvas.clear();
        self.canvas.present();

        let mut event_pump = self
            .context
            .event_pump()
            .expect("failed to create event pump");

        let matcher = SkimMatcherV2::default();

        self.text.start();
        'main_loop: loop {
            self.canvas.set_draw_color(self.config.background);
            self.canvas.clear();

            // filter all options
            let mut filtered_options: Vec<String> = self
                .options
                .iter()
                .filter(|s| matcher.fuzzy_indices(s, &self.query).is_some())
                .map(|s| String::from(s))
                .collect();

            filtered_options.sort_by(|a, b| {
                a.len()
                    .partial_cmp(&b.len())
                    .expect("couldn't order strings")
            });

            for event in event_pump.poll_iter() {
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
            let mut cursor = Rect::new(
                self.config.padding,
                self.config.padding,
                5,
                font.height() as u32,
            );
            if !self.query.is_empty() {
                // `blended` means something like antialiasing
                let surface = font
                    .render(&self.query)
                    .blended(self.config.input)
                    .expect("failed to render text");

                let rect = Rect::new(
                    self.config.padding,
                    self.config.padding,
                    surface.width(),
                    surface.height(),
                );

                let texture = creator
                    .create_texture_from_surface(&surface)
                    .expect("failed to create texture");

                cursor = Rect::new(rect.right(), rect.y, 5, rect.height());

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }

            let mut options_to_render = ((self.config.height - 2 * self.config.padding)
                / (font.height() + self.config.line_spacing)
                - 1) as usize;

            options_to_render = options_to_render.min(filtered_options.len());

            for i in 0..options_to_render {
                let offset = self.config.padding
                    + (font.height() + self.config.line_spacing) * (i + 1) as i32;
                let surface = font
                    .render(
                        filtered_options
                            .get(i as usize)
                            .expect("failed to calculate valid index"),
                    )
                    .blended(Color::GRAY)
                    .expect("failed to render options");

                let rect = Rect::new(
                    self.config.padding,
                    offset,
                    surface.width(),
                    surface.height(),
                );
                let texture = creator
                    .create_texture_from_surface(surface)
                    .expect("failed to create texture");

                let _ = self.canvas.copy(&texture, None, Some(rect));
            }
            self.canvas.set_draw_color(self.config.cursor);
            let _ = self.canvas.fill_rect(cursor);
            self.canvas.present();
        }

        self.canvas.window_mut().hide();

        result
    }
}
