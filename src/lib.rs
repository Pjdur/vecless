use std::fmt::{self, Display};

/// A minimal singly linked list that avoids using `Vec`.
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    /// Create a new empty list.
    pub fn new() -> Self {
        List { head: None }
    }

    /// Add multiple elements to the list, preserving their original order.
    pub fn add(self, items: impl IntoIterator<Item = T>) -> Self {
        let mut temp = List::new();
        for item in items {
            temp = temp.push(item);
        }
        self.append(temp.reverse())
    }

    /// Push a single element to the front of the list.
    pub fn push(mut self, elem: T) -> Self {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self
    }

    /// Append another list to the end of this one.
    fn append(mut self, mut other: Self) -> Self {
        if self.head.is_none() {
            return other;
        }

        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = other.head.take();
                break;
            }
            current = &mut node.next;
        }
        self
    }

    /// Reverse the list.
    fn reverse(mut self) -> Self {
        let mut reversed = List::new();
        while let Some(node) = self.head.take() {
            reversed = reversed.push(node.elem);
            self.head = node.next;
        }
        reversed
    }

    /// Get an iterator over the list.
    pub fn iter(&self) -> ListIter<'_, T> {
        ListIter {
            next: self.head.as_deref(),
        }
    }
}

/// Iterator for the list.
pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

/// Implement Display so the list can be printed with `{}`.
impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        write!(f, "[")?;
        for item in self.iter() {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
            first = false;
        }
        write!(f, "]")
    }
}
