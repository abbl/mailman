use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Node<T> {
    data: T,
    children: Vec<Node<T>>,
}

#[derive(Deserialize, Serialize)]
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

    pub fn root_mut(&mut self) -> &mut Node<T> {
        &mut self.root
    }

    pub fn find_mut(&mut self, predicate: fn(&T) -> bool) -> Option<&mut Node<T>> {
        Tree::traverse(&mut self.root, predicate)
    }

    fn traverse(node: &mut Node<T>, predicate: fn(&T) -> bool) -> Option<&mut Node<T>> {
        for child in &mut node.children {
            if predicate(&child.data) {
                return Some(child);
            } else if child.has_children() {
                match Tree::traverse(child, predicate) {
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
