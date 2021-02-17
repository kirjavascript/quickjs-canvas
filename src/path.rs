use quick_js::JsValue;

pub struct Path {
    points: Vec<(i32, i32)>,
}

impl Path {
    pub fn from_args(path: JsValue) -> Vec<Path> {
        println!("{:?}", path);

        vec![]
    }
}
