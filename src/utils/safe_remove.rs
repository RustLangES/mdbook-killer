pub trait SafeRemove<T> {
    fn safe_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> SafeRemove<T> for Vec<T> {
    fn safe_remove(&mut self, index: usize) -> Option<T> {
        self.len()
            .ge(&index.checked_add(1)?)
            .then(|| self.swap_remove(index))
    }
}
