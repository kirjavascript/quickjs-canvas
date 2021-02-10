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

    // TODO: JSEnv struct
    // TODO: move bindings into a module with this

    context.add_callback("QJSC_initCanvas", clone!(canvases =>
        move |id: i32| {
            let video = video.lock().unwrap();
            canvases.lock().unwrap().insert(id, CanvasWindow::new(&video));
            id
        }
     )).unwrap();

    context.add_callback("QJSC_fillText", clone!(canvases =>
        move |id: i32, text: String, x: f64, y: f64| {
            canvases.lock().unwrap().get_mut(&id).unwrap().fill_text(text, x, y);
            0
        }
     )).unwrap();

    // load initial user code

    eval(include_str!("../demo.js")); // TODO: replace with user supplied code

    // event loop

    let mut frames = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {

        match event_pump.wait_event() { // TODO: poll_iter
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            Event::Window { win_event: WindowEvent::Exposed, .. } => {
                for (_, canvas) in canvases.lock().unwrap().iter_mut() {
                    // canvas.test(frames);
                    canvas.render();
                }
            },
            _ => {}
        }

        // TODO: framerate limiting

        frames += 1;
    }

}
