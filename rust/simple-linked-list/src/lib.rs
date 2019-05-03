use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        return SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut curr = &self.head;
        let mut len : usize = 0;
        while let Some(node) = curr {
            len += 1;
            curr = &node.next; 
        }
        len
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Box::new(Node {
            data: element,
            next: mem::replace(&mut self.head, None),
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = mem::replace(&mut self.head, None) {
            self.head = node.next;
            return Some(node.data);
        }
        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = &self.head {
            return Some(&node.data);
        }
        None
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev = SimpleLinkedList::new();
        let mut curr = &self.head;
        while let Some(node) = curr {
            rev.push(node.data.clone());
            curr = &node.next;
        }
        rev
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(item: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for el in item {
            list.push(el.clone());
        }
        list
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec : Vec<T> = Vec::new();
        let mut curr = self.head;
        while let Some(node) = curr {
            vec.push(node.data);
            curr = node.next;
        }
        vec.reverse();
        vec
    }
}
