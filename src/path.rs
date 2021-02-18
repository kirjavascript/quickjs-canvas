use quick_js::JsValue;

pub struct Path {
    points: Vec<(i32, i32)>,
}

impl Path {
    pub fn from_args(paths: JsValue) -> Vec<Path> {
        let paths = array_to_vec(paths);

        let out = paths.into_iter().map(|path| {
            let points = value_to_vec(path);
            println!("{:?}", points);

            ()
        }).collect::<Vec<_>>();

        vec![]
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
        let iter = vec.into_iter();
        (
            if let JsValue::Int(int) = iter.next().expect("missing point") {
                int
            } else {
                0
            },
            if let JsValue::Int(int) = iter.next().expect("missing point") {
                int
            } else {
                0
            }
        )
    } else {
        unreachable!("bad interop data");
    }
}
