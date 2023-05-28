struct BSTNode<T> where T: std::cmp::Ord, T: std::fmt::Display {
    value: T,
    left: Option<Box<BSTNode<T>>>,
    right: Option<Box<BSTNode<T>>>,
}

impl<T> BSTNode<T> where T: std::cmp::Ord, T: std::fmt::Display {
    pub fn new(value: T) -> Self {
        return Self {
            value,
            left: None,
            right: None,
        };
    }

    fn add(&mut self, value: T) {
        if value < self.value { // going left
            match self.left {
                Some(ref mut x) => x.add(value),
                None => self.left = Some(Box::new(BSTNode::new(value))),
            }
        } else { // going right
            match self.right {
                Some(ref mut x) => x.add(value),
                None => self.right = Some(Box::new(BSTNode::new(value))),
            }
        }
    }

    fn for_each(&self) {
        if let Some(ref node) = &self.left {
            node.for_each()
        }
        println!("{}", self.value);
        if let Some(ref node) = &self.right {
            node.for_each()
        }
    }
}

pub struct BST<T> where T: std::cmp::Ord, T: std::fmt::Display {
    head: Option<Box<BSTNode<T>>>,
}

impl<T> BST<T> where T: std::cmp::Ord, T: std::fmt::Display {
    pub fn new() -> Self {
        return Self {
            head: None
        };
    }

    pub fn add(&mut self, value: T) {
        match self.head {
            Some(ref mut x) => x.add(value),
            None => self.head = Option::from(Box::new(BSTNode::new(value))),
        }
    }

    pub fn for_each(&self) {
        if let Some(ref node) = &self.head {
            node.for_each()
        }
    }
}