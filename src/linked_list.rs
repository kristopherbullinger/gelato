#[derive(Default, Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Node<T> {
    pub value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
        }
    }

    fn into_inner(self) -> T {
        self.value
    }

    fn inner(&self) -> &T {
        &self.value
    }

    fn inner_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0,
        }
    }

    fn pop_node_front(&mut self) -> Option<Box<Node<T>>> {
        let mut out = self.head.take()?;
        self.head = out.next.take();
        self.len -= 1;
        Some(out)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_node_front().map(|n| n.into_inner())
    }

    fn pop_node_back(&mut self) -> Option<Box<Node<T>>> {
        if self.is_empty() {
            return None;
        }
        self.len -= 1;
        let mut penultimate_node = self.get_node_mut(self.len - 2).unwrap(); //annoying variable name
        penultimate_node.next.take()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_node_back().map(|n| n.into_inner())
    }

    fn push_node_front(&mut self, mut new_head: Box<Node<T>>) {
        let old_head = self.head.take();
        new_head.next = old_head;
        self.head = Some(new_head);
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let old_head = self.head.take();
        let mut new_head = Node::new(value);
        new_head.next = old_head;
        self.head = Some(Box::new(new_head));
        self.len += 1;
    }

    //inserts value such that it is the nth element in the list.
    //panics if n > self.len
    pub fn insert(&mut self, value: T, n: usize) {
        match n {
            0 => {
                let mut new_node = Box::new(Node::new(value));
                new_node.next = self.head.take();
                self.head = Some(new_node);
            }
            _ => {
                let mut node = match self.get_node_mut(n - 1) {
                    Some(n) => n,
                    None => panic!("Cannot insert element at n. (n is {}, but len is {})", n, self.len)
                };
                let mut new_node = Box::new(Node::new(value));
                new_node.next = node.next.take();
                node.next = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            node: self.head.as_ref(),
        }
    }

    fn get_node(&self, n: usize) -> Option<&Box<Node<T>>> {
        if n >= self.len {
            return None;
        }
        let mut out_node = self.head.as_ref()?;
        for _ in 0..n {
            out_node = out_node.next.as_ref()?;
        }
        Some(out_node)
    }

    pub fn get(&self, n: usize) -> Option<&T> {
        self.get_node(n).map(|n| n.inner())
    }

    fn get_node_mut(&mut self, n: usize) -> Option<&mut Box<Node<T>>> {
        if n >= self.len {
            return None;
        }
        let mut out_node = self.head.as_mut().unwrap();
        for _ in 0..n {
            out_node = out_node.next.as_mut().unwrap();
        }
        Some(out_node)
    }

    pub fn get_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_node_mut(n).map(|n| n.inner_mut())
    }

    pub fn push_back(&mut self, value: T) {
        let new_tail = Some(Box::new(Node::new(value)));
        let mut tail_node = &mut self.head;
        while let Some(node) = tail_node {
            tail_node = &mut node.next;
        }
        *tail_node = new_tail;
        self.len += 1;
    }

    fn head_node(&self) -> Option<&Box<Node<T>>> {
        self.get_node(0)
    }

    pub fn head(&self) -> Option<&T> {
        self.head_node().map(|n| n.inner())
    }

    fn tail_node(&self) -> Option<&Box<Node<T>>> {
        self.get_node(self.len - 1)
    }

    pub fn tail(&self) -> Option<&T> {
        self.tail_node().map(|n| n.inner())
    }

    fn get_tail_node_mut(&mut self) -> Option<&mut Box<Node<T>>> {
        self.get_node_mut(self.len - 1)
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        self.get_tail_node_mut().map(|n| n.inner_mut())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn reverse(&mut self) {
        let mut stack = Vec::with_capacity(self.len);
        while let Some(node) = self.pop_node_front() {
            stack.push(node);
        }
        while let Some(node) = stack.pop() {
            self.push_node_front(node)
        }
    }

}

pub struct Iter<'a, T> {
    node: Option<&'a Box<Node<T>>>,
}

impl<'a, T> std::iter::Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let mut out = self.node?;
        self.node = out.next.as_ref();
        Some(out.inner())
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> std::iter::IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            list: self,
        }
    }
}

impl<T> std::iter::Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

impl<T> std::iter::FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::<T>::new();
        let mut tail = &mut list.head;
        for i in iter {
            let newest_node = Box::new(Node::new(i));
            *tail = Some(newest_node);
            tail = tail.as_mut().map(|n| &mut n.next).unwrap();
            list.len += 1;
        }
        list
    }
}

impl<T: std::cmp::PartialEq> std::cmp::PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && {
            let a = self.iter();
            let b = other.iter();
            a.zip(b).all(|(ai, bi)| ai == bi)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_from_iter() {
        let list = (0..100).collect::<LinkedList<_>>();
        assert_eq!(list.len(), 100);
        assert_eq!(list.head(), Some(&0));
        assert_eq!(list.tail(), Some(&99));
    }

    #[test]
    fn can_push_front() {
        let mut list = LinkedList::<usize>::new();
        list.push_front(100);
        assert_eq!(list.head(), Some(&100));
    }

    #[test]
    fn can_push_back() {
        let mut list = LinkedList::<usize>::new();
        list.push_back(100);
        assert_eq!(list.tail(), Some(&100));
    }

    #[test]
    fn can_insert_into_middle() {
        let mut list = (0..100).collect::<LinkedList<_>>();
        let a = 9999999;
        let b = 45;
        list.insert(a, b);
        assert_eq!(list.get(b), Some(&a));
        let a = 123123;
        let b = 68;
        list.insert(a, b);
        assert_eq!(list.get(b), Some(&a));
        let a = 0;
        let b = list.len();
        list.insert(a, b);
        assert_eq!(list.get(b), Some(&a));
    }

    #[test]
    fn inserting_increments_length_by_one() {
        let mut list = (0..100).collect::<LinkedList<_>>();
        assert_eq!(list.len(), 100);
        list.insert(1000, 0);
        assert_eq!(list.len(), 101);
    }

    #[test]
    fn pop_front_and_back_decr_length_by_one() {
        let mut list = (0..100).collect::<LinkedList<_>>();
        assert_eq!(list.len(), 100);
        list.pop_front();
        assert_eq!(list.len(), 99);
        list.pop_back();
        assert_eq!(list.len(), 98);
    }

    #[test]
    fn can_into_iter() {
        let list = (0..100).collect::<LinkedList<_>>();
        let expected = (0..100).collect::<Vec<_>>();
        let observed = list.into_iter().collect::<Vec<_>>();
        assert_eq!(expected, observed);
    }

    #[test]
    fn similar_lists_compare_equal() {
        let list1 = (0..100).collect::<LinkedList<_>>();
        let list2 = (0..100).collect::<LinkedList<_>>();
        assert_eq!(list1, list2);
    }

    #[test]
    fn can_reverse() {
        let forward = (0..100).rev().collect::<LinkedList<_>>();
        let reversed = {
            let mut list = (0..100).collect::<LinkedList<_>>();
            list.reverse();
            list
        };
        assert_eq!(forward, reversed);
    }
}

