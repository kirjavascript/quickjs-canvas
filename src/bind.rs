use quick_js::{Context, JsValue};
use crate::sdl_env::SDLEnv;
use crate::canvas::CanvasWindow;
use crate::css_color;
use crate::clone;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub fn bind_js(
    context: &Context,
    sdl_env: Arc<Mutex<SDLEnv>>,
    canvases: Arc<Mutex<HashMap<i32, CanvasWindow>>>,
) {
    // canvas methods

    context.add_callback("QJSC_initCanvas", clone!(canvases =>
        move |id: i32, width: i32, height: i32| {
            let sdl_env = sdl_env.lock().unwrap();
            let canvas = CanvasWindow::new(&sdl_env, width as _, height as _);
            canvases.lock().unwrap().insert(id, canvas);
            JsValue::Null
        }
     )).unwrap();

    context.add_callback("QJSC_setSize", clone!(canvases =>
        move |id: i32, width: i32, height: i32| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().set_size(width, height);
            JsValue::Null
        }
     )).unwrap();

    // ctx methods

    context.add_callback("QJSC_clearRect", clone!(canvases =>
        move |id: i32, x: i32, y: i32, w: i32, h: i32| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().clear_rect(x, y, w, h);
            JsValue::Null
        }
     )).unwrap();

    context.add_callback("QJSC_fillRect", clone!(canvases =>
        move |id: i32, x: i32, y: i32, w: i32, h: i32| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().fill_rect(x, y, w, h);
            JsValue::Null
        }
     )).unwrap();

    context.add_callback("QJSC_strokeRect", clone!(canvases =>
        move |id: i32, x: i32, y: i32, w: i32, h: i32| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().stroke_rect(x, y, w, h);
            JsValue::Null
        }
     )).unwrap();


    context.add_callback("QJSC_fillText", clone!(canvases =>
        move |id: i32, text: String, x: i32, y: i32| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().fill_text(text, x, y);
            JsValue::Null
        }
     )).unwrap();


    // window methods

    context.add_callback("QJSC_setTitle", clone!(canvases =>
        move |id: i32, text: String| {
            let mut canvases = canvases.lock().unwrap();
            canvases.get_mut(&id).unwrap().set_title(text);
            JsValue::Null
        }
     )).unwrap();

    // println!("{:?}", css_color::parse("lime"));
    // println!("{:?}", css_color::parse("rgba(128, 255, 255, 1)"));
    println!("{:?}", css_color::web_format(&css_color::parse("lime").unwrap()));
    println!("{:?}", css_color::web_format(&css_color::parse("rebeccapurple").unwrap()));
}
