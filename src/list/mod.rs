
// linked list

// Path: src/list/mod.rs
use std::{fmt, iter::Iterator};
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

pub fn new<T>() -> List<T> {
    List { head: None }
}

impl<T> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head.take() {
            Some(mut node) => {
                self.head = node.next.take();
                Some(node.elem)
            },
            None => None,
        }
    }
}

impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIter { next: self.head.as_ref().map(|node| &**node) }
    }
}

pub struct ListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut node = &self.head;
        write!(f, "[")?;
        while let Some(n) = node {
            write!(f, "{}", n.elem)?;
            node = &n.next;
            if node.is_some() {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

impl<T> List<T> {
    pub fn append(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: None,
        });

        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }

    pub fn prepend(&mut self, elem: T) {
        let mut new_node = Box::new(Node {
            elem,
            next: None,
        });

        match self.head.take() {
            Some(old_node) => {
                new_node.next = Some(old_node);
            }
            None => {}
        }

        self.head = Some(new_node);
    }

    pub fn size (&self) -> usize {
        self.into_iter().count()
    }

    pub fn head (&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.elem),
            None => None,
        }
    }

    pub fn tail (&self) -> Option<&T> {
        match &self.head {
            Some(node) => {
                let mut current = node;
                while let Some(next) = &current.next {
                    current = next;
                }
                Some(&current.elem)
            },
            None => None,
        }
    }

    pub fn at(&self, index: usize) -> Option<&T> {
        let mut current = &self.head;
        let mut i = 0;
        while let Some(node) = current {
            if i == index {
                return Some(&node.elem);
            }
            current = &node.next;
            i += 1;
        }
        None
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        // single node
        if self.head.as_ref().unwrap().next.is_none() {
            return self.head.take().map(|node| node.elem);
        }

        // traverse the list until the second to last node
        let mut current = &mut self.head;
        while current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        // remove & move the last node
        current.as_mut().unwrap().next.take().map(|node| node.elem)
    }

    pub fn contains (&self, elem: &T) -> bool
    where T: PartialEq {
        self.into_iter().any(|e| *e == *elem)
    }

    pub fn find (&self, elem: &T) -> Option<usize>
    where T: PartialEq {
        self.into_iter().position(|e| *e == *elem)
    }
}



