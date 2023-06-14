use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tree<T> {
    root: Node<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data,
            children: Vec::new(),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn add_child(&mut self, data: T) -> &mut Node<T> {
        self.children.push(Node::new(data));

        self.children.last_mut().unwrap()
    }

    pub fn has_children(&self) -> bool {
        self.children.len() > 0
    }
}

impl<T> Tree<T> {
    pub fn new(root_data: T) -> Tree<T> {
        Tree {
            root: Node::new(root_data),
        }
    }

    pub fn root(&self) -> &Node<T> {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.root
    }

    pub fn find_mut<F>(&mut self, predicate: F) -> Option<&mut Node<T>>
    where
        F: Fn(&T) -> bool,
    {
        Self::traverse_mut(&mut self.root, Rc::new(predicate))
    }

    pub fn find<F>(&mut self, predicate: F) -> Option<&Node<T>>
    where
        F: Fn(&T) -> bool,
    {
        match Self::traverse_mut(&mut self.root, Rc::new(predicate)) {
            Some(node) => Some(&*node),
            None => None,
        }
    }

    fn traverse_mut<F>(node: &mut Node<T>, predicate: Rc<F>) -> Option<&mut Node<T>>
    where
        F: Fn(&T) -> bool,
    {
        for child in &mut node.children {
            if predicate(&child.data) {
                return Some(child);
            } else if child.has_children() {
                match Self::traverse_mut(child, predicate.clone()) {
                    Some(child) => return Some(child),
                    None => continue,
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_node() {
        let mut tree = build_tree();

        assert_eq!(tree.find_mut(|d| d == &5).unwrap().data, 5);
        assert_eq!(tree.find_mut(|d| d == &999), None);
        assert_eq!(tree.find(|d| d == &5).unwrap().data, 5);
        assert_eq!(tree.find(|d| d == &999), None);
    }

    fn build_tree() -> Tree<i32> {
        let (root, child_a, child_b, child_c, child_d) = (1, 2, 3, 4, 5);

        let mut tree = Tree::new(root);
        let root = tree.root_mut();
        root.add_child(child_a);

        let parent = root.add_child(child_b);
        parent.add_child(child_c);
        parent.add_child(child_d);

        tree
    }
}
