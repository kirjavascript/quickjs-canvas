use quick_js::Context;
use crate::sdl_env::SDLEnv;
use crate::canvas::CanvasWindow;
use crate::clone;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub fn bind_js(
    context: &Context,
    sdl_env: Arc<Mutex<SDLEnv>>,
    canvases: Arc<Mutex<HashMap<i32, CanvasWindow>>>,
) {
    context.add_callback("QJSC_initCanvas", clone!(canvases =>
        move |id: i32| {
            let sdl_env = sdl_env.lock().unwrap();
            canvases.lock().unwrap().insert(id, CanvasWindow::new(&sdl_env));
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
}
