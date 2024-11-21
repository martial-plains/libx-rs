use core::ops::Index;

use alloc::boxed::Box;

pub mod iter;

#[derive(Debug, Clone)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Default for Node<T>
where
    T: Clone + Default,
{
    fn default() -> Self {
        Self {
            data: T::default(),
            next: None,
        }
    }
}

/// A stack data structure implemented using a linked list.
///
/// # Examples
///
/// ```
/// use libx::collections::stack::linked_list::Stack;
///
/// // Creates and initializes a new Stack.
/// let mut stack = Stack::new();
/// stack.push("Hello");
/// stack.push("World");
/// stack.push("!");
///
/// println!("stack");
/// println!("\tLength: {0}", stack.len());
/// print!("\tValues:");
/// println!("\t{stack:?}");
///
///
/// ```
#[derive(Default, Clone)]
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    #[must_use]
    pub const fn new() -> Self {
        Self { top: None, len: 0 }
    }

    /// Pushes a value onto the top of the stack.
    ///
    /// # Arguments
    ///
    /// * `data` - The value to be pushed onto the stack.
    pub fn push(&mut self, data: T) {
        let node = Node {
            data,
            next: self.top.take(),
        };

        self.top = Some(Box::new(node));
        self.len += 1;
    }

    /// Removes and returns the top element from the stack.
    ///
    /// Returns `None` if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        let top = self.top.take();

        top.map(|mut node| {
            self.top = node.next.take();
            self.len -= 1;
            node.data
        })
    }

    /// Checks if the stack is empty.
    ///
    /// Returns `true` if the stack is empty, `false` otherwise.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the length of the stack.
    ///
    /// # Returns
    ///
    /// The number of elements in the stack.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns a reference to the top element of the stack.
    ///
    /// Returns `None` if the stack is empty.
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }

    /// Removes all elements from the stack.
    pub fn clear(&mut self) {
        for _ in 0..self.len() {
            self.pop();
        }
    }

    /// Returns an iterator over the elements of the stack.
    ///
    /// # Returns
    ///
    /// An iterator that yields references to the elements in the stack in LIFO order.
    #[must_use]
    pub const fn iter(&self) -> iter::Iter<'_, T>
    where
        T: Clone,
    {
        iter::Iter {
            stack: self,
            index: 0,
        }
    }
}

unsafe impl<T> Send for Stack<T> where T: Send {}
unsafe impl<T> Sync for Stack<T> {}

impl<T> core::fmt::Debug for Stack<T>
where
    T: Clone + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let mut first = true;
        let mut iter = self.top.clone();
        while let Some(node) = iter.take() {
            if first {
                write!(f, "{:?}", node.data)?;
                first = false;
            } else {
                write!(f, " -> {:?}", node.data)?;
            }
            iter.clone_from(&node.next);
        }
        Ok(())
    }
}

impl<T> Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut current = self.top.as_ref();

        for i in (0..self.len()).rev() {
            if let Some(node) = current {
                if i == index {
                    return &node.data;
                }
                current = node.next.as_ref();
            } else {
                break;
            }
        }

        panic!("Index out of bounds");
    }
}

impl<'a, T> IntoIterator for &'a Stack<T>
where
    T: Clone,
{
    type Item = T;

    type IntoIter = iter::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use alloc::{format, string::String};

    use super::*;

    #[test]
    fn test_stack_push() {
        let mut stack = super::Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_pop() {
        let mut stack = super::Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack = super::Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_stack_len() {
        let mut stack = super::Stack::new();
        assert_eq!(stack.len(), 0);
        stack.push(1);
        assert_eq!(stack.len(), 1);
        stack.push(2);
        assert_eq!(stack.len(), 2);
        stack.pop();
        assert_eq!(stack.len(), 1);
        stack.pop();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_stack_display() {
        let mut stack = super::Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(format!("{stack:?}"), "3 -> 2 -> 1");
    }

    #[test]
    fn test_stack_display_empty() {
        let stack = super::Stack::<String>::new();
        assert_eq!(format!("{stack:?}"), "");
    }

    #[test]
    fn test_stack_display_one() {
        let mut stack = super::Stack::new();
        stack.push(1);
        assert_eq!(format!("{stack:?}"), "1");
    }

    #[test]
    fn test_stack_peak() {
        let mut stack = super::Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.peek(), Some(&3));
        stack.pop();
        assert_eq!(stack.peek(), Some(&2));
        stack.pop();
        assert_eq!(stack.peek(), Some(&1));
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_stack_clear() {
        let mut stack = Stack::new();
        stack.push("The");
        stack.push("quick");
        stack.push("brown");
        stack.push("fox");
        stack.push("jumps");

        assert_eq!(stack.len(), 5);

        stack.clear();
        assert_eq!(stack.len(), 0);
    }
}
