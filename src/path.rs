#[derive(Debug)]
pub struct Path(Vec<(i32, i32)>);

impl Path {
    pub fn from_interop(paths: Vec<Vec<Vec<i32>>>) -> Vec<Path> {
        paths.into_iter().map(|path| {
            Path(path.into_iter().map(vec_to_point).collect())
        }).collect()
    }
}

fn vec_to_point(vec: Vec<i32>) -> (i32, i32) {
    let mut iter = vec.into_iter();
    (
        iter.next().expect("missing point"),
        iter.next().expect("missing point"),
    )
}
