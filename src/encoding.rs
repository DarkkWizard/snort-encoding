use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use snort::{Tree, Tree::Leaf, Tree::Node};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompressedData<T: Eq + std::hash::Hash> {
    encoder: HashMap<T, BitVec>,
    data: Vec<BitVec>,
}

pub fn huffman_tree<T: Eq + Clone>(input_with_freqs: &HashMap<T, u64>) -> Tree<T> {
    let mut heap = BinaryHeap::new();

    // we have to use it reversed so that we can have a min first one, not a max first one
    for (token, freq) in input_with_freqs {
        heap.push(Reverse(Leaf {
            freq: *freq,
            token: token.clone(),
        }));
    }

    while heap.len() > 1 {
        let (node1, node2) = (heap.pop().unwrap().0, heap.pop().unwrap().0);
        let new = Node {
            freq: node1.freq() + node2.freq(),
            left_node: Box::new(node1),
            right_node: Box::new(node2),
        };

        heap.push(Reverse(new));
    }

    //only happens when we have one node left, and the full list is sorted
    heap.pop().unwrap().0
}

pub fn freqs_chars(encoding_chaff: &String) -> HashMap<char, u64> {
    encoding_chaff
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc: HashMap<_, _>, ch: char| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        })
}

// fn numbers_to_tree<T>(tree: &Tree<T>) -> HashMap<T,

// fn create_prepend_table(table: &HuffmanNode) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
//     let serialized = serde_json::to_vec(&table)?;
//     Ok(serialized)
// }

fn compress_to_struct<T: Eq + std::hash::Hash>(tree: &Tree<T>) -> CompressedData<T> {
    let encoder = HashMap::new();
}

pub fn encode_huffman_solo<'a, T, TokenExtractor, TokensIter>(
    text: &String,
    extract_tokens: TokenExtractor,
) -> CompressedData<char>
where
    TokenExtractor: Fn(&'a str) -> TokensIter,
    TokensIter: Iterator<Item = T>,
{
    let freqs = freqs_chars(&text);
    let working_tree = huffman_tree(&freqs);
}

#[cfg(test)]
mod tests {
    use super::*;
}
