use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use snort::{Tree, Tree::Leaf, Tree::Node};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CompressedData<T: Eq + std::hash::Hash> {
    encoder: HashMap<T, BitVec>,
    data: Vec<BitVec>,
    encode_type: crate::Mode,
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
    String: FromIterator<T> + std::fmt::Debug,
>(
    text: &'a String,
    extract_tokens: TokenExtractor,
    freq_finder: FreqF,
    mode: crate::Mode,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    TokenExtractor: Fn(&'a String) -> TokensIter,
    TokensIter: Iterator<Item = T>,
    FreqF: Fn(&Vec<String>) -> HashMap<T, u64>,
    Vec<String>: FromIterator<T>,
{
    let w: Vec<String> = extract_tokens(text).collect();
    let freqs = freq_finder(&w);

    let working_tree = huffman_tree(&freqs);

    let encoder = working_tree.create_encode_table();

    let data: Vec<BitVec> = extract_tokens(text)
        .map(|token| encoder.get(&token).unwrap().clone())
        .collect();

    let mut datums = Vec::new();
    let mut serializer =
        rmp_serde::Serializer::new(&mut datums).with_bytes(rmp_serde::config::BytesMode::ForceAll);

    CompressedData {
        encoder,
        data,
        encode_type: mode,
    }
    .serialize(&mut serializer)?;

    Ok(datums)
}

pub fn decode_huffman_solo<
    'a,
    T: Eq + Clone + std::hash::Hash + std::fmt::Display + Deserialize<'a>,
>(
    text: &'a Vec<u8>,
) -> Result<String, Box<dyn std::error::Error>> {
    let strct: CompressedData<T> = rmp_serde::from_slice(text)?;
    let decode_tree = encoder_to_decoder(&strct.encoder);

    let data_remaining: Result<String, Box<dyn std::error::Error>> =
        strct.data.iter().fold(Ok(String::new()), |acc, val| {
            let mut acc_new = acc?;
            match decode_tree.get(val) {
                Some(decoded_value) => {
                    acc_new.push_str(&decoded_value.to_string());
                    Ok(acc_new)
                }
                None => Err("Invalid data, could not be decoded".into()),
            }
        });

    let result = data_remaining?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
}
