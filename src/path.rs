use quick_js::JsValue;

pub struct Path(Vec<(i32, i32)>);


impl Path {
    pub fn from_args(paths: JsValue) -> Vec<Path> {
        value_to_vec(paths).into_iter().map(|path| {
            Path(value_to_vec(path).into_iter().map(value_to_point).collect())
        }).collect()
    }
}

fn value_to_vec(arr: JsValue) -> Vec<JsValue> {
    if let JsValue::Array(vec) = arr {
        vec
    } else {
        vec![]
    }
}

fn value_to_point(arr: JsValue) -> (i32, i32) {
    if let JsValue::Array(vec) = arr {
        let mut iter = vec.into_iter();
        (
            if let JsValue::Int(int) = iter.next().expect("missing point") {
                int
            } else {
                unreachable!("bad interop data");
            },
            if let JsValue::Int(int) = iter.next().expect("missing point") {
                int
            } else {
                unreachable!("bad interop data");
            }
        )
    } else {
        unreachable!("bad interop data");
    }
}
