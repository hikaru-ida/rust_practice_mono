use ch07::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgeringar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgeringar".to_string()));
}

pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}
