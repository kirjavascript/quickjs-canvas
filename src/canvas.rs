use crate::sdl_env::SDLEnv;
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2_unifont::renderer::SurfaceRenderer;

pub struct CanvasWindow {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    width: u32,
    height: u32,
}

impl CanvasWindow {
    pub fn new(sdl_env: &SDLEnv, width: u32, height: u32) -> Self {
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
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        // TODO: support backgroundColor
        // TODO: change to ints
        if x == 0. && y == 0. && w as u32 == self.width && h as u32 == self.height {
            // will need to reset bg color
            self.canvas.clear();
        } else {
            self.canvas.set_draw_color(Color::RGB(255, 255, 255));
            self.canvas.fill_rect(Rect::new(x as i32, y as i32, w as u32, h as u32)).unwrap();

        }
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width as u32;
        self.height = height as u32;
        self.canvas.window_mut().set_size(self.width, self.height)
            .expect("failed to set window size");
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn set_title(&mut self, text: String) {
        self.canvas.window_mut().set_title(&text).ok();
    }

    pub fn render(&mut self) {
        self.canvas.present();
    }
}
