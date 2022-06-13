use itertools::Itertools;
use std::collections::VecDeque;
use std::iter;

//https://www.codewars.com/kata/52bef5e3588c56132c0003bc/train/rust
//use preloaded::Node;
#[derive(Debug)]
struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, node: Node) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn right(mut self, node: Node) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

fn tree_by_levels(root: &Node) -> Vec<u32> {
    let mut deque = VecDeque::from([root]);
    iter::from_fn(move || {
        let node = deque.pop_front()?;
        deque.extend(
            [&node.left, &node.right]
                .into_iter()
                .flatten()
                .map(|node| &**node),
        );
        Some(node.value)
    })
    .collect_vec()
}

#[cfg(test)]
mod sample_tests {
    use super::*;

    #[test]
    fn root_only() {
        assert_eq!(
            tree_by_levels(&Node::new(42)),
            [42],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }

    #[test]
    fn complete_tree() {
        let root = Node::new(1)
            .left(Node::new(2).left(Node::new(4)).right(Node::new(5)))
            .right(Node::new(3).left(Node::new(6)));
        assert_eq!(
            tree_by_levels(&root),
            [1, 2, 3, 4, 5, 6],
            "\nYour result (left) didn't match the expected output (right)."
        );
    }
}
