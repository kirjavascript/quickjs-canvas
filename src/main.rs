use quick_js::{Context, JsValue, console::Level};

static PRELUDE: &str = include_str!("./prelude.js");

static DEMO: &str = include_str!("../demo.js"); // tmp

fn main() {
    let context = Context::builder()
        .console(|level: Level, args: Vec<JsValue>| {
            eprintln!("{}: {:?}", level, args);
        })
        .build()
        .unwrap();

    if let Err(err) = context.eval(PRELUDE) {
        eprintln!("{:?}", err);
    };

    if let Err(err) = context.eval(DEMO) {
        eprintln!("{:?}", err);
    };
}
