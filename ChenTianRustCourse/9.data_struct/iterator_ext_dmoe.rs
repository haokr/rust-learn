
trait IteratorExt {
    fn window_count(&self);
    fn next(&self);
}

impl IntoIterator for Vec {
    fn window_count(&self) -> &self {
        // todo
    }
}

fn main() {
    let v = vec!(1,2,3,4,5,6,7,8,9);

    v.iter()
}