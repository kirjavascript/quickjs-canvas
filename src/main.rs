mod args;
mod bind;
mod canvas;
mod clone;
mod css_color;
mod msg_box;
mod path;
mod sdl_env;
mod text;

use sdl_env::SDLEnv;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

use quick_js::{Context, JsValue, console::Level};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

fn main() {
    let code = args::get_script();

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

    // setup UI and IO

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // disable double buffering

    video.gl_attr().set_double_buffer(false);

    // stick SDL stuff in a mutex so canvas can access it

    let sdl_env = Arc::new(Mutex::new(SDLEnv {
        video,
    }));

    // attach QJSC controls to context

    let canvases = Arc::new(Mutex::new(HashMap::new()));

    // creating bindings to JS context

    bind::bind_js(&context, sdl_env.clone(), canvases.clone());

    // load initial user code

    eval(&code);

    // if no canvases created, exit at this point

    if canvases.lock().unwrap().len() == 0 {
        std::process::exit(0);
    }

    // event loop

    let mut time = Instant::now();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let frame_size = Duration::from_micros(16666);
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
