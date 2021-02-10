use pathfinder_canvas::{
    Canvas, CanvasRenderingContext2D, CanvasFontContext, Vector2I
};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::executor::SequentialExecutor;
use pathfinder_renderer::scene::Scene;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use sdl2::VideoSubsystem;
use sdl2::video::{Window, GLContext};

pub struct CanvasWindow {
    window: Window,
    // ctx: CanvasRenderingContext2D,
    // scene: Scene,
    renderer: Renderer<GLDevice>,
    window_size: Vector2I,
    gl_context: GLContext,
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

        // let mut ctx = Canvas::new(window_size.to_f32()).get_context_2d(CanvasFontContext::from_system_source());

        // // Draw the text.
        // ctx.set_font("Hack-Regular");
        // ctx.set_font_size(32.0);
        // ctx.fill_text("hi servo", vec2f(32.0, 48.0));

        // let mut scene = SceneProxy::from_scene(ctx.into_canvas().into_scene(), RayonExecutor);
        // scene.build_and_render(&mut renderer, BuildOptions::default());
        // window.gl_swap_window();

        // // Render the canvas to screen.

        Self {
            window,
            // ctx,
            // scene: Canvas::new(window_size.to_f32()).into_scene(),
            renderer,
            window_size,
            gl_context, // dropping this too soon breaks shit
        }
    }

    pub fn test(&mut self) {
        // let canvas = Canvas::from_scene(self.scene.clone());
        // let mut ctx = canvas.get_context_2d(CanvasFontContext::from_system_source());

        // ctx.set_font("Hack-Regular");
        // ctx.set_font_size(32.0);
        // ctx.fill_text("hi servo", vec2f(32.0, 48.0));

        // self.scene = ctx.into_canvas().into_scene();
    }

    pub fn render(&mut self) {

        let mut ctx = Canvas::new(self.window_size.to_f32()).get_context_2d(CanvasFontContext::from_system_source());

        // Draw the text.
        ctx.set_font("Hack-Regular");
        ctx.set_font_size(32.0);
        ctx.fill_text("omg hi servo", vec2f(32.0, 48.0));
        // ctx.set_fill_style(pathfinder_canvas::FillStyle::Color(pathfinder_canvas::ColorU::black()));
        // ctx.fill_rect(pathfinder_canvas::RectF::new(vec2f(0.,0.), vec2f(10.,10.)));

        let mut scene = SceneProxy::from_scene(ctx.into_canvas().into_scene(), RayonExecutor);
        scene.build_and_render(&mut self.renderer, BuildOptions::default());
        self.window.gl_swap_window();

        // let scene = self.scene.clone();


        // TODO: make this function less shit

        // create a fake default that self can hold while we grab self.ctx to play with
        // let fake = Canvas::new(vec2f(0.,0.)).get_context_2d(CanvasFontContext::from_system_source());
        // let ctx = std::mem::replace(&mut self.ctx, fake);

        // // extract a scene
        // let scene = ctx.into_canvas().into_scene();
        // let scene_clone = scene.clone();

        // // use the clone to restore self.ctx
        // self.ctx = Canvas::from_scene(scene_clone).get_context_2d(CanvasFontContext::from_system_source());

        // create the scene proxy and render
        // let scene = SceneProxy::from_scene(scene, RayonExecutor);
        // scene.build_and_render(&mut self.renderer, BuildOptions::default());
        // self.window.gl_swap_window();
    }
}
