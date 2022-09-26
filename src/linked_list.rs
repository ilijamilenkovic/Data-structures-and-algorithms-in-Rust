use std::fmt::Display;

struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(val: i32, next: Option<Box<Node>>) -> Self {
        Self {
            value: val,
            next: next,
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut current_node_value) = current_node {
            current_node = current_node_value.next.take();
        }
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current_node = &self.head;

        let _ = write!(f, "[ ");
        while let Some(node) = current_node {
            let _ = write!(f, "{} ", node.value);
            current_node = &node.next;
        }
        write!(f, "]")
    }
}

impl Into<Vec<i32>> for LinkedList {
    fn into(self) -> Vec<i32> {
        let mut vector = Vec::<i32>::new();
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            vector.push(node.value);
            current_node = &node.next;
        }
        vector
    }
}

impl LinkedList {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push_head(&mut self, new_value: i32) {
        let old_head = self.head.take();
        let new_head = Node::new(new_value, old_head);
        self.head = Some(Box::new(new_head));
    }

    pub fn pop_head(&mut self) -> Option<i32> {
        let old_head = self.head.take();
        match old_head {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&i32> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
    pub fn len(&self) -> i32 {
        let mut count = 0;
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            count += 1;
            current_node = &node.next;
        }

        count
    }

    pub fn push_back(&mut self, new_value: i32) {
        let mut current_node = self.head.as_mut();

        while let Some(node) = current_node {
            if let None = node.next {
                node.next = Some(Box::new(Node::new(new_value, None)));
                return;
            }
            current_node = node.next.as_mut();
        }
        //we didn't get into while let loop, it means that the list is empty
        self.push_head(new_value);
    }

    pub fn reverse(&mut self) {
        let mut current_node = self.head.take();
        while let Some(mut current_node_value) = current_node {
            let next_node = current_node_value.next;
            current_node_value.next = self.head.take();
            self.head = Some(current_node_value);
            current_node = next_node;
        }
    }

    pub fn traverse(&self) {
        let mut current_node = &self.head;
        while let Some(current_node_value) = current_node {
            //do something
            println!("{}", current_node_value.value);
            current_node = &current_node_value.next;
        }
    }
    pub fn pop_back(&mut self) -> Option<i32> {
        todo!()
    }

    pub fn push_in_position(&mut self, index: i32) {
        todo!()
    }

    pub fn pop_from_position(&mut self, index: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_pushes_and_pops_head() {
        let mut ll = LinkedList::new();
        ll.push_head(3);
        ll.push_head(4);
        ll.push_head(11);
        ll.push_head(123);
        assert_eq!(ll.len(), 4);
        assert_eq!(ll.pop_head().unwrap(), 123);
        assert_eq!(ll.pop_head().unwrap(), 11);
        assert_eq!(ll.pop_head().unwrap(), 4);
        assert_eq!(ll.pop_head().unwrap(), 3);
        assert_eq!(ll.pop_head(), None);
        assert_eq!(ll.len(), 0);
    }
}

#[test]
fn it_converts_to_vec() {
    let mut ll = LinkedList::new();
    ll.push_head(3);
    ll.push_head(4);
    ll.push_head(11);
    ll.push_head(123);
    let vec: Vec<i32> = ll.into();
    assert_eq!(vec, vec![123, 11, 4, 3]);
}

#[test]
fn it_reverses() {
    let mut ll = LinkedList::new();
    ll.push_head(3);
    ll.push_head(4);
    ll.push_head(11);
    ll.push_head(123);
    let vec: Vec<i32> = ll.into();
    assert_eq!(vec, vec![123, 11, 4, 3]);

    let mut ll = LinkedList::new();
    ll.push_head(3);
    ll.push_head(4);
    ll.push_head(11);
    ll.push_head(123);
    ll.reverse();
    let vec_reversed: Vec<i32> = ll.into();
    assert_eq!(vec_reversed, vec![3, 4, 11, 123]);
}

#[test]
fn it_pushes_back_and_pops() {
    let mut ll = LinkedList::new();
    ll.push_back(3);
    ll.push_back(4);
    ll.push_back(5);
    let _s = ll.pop_back().unwrap();

    println!("{}", ll);
    let vec: Vec<i32> = ll.into();
    assert_eq!(vec, vec![3, 4]);
}
