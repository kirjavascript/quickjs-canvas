use pathfinder_canvas::{
    Canvas, CanvasRenderingContext2D, CanvasFontContext, TextAlign
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
use sdl2::video::Window;

pub struct CanvasWindow {
    window: Window,
    ctx: CanvasRenderingContext2D,
    renderer: Renderer<GLDevice>,
}

impl CanvasWindow {
    pub fn new(video: &VideoSubsystem) -> Self {

        let window_size = vec2i(300, 150);
        let window = video.window("quickjs-canvas", window_size.x() as u32, window_size.y() as u32)
            .opengl()
            .build()
            .unwrap();

        // Create the GL context, and make it current.
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
        window.gl_make_current(&gl_context).unwrap();

        // Create a Pathfinder renderer.
        let resource_loader = EmbeddedResourceLoader::new();
        let device = GLDevice::new(GLVersion::GL3, 0);
        let options = RendererOptions {
            background_color: Some(ColorF::white()),
            ..RendererOptions::default()
        };
        let mut renderer = Renderer::new(device, &resource_loader, DestFramebuffer::full_window(window_size), options);

        let mut ctx = Canvas::new(window_size.to_f32()).get_context_2d(CanvasFontContext::from_system_source());

        // Draw the text.
        ctx.set_font("Hack-Regular");
        ctx.set_font_size(32.0);
        ctx.fill_text("omg hi servo", vec2f(32.0, 48.0));

        // Render the canvas to screen.

        Self {
            window,
            ctx,
            renderer,
        }
    }

    pub fn set_size() {
    }

    pub fn render(&mut self) {
        self.ctx.into_canvas();
        // let mut scene = SceneProxy::from_scene(self.canvas.into_canvas().into_scene(), RayonExecutor);
        // scene.build_and_render(&mut self.renderer, BuildOptions::default());
        // self.window.gl_swap_window();
    }
}
