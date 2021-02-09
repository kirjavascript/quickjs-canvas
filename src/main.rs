mod canvas;
mod clone;

use canvas::CanvasWindow;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use quick_js::{Context, JsValue, console::Level};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    // create JS env
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

    // setup UI

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Make sure we have at least a GL 3.0 context. Pathfinder requires this.
    let gl_attributes = video.gl_attr();
    gl_attributes.set_context_profile(GLProfile::Core);
    gl_attributes.set_context_version(3, 3);

    // stick video in a mutex so canvas can access it

    let video = Arc::new(Mutex::new(video));

    // attach QJSC controls to context

    let canvases = Arc::new(Mutex::new(HashMap::new()));

    context.add_callback("QJSC_initCanvas", clone!(canvases =>
        move |id: i32| {
            let video = video.lock().unwrap();
            canvases.lock().unwrap().insert(id, CanvasWindow::new(&video));
            0
        }
     )).unwrap();

    // load initial user code

    eval(include_str!("../demo.js")); // TODO: replace with user supplied code

        for (_, canvas) in canvases.lock().unwrap().iter_mut() {
            // canvas.test();
            canvas.render();
        }


    // event loop

    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {

        match event_pump.wait_event() { // TODO: poll_iter
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            Event::Window { win_event: WindowEvent::Exposed, .. } => {


            },
            _ => {}
        }


        // TODO: framerate limiting
    }

}



// fn _main() {
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
