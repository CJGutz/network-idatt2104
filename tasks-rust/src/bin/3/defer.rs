struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        self.0.take().map(|f| f());
    }
}

pub fn defer<F: FnOnce()>(f: F) -> impl Drop {
    Defer(Some(f))
}
