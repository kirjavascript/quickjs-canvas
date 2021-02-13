use sdl2::VideoSubsystem;
use sdl2::ttf::Sdl2TtfContext;

pub struct SDLEnv {
    pub video: VideoSubsystem,
    pub ttf_context: Sdl2TtfContext,
}
