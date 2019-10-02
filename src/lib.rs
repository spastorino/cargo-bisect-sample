pub struct Slice<'a, T>(&'a [T]);

impl<'a, T: 'a> Slice<'a, T> {
    pub const EMPTY: Self = Slice ({
        let v: &[T] = &[];
        v
    });
}

#[test]
fn test_empty() {
    let s = Slice(&[1, 2]);
    assert!(s.0 != Slice::EMPTY.0);
}
