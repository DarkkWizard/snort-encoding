use bitvec::vec::BitVec;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

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

impl<T> Tree<T>
where
    T: Clone,
{
    pub fn freq(&self) -> u64 {
        match self {
            Tree::Leaf { freq, .. } => *freq,
            Tree::Node { freq, .. } => *freq,
        }
    }

    pub fn left(&self) -> Option<&Tree<T>> {
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { left_node, .. } => Some(left_node),
        }
    }

    pub fn right(&self) -> Option<&Tree<T>> {
        match self {
            Tree::Leaf { .. } => None,
            Tree::Node { right_node, .. } => Some(right_node),
        }
    }

    pub fn token(&self) -> Option<T> {
        match self {
            Tree::Leaf { token, .. } => Some(token.clone()),
            Tree::Node { .. } => None,
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

impl<T: Eq + Clone + std::hash::Hash> Tree<T> {
    pub fn create_encode_table(&self) -> HashMap<T, BitVec> {
        let mut encode_table = HashMap::new();

        let mut stack = vec![(self, BitVec::new())];
        while !stack.is_empty() {
            let (node, code) = stack.pop().unwrap();
            match node {
                Tree::Leaf { token, .. } => {
                    encode_table.insert(token.clone(), code.clone());
                }
                Tree::Node {
                    left_node,
                    right_node,
                    ..
                } => {
                    let mut left_path = code.clone();
                    left_path.push(false);
                    stack.push((left_node, left_path));

                    let mut right_path = code.clone();
                    right_path.push(true);
                    stack.push((right_node, right_path));
                }
            }
        }

        encode_table
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

#[cfg(test)]
mod tests {
    // use super::*;
}
