use pathfinder_canvas::{Canvas, CanvasFontContext, TextAlign};
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

pub struct CanvasWindow {
}

impl CanvasWindow {
    pub fn new(video: &VideoSubsystem) -> Self {
        Self {
        }
    }

    pub fn render(&self) {

    }
}
