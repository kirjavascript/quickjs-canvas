use pathfinder_canvas::{
    Canvas, CanvasRenderingContext2D, CanvasFontContext, Vector2I
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
    window_size: Vector2I,
}

impl CanvasWindow {
    pub fn new(video: &VideoSubsystem) -> Self {

        let window_size = vec2i(300, 150);
        let window = video.window("quickjs-canvas", window_size.x() as u32, window_size.y() as u32)
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
        let renderer = Renderer::new(device, &resource_loader, DestFramebuffer::full_window(window_size), options);

        let ctx = Canvas::new(window_size.to_f32()).get_context_2d(CanvasFontContext::from_system_source());

        Self {
            window,
            ctx,
            renderer,
            window_size,
            gl_context,
        }
    }

    pub fn fill_text(&mut self, text: String, x: f64, y: f64) {
        self.ctx.set_font("Hack-Regular");
        self.ctx.set_font_size(32.0);
        self.ctx.fill_text(&text, vec2f(x as f32, y as f32));
    }

    pub fn render(&mut self) {
        // TODO: make this function less shit

        // create a fake default that self can hold while we grab self.ctx to play with
        let fake = Canvas::new(vec2f(0.,0.))
            .get_context_2d(CanvasFontContext::from_system_source());
        let ctx = std::mem::replace(&mut self.ctx, fake);

        // extract a scene
        let scene = ctx.into_canvas().into_scene();
        let scene_clone = scene.clone();

        // use the clone to restore self.ctx
        self.ctx = Canvas::from_scene(scene_clone)
            .get_context_2d(CanvasFontContext::from_system_source());

        // set the current GL context
        self.window.gl_make_current(&self.gl_context).unwrap();

        // create the scene proxy and render
        let scene = SceneProxy::from_scene(scene, RayonExecutor);
        scene.build_and_render(&mut self.renderer, BuildOptions::default());
        self.window.gl_swap_window();
    }
}
