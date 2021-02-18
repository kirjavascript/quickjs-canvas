use crate::sdl_env::SDLEnv;
use crate::text;
use crate::path::Path;
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2_unifont::renderer::SurfaceRenderer;

pub struct CanvasWindow {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    font_renderer: SurfaceRenderer,
    width: u32,
    height: u32,
    background_color: Color,
    fill_style: Color,
    stroke_style: Color,
    dirty: bool,
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

        let font_renderer = SurfaceRenderer::new(
            Color::RGBA(0, 0, 0, 0),
            Color::RGBA(0, 0, 0, 0),
        );

        Self {
            canvas,
            texture_creator,
            font_renderer,
            width,
            height,
            background_color: Color::RGB(255, 255, 255),
            fill_style: Color::RGB(0, 0, 0),
            stroke_style: Color::RGB(0, 0, 0),
            dirty: true,
        }
    }

    pub fn fill_path(&mut self, path: Path) {
        self.canvas.filled_polygon(
            path.get_x_points().as_slice(),
            path.get_y_points().as_slice(),
            self.fill_style
        ).unwrap();
    }

    pub fn clear_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.canvas.set_draw_color(self.background_color);
        if x == 0 && y == 0 && w as u32 == self.width && h as u32 == self.height {
            // will need to reset bg color
            self.canvas.clear();
        } else {
            self.canvas.fill_rect(Rect::new(x, y, w as u32, h as u32)).unwrap();

        }
        self.dirty = true;
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.canvas.set_draw_color(self.fill_style);
        self.canvas.fill_rect(Rect::new(x, y, w as u32, h as u32)).unwrap();
        self.dirty = true;
    }

    pub fn stroke_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.canvas.set_draw_color(self.stroke_style);
        self.canvas.draw_rect(Rect::new(x, y, w as u32, h as u32)).unwrap();
        self.dirty = true;
    }

    // TODO: positioning, styles
    pub fn fill_text(&mut self, text: String, x: i32, y: i32) {
        let mut screen = sdl2::surface::Surface::new(
            self.width,
            self.height,
            sdl2::pixels::PixelFormatEnum::RGBA8888,
        ).unwrap();

        self.font_renderer.fg_color = self.fill_style; // transparent BG
        // renderer.bg_color = Color::RGBA(255, 255, 255, 0); // transparent BG
        // self.font_renderer.bold = true;
        // self.font_renderer.scale = 1;
        self.font_renderer
            .draw(&text::to_canvas(&text))
            .unwrap()
            .blit(None, &mut screen, Rect::new(x, y, 0, 0)).unwrap();

        let text = self.texture_creator
            .create_texture_from_surface(screen)
            .unwrap();
        self.canvas.copy(&text, None, None).unwrap();
        self.dirty = true;
    }

    pub fn fill_style(&mut self, color: Color) {
        self.fill_style = color;
    }

    pub fn stroke_style(&mut self, color: Color) {
        self.stroke_style = color;
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width as u32;
        self.height = height as u32;
        self.canvas.window_mut().set_size(self.width, self.height)
            .expect("failed to set window size");
        self.canvas.set_draw_color(self.background_color);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn set_title(&mut self, text: String) {
        self.canvas.window_mut().set_title(&text).ok();
    }

    pub fn render(&mut self) {
        if self.dirty {
            self.canvas.present();
            self.dirty = false;
        }
    }

    pub fn window(&self) -> &Window {
        self.canvas.window()
    }
}
