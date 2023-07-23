use super::List;

pub struct Iter<'a, T> {
    pub(super) stack: &'a List<T>,
    pub(super) index: usize,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.stack.len() {
            None
        } else {
            let item = self.stack[self.index].clone();

            self.index += 1;

            Some(item)
        }
    }
}
