use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HuffmanNode {
    pub left_node: Option<Box<HuffmanNode>>,
    pub right_node: Option<Box<HuffmanNode>>,
    /// The number of times that the letter is used in the data given
    pub freq: usize,
    /// The letter that we are encoding
    pub value: Option<char>,
}

impl HuffmanNode {
    /// Creates a new HuffmanNode from data that we give it. Use this for leaf nodes
    /// as if we want an inner node then we would need to pass in other HuffmanNodes
    /// to work with.
    pub fn new_alone(value: char, freq: usize) -> Self {
        Self {
            left_node: None,
            right_node: None,
            freq,
            value: Some(value),
        }
    }

    /// Creates a new HuffmanNode from two already existing nodes. Used for creating
    /// the tree that we use to encode.
    pub fn new_from_two(left_node: HuffmanNode, right_node: HuffmanNode) -> Self {
        Self {
            freq: left_node.freq + right_node.freq,
            left_node: Some(Box::new(left_node.clone())),
            right_node: Some(Box::new(right_node.clone())),
            value: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Tree<T> {
    Leaf {
        freq: u64,
        token: T,
    },
    Node {
        freq: u64,
        left_node: Box<Tree<T>>,
        right_node: Box<Tree<T>>,
    },
}

impl<T> Tree<T> {
    pub fn freq(&self) -> u64 {
        match self {
            Tree::Leaf { freq, .. } => *freq,
            Tree::Node { freq, .. } => *freq,
        }
    }
}

impl<T: Clone + Eq> Ord for Tree<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.freq().cmp(&other.freq())
    }
}

impl<T: Clone + Eq> PartialOrd for Tree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn print_tree<T: std::fmt::Debug>(tree: &Tree<T>) {
    match tree {
        Tree::Leaf { token, .. } => {
            println!("{:?}", token);
        }
        Tree::Node { .. } => {
            println!("*");
            print_subtree(tree, String::new());
            println!();
        }
    }
}

fn print_subtree<T: std::fmt::Debug>(tree: &Tree<T>, prefix: String) {
    match tree {
        Tree::Node {
            left_node,
            right_node,
            ..
        } => {
            let has_left = matches!(**left_node, Tree::Node { .. } | Tree::Leaf { .. });
            let has_right = matches!(**right_node, Tree::Node { .. } | Tree::Leaf { .. });

            // Right side
            if has_right {
                let print_strand = has_left && matches!(**right_node, Tree::Node { .. });
                print!("{}", prefix);
                print!("{}", if has_left { "├── " } else { "└── " });
                print_node_value(&**right_node);
                let new_prefix =
                    format!("{}{}", prefix, if print_strand { "│   " } else { "    " });
                print_subtree(&**right_node, new_prefix);
            }

            // Left side
            if has_left {
                print!(
                    "{}",
                    if has_right {
                        prefix.clone()
                    } else {
                        prefix.clone()
                    }
                );
                print!("└── ");
                print_node_value(&**left_node);
                let new_prefix = format!("{}    ", prefix);
                print_subtree(&**left_node, new_prefix);
            }
        }
        Tree::Leaf { .. } => {}
    }
}

fn print_node_value<T: std::fmt::Debug>(tree: &Tree<T>) {
    match tree {
        Tree::Leaf { token, .. } => println!("{:?}", token),
        Tree::Node { .. } => println!("*"),
    }
}
