mod canvas;

use quick_js::{Context, JsValue, console::Level};

use canvas::Canvas;

use std::collectio::HashMap;
use std::sync::Mutex;
// use std::cell::RefCell;
// use std::rc::Rc;

// #[macro_export]
// macro_rules! clone {
//     ( $( $x:ident ),* => $y:expr ) => {
//         {
//             $(let $x = $x.clone();)*
//             $y
//         }
//     };
//     ( $x:ident $y:expr ) => {
//         {
//             let $x = $x.clone();
//             $y
//         }
//     };
// }

fn main() {
    let context = Context::builder()
        .console(|level: Level, args: Vec<JsValue>| {
            eprintln!("{}: {:?}", level, args);
        })
        .build()
        .unwrap();

    let eval = |code| {
        if let Err(err) = context.eval(code) {
            eprintln!("{:?}", err);
        };
    };

    eval(include_str!("./js/prelude.js"));

    // let canvases: Rc<RefCell<HashMap<i32, Canvas>>> = Rc::new(
    //     RefCell::new(HashMap::new())
    // );

    // context.add_callback("QJSC_initCanvas", clone!(canvases => |id: i32| {
    //     canvases.borrow_mut().insert(id, Canvas::new());
    //     0
    // })).unwrap();

    let mut canvases: HashMap<i32, Canvas> = HashMap::new();

    context.add_callback("QJSC_initCanvas", |id: i32| {
        canvases.insert(id, Canvas::new());
        0
    }).unwrap();





    eval(include_str!("../demo.js")); // tmp

}

// use pathfinder_canvas::{Canvas, CanvasFontContext, TextAlign};
// use pathfinder_color::ColorF;
// use pathfinder_geometry::vector::{vec2f, vec2i};
// use pathfinder_gl::{GLDevice, GLVersion};
// use pathfinder_renderer::concurrent::rayon::RayonExecutor;
// use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
// use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
// use pathfinder_renderer::gpu::renderer::Renderer;
// use pathfinder_renderer::options::BuildOptions;
// use pathfinder_resources::embedded::EmbeddedResourceLoader;
// use sdl2::event::{Event, WindowEvent};
// use sdl2::keyboard::Keycode;
// use sdl2::video::GLProfile;


// fn main() {
//     // Set up SDL2.
//     let sdl_context = sdl2::init().unwrap();
//     let video = sdl_context.video().unwrap();

//     // Make sure we have at least a GL 3.0 context. Pathfinder requires this.
//     let gl_attributes = video.gl_attr();
//     gl_attributes.set_context_profile(GLProfile::Core);
//     gl_attributes.set_context_version(3, 3);

//     // Open a window.
//     let window_size = vec2i(640, 480);
//     let window = video.window("quickjs-canvas", window_size.x() as u32, window_size.y() as u32)
//                       .opengl()
//                       .build()
//                       .unwrap();

//     // Create the GL context, and make it current.
//     let gl_context = window.gl_create_context().unwrap();
//     gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
//     window.gl_make_current(&gl_context).unwrap();

//     // Create a Pathfinder renderer.
//     let resource_loader = EmbeddedResourceLoader::new();
//     let device = GLDevice::new(GLVersion::GL3, 0);
//     let options = RendererOptions {
//         background_color: Some(ColorF::white()),
//         ..RendererOptions::default()
//     };
//     let mut renderer = Renderer::new(device, &resource_loader, DestFramebuffer::full_window(window_size), options);

//     let font_context = CanvasFontContext::from_system_source();

//     // Wait for a keypress.
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     loop {
//         match event_pump.wait_event() {
//             Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
//             Event::Window { win_event: WindowEvent::Exposed, .. } => {
//                 // Make a canvas.
//                 let mut canvas = Canvas::new(window_size.to_f32()).get_context_2d(font_context.clone());

//                 // Draw the text.
//                 canvas.set_font("Hack-Regular");
//                 canvas.set_font_size(32.0);
//                 canvas.fill_text("omg hi servo", vec2f(32.0, 48.0));

//                 // Render the canvas to screen.
//                 let mut scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
//                 scene.build_and_render(&mut renderer, BuildOptions::default());
//                 window.gl_swap_window();
//             },
//             _ => {}
//         }
//     }
// }
