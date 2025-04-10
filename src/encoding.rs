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

fn encoder_to_decoder<K, V>(map: &HashMap<K, V>) -> HashMap<V, K>
where
    V: std::hash::Hash + Clone + Eq,
    K: std::hash::Hash + Clone + Eq,
{
    map.iter().map(|(k, v)| (v.clone(), k.clone())).collect()
}

pub fn encode_huffman_solo<
    'a,
    T: std::fmt::Debug + Clone + Eq + std::hash::Hash + Serialize,
    TokenExtractor,
    FreqF,
    TokensIter,
>(
    text: &'a String,
    extract_tokens: TokenExtractor,
    freq_finder: FreqF,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    TokenExtractor: Fn(&'a String) -> TokensIter,
    TokensIter: Iterator<Item = T>,
    FreqF: Fn(&String) -> HashMap<T, u64>,
{
    let freqs = freq_finder(text);
    let working_tree = huffman_tree(&freqs);

    let encoder = working_tree.create_encode_table();

    let data: Vec<BitVec> = extract_tokens(text)
        .map(|token| encoder.get(&token).unwrap().clone())
        .collect();
    rmp_serde::to_vec(&CompressedData { encoder, data }).map_err(|x| x.into())
}

pub fn decode_huffman_solo<'a, T: Eq + Clone + std::hash::Hash + Deserialize<'a>>(
    text: &'a Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    let strct: CompressedData<T> = rmp_serde::from_slice(text)?;
    let decode_tree = encoder_to_decoder(&strct.encoder);
}

#[cfg(test)]
mod tests {
    use super::*;
}
