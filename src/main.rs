mod canvas;
mod clone;

use canvas::CanvasWindow;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

use quick_js::{Context, JsValue, console::Level};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

fn main() {
    // create JS ctx
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
    // TODO: http://www.zebkit.org/dark/about.html

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
            id
        }
     )).unwrap();

    context.add_callback("QJSC_clearRect", clone!(canvases =>
        move |id: i32, x: f64, y: f64, w: f64, h: f64| {
            canvases.lock().unwrap().get_mut(&id).unwrap().clear_rect(x, y, w, h);
            id
        }
     )).unwrap();

    // load initial user code

    eval(include_str!("../demo_raf.js")); // TODO: replace with user supplied code

    // event loop

    let mut time = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let frame_size = Duration::from_millis(16);
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
                Event::Window { win_event: WindowEvent::Exposed, .. } => {
                },
                _ => {}
            }
        }

        let args: Vec<i32> = vec![];
        context.call_function("flushRAFQueue", args).unwrap();

        for (_, canvas) in canvases.lock().unwrap().iter_mut() {
            canvas.render();
        }

        // framerate limiting

        let now = Instant::now();
        let delta = now.duration_since(time);
        if delta < frame_size {
            std::thread::sleep(frame_size - delta);
        }
        time = now;
    }

}
