use quick_js::{Context, JsValue};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use std::collections::HashMap;

pub fn poll_events(event_pump: &mut EventPump, context: &Context) -> bool  {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return false,
            Event::KeyDown { window_id, keycode: Some(keycode), ..} => {
                let event: HashMap<String, JsValue> = [
                    ("key".to_string(), JsValue::String(format!("{}", keycode))),
                ].iter().cloned().collect();
                let args: Vec<JsValue> = vec![
                    JsValue::Int(window_id as _),
                    JsValue::String("keydown".to_string()),
                    JsValue::Object(event),
                ];
                if let Err(err) = context.call_function("QJSC_Event", args) {
                    eprintln!("{:?}", err);
                };
            },
            Event::KeyUp { window_id, keycode: Some(keycode), ..} => {
                let event: HashMap<String, JsValue> = [
                    ("key".to_string(), JsValue::String(format!("{}", keycode))),
                ].iter().cloned().collect();
                let args: Vec<JsValue> = vec![
                    JsValue::Int(window_id as _),
                    JsValue::String("keyup".to_string()),
                    JsValue::Object(event),
                ];
                if let Err(err) = context.call_function("QJSC_Event", args) {
                    eprintln!("{:?}", err);
                };
            },
            Event::MouseMotion { window_id, x, y, xrel, yrel, which, ..} => {
                let event: HashMap<String, JsValue> = [
                    ("x".to_string(), JsValue::Int(x)),
                    ("pageX".to_string(), JsValue::Int(x)),
                    ("offsetX".to_string(), JsValue::Int(x)),
                    ("clientX".to_string(), JsValue::Int(x)),
                    ("y".to_string(), JsValue::Int(y)),
                    ("pageY".to_string(), JsValue::Int(y)),
                    ("offsetY".to_string(), JsValue::Int(y)),
                    ("pageY".to_string(), JsValue::Int(y)),
                    ("which".to_string(), JsValue::Int(which as _)),
                    ("movementX".to_string(), JsValue::Int(xrel)),
                    ("movementY".to_string(), JsValue::Int(yrel)),
                ].iter().cloned().collect();
                let args: Vec<JsValue> = vec![
                    JsValue::Int(window_id as _),
                    JsValue::String("mousemove".to_string()),
                    JsValue::Object(event),
                ];
                if let Err(err) = context.call_function("QJSC_Event", args) {
                    eprintln!("{:?}", err);
                };
            },
            _ => {}
        }
    }
    true
}
