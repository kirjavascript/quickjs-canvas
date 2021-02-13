use pathfinder_canvas::{
    Canvas,
    CanvasRenderingContext2D,
    CanvasFontContext,
    Vector2F,
    RectF,
    ColorU,
    Transform2F,
};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use sdl2::VideoSubsystem;
use sdl2::video::{Window, GLContext};

pub struct CanvasWindow {
    ctx: CanvasRenderingContext2D,
    // scene: Scene,
    gl_context: GLContext, // need to make sure this isn't dropped
    renderer: Renderer<GLDevice>,
    window: Window,
    window_size: Vector2F,
    font_context: CanvasFontContext,
    dirty: bool,
}

impl CanvasWindow {
    pub fn new(video: &VideoSubsystem) -> Self {

        let window_size = vec2f(300., 150.);
        let window = video.window("quickjs-canvas", window_size.x() as u32, window_size.y() as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        // create the GL context for the window
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

        // create a Pathfinder renderer
        let resource_loader = EmbeddedResourceLoader::new();
        let device = GLDevice::new(GLVersion::GL3, 0);
        let options = RendererOptions {
            background_color: Some(ColorF::white()),
            ..RendererOptions::default()
        };
        let renderer = Renderer::new(device, &resource_loader, DestFramebuffer::full_window(window_size.to_i32()), options);

        let font_context = CanvasFontContext::from_system_source();

        let ctx = Canvas::new(window_size).get_context_2d(font_context.clone());

        Self {
            window,
            ctx,
            renderer,
            window_size,
            gl_context,
            font_context,
            dirty: true,
        }
    }

    pub fn fill_text(&mut self, text: String, x: f64, y: f64) {
        // self.ctx.set_font("Hack-Regular");
        self.ctx.set_font_size(32.0);
        self.ctx.set_fill_style(ColorU::black());
        self.ctx.fill_text(&text, vec2f(x as f32, y as f32));
        self.dirty = true;
    }

    pub fn clear_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        let rect = RectF::from_points(vec2f(x as f32, y as f32), vec2f(w as f32, h as f32));
        self.ctx.set_fill_style(ColorU::white());
        self.ctx.fill_rect(rect);
        self.dirty = true;
    }

    pub fn render(&mut self) {
        if self.dirty {
            self.render_base();
            self.dirty = false;
        }
    }

    fn render_base(&mut self) {

        let next_ctx = Canvas::new(self.window_size)
            .get_context_2d(self.font_context.clone());
        let last_ctx = std::mem::replace(&mut self.ctx, next_ctx);

        let scene = last_ctx.into_canvas().into_scene();

        // let canvas_clone = Canvas::from_scene(scene.clone());
        // let pattern = self.ctx.create_pattern_from_canvas(
        //     canvas_clone,
        //     Transform2F::from_rotation(0.),
        // );

        // self.ctx.draw_image(pattern, self.window_size);

        self.ctx.draw_image(Canvas::from_scene(scene.clone()), self.window_size);

        // set the current GL context
        self.window.gl_make_current(&self.gl_context).unwrap();

        // create the scene proxy and render
        let scene = SceneProxy::from_scene(scene, RayonExecutor);
        scene.build_and_render(&mut self.renderer, BuildOptions::default());
        self.window.gl_swap_window();
    }
}
