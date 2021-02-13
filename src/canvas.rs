use sdl2::VideoSubsystem;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::gfx::primitives::DrawRenderer;

pub struct CanvasWindow {
    dirty: bool,
    canvas: Canvas<Window>,
}

impl CanvasWindow {
    pub fn new(video: &VideoSubsystem) -> Self {
        let window = video.window("quickjs-canvas", 300, 150)
            .position_centered()
            .opengl()
            .build()
            .expect("could not open SDL window");

        let mut canvas = window
            .into_canvas()
            .build()
            .expect("could not cfeate canvas from window");

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        Self {
            dirty: false,
            canvas,
        }
    }

    pub fn fill_text(&mut self, text: String, x: f64, y: f64) {
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(Rect::new(x as i32, y as i32, 20, 20)).unwrap();
        self.dirty = true;
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.fill_rect(Rect::new(x as i32, y as i32, w as u32, h as u32)).unwrap();
        self.dirty = true;
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
