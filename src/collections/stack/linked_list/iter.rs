use super::Stack;

#[derive(Debug)]
pub struct Iter<'a, T>
where
    T: Clone,
{
    pub(super) stack: &'a Stack<T>,
    pub(super) index: usize,
}

impl<T> Iterator for Iter<'_, T>
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
