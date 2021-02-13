use crate::sdl_env::SDLEnv;
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2_unifont::renderer::SurfaceRenderer;

pub struct CanvasWindow {
    dirty: bool,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    width: u32,
    height: u32,
}

impl CanvasWindow {
    pub fn new(sdl_env: &SDLEnv) -> Self {
        let width = 300;
        let height = 150;
        let window = sdl_env.video.window("quickjs-canvas", width, height)
            .position_centered()
            .opengl()
            .build()
            .expect("could not open SDL window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not create canvas from window");

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        let texture_creator = canvas.texture_creator();


        Self {
            dirty: false,
            canvas,
            width,
            height,
            texture_creator,
        }
    }

    pub fn fill_text(&mut self, text: String, x: f64, y: f64) {
        let mut renderer =
            SurfaceRenderer::new(Color::RGB(255, 0, 0), Color::RGBA(0, 0, 0, 0));

        let mut screen = sdl2::surface::Surface::new(
            self.width,
            self.height,
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        ).unwrap();

        renderer.bg_color = Color::RGBA(255, 255, 255, 0); // transparent BG
        renderer.bold = false;
        renderer.scale = 1;
        renderer.draw(&text).unwrap().blit(None, &mut screen, Rect::new(x as _, y as _, 0, 0)).unwrap();

        let text = self.texture_creator
            .create_texture_from_surface(screen)
            .unwrap();
        self.canvas.copy(&text, None, None).unwrap();

        self.dirty = true;
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        // TODO: check if equal to size on canvas and call canvas.clear()
        // TODO: support backgroundColor
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(x as i32, y as i32, w as u32, h as u32)).unwrap();
        self.dirty = true;
    }

    pub fn set_title(&mut self, text: String) {
        self.canvas.window_mut().set_title(&text).expect("unable to set title");
    }

    pub fn render(&mut self) {
        if self.dirty {
            self.render_base();
            self.dirty = false;
        }
    }

    fn render_base(&mut self) {
        self.canvas.present();
    }
}
