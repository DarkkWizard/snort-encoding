use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use snort::{Tree, Tree::Leaf, Tree::Node};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CompressedData<T: Eq + std::hash::Hash> {
    encoder: HashMap<T, BitVec>,
    data: BitVec,
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

pub fn encode_huffman<
    'a,
    T: std::fmt::Debug + Clone + Eq + std::hash::Hash + Serialize,
    TokenExtractor,
    FreqF,
    TokensIter,
    String: FromIterator<T> + std::fmt::Debug,
>(
    text: &'a String,
    extract_tokens_iter: TokenExtractor,
    freq_finder: FreqF,
    mode: crate::Mode,
) -> Result<Vec<u8>, Box<dyn std::error::Error>>
where
    TokenExtractor: Fn(&'a String) -> TokensIter,
    TokensIter: Iterator<Item = T>,
    FreqF: Fn(&Vec<String>) -> HashMap<T, u64>,
    Vec<String>: FromIterator<T>,
{
    let w: Vec<String> = extract_tokens_iter(text).collect();
    let freqs = freq_finder(&w);

    let working_tree = huffman_tree(&freqs);

    let encoder = working_tree.create_encode_table();

    let data: BitVec = extract_tokens_iter(text)
        .map(|token| encoder.get(&token).unwrap().clone())
        .fold(BitVec::new(), |mut acc, x| {
            acc.extend_from_bitslice(&x);
            acc
        });

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

pub fn decode_huffman(text: &Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
    let strct: CompressedData<String> = rmp_serde::from_slice(text)?;
    let decoder = encoder_to_decoder(&strct.encoder);

    let data_remaining: Result<String, Box<dyn std::error::Error>> = {
        let mut tokens: Vec<String> = vec![];
        let mut canditate = BitVec::new();

        for bit in strct.data {
            canditate.push(bit);

            match decoder.get(&canditate) {
                Some(success) => {
                    tokens.push(success.clone().to_string());
                    canditate = BitVec::new();
                }
                None => (),
            }
        }
        Ok(tokens.iter().map(|c| c.to_string()).collect())
    };

    let result = data_remaining?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Mode;

    #[test]
    fn encode_decode() {
        let encodable = "other greater pleasures, or else he endures pains to avoid worse
                        pains.But I must explain to you how all this mistaken idea of denouncing
                        pleasure and praising pain was born and I will give you a complete
                        account of the system, and expound the actual teachings of the great
                        explorer of the truth, the master-builder of human happiness. No one
                        rejects, dislikes, or avoids pleasure itself, because it is pleasure,
                        but because those who do not know how to pursue pleasure rationally
                        encounter consequences that are extremely painful. Nor again is there
                        anyone who loves or pursues or desires to obtain pain of itself, because
                        it is pain, but because occasionally circumstances occur in which toil
                        and pain can procure him some great pleasure. To take a trivial example,
                        which of us ever undertakes laborious physical exercise, except to
                        obtain some advantage from it? But who has any right to find fault with
                        a man who chooses to enjoy a pleasure that has no annoying consequences,
                        or one who avoids a pain"
            .to_string();
        let encoded = encode_huffman(
            &encodable,
            |x| x.chars().map(|g| g.to_string()),
            |y| {
                y.into_iter()
                    .fold(HashMap::new(), |mut acc: HashMap<_, _>, ch: &String| {
                        *acc.entry(ch.to_string()).or_insert(0) += 1;
                        acc
                    })
            },
            Mode::HuffmanChars,
        )
        .unwrap();

        let decoded = decode_huffman(&encoded).unwrap();

        assert_eq!(&encodable, &decoded);
    }

    #[test]
    fn tree() {
        let mut freqs = HashMap::new();
        freqs.insert("hey".to_string(), 45);
        freqs.insert("sup".to_string(), 30);
        freqs.insert("kool".to_string(), 15);
        freqs.insert("rad".to_string(), 10);

        let tree = huffman_tree(&freqs);

        assert_eq!(tree.freq(), 100);

        // most frequent token should be the shortest
        assert_eq!(tree.left().and_then(|l| Some(l.freq())), Some(45));
        assert_eq!(tree.left().and_then(|l| l.token()), Some("hey".to_string()));

        // second most should have the second shortest
        assert_eq!(
            tree.right().and_then(|l| l.right()).and_then(|r| r.token()),
            Some("sup".to_string())
        );
        assert_eq!(
            tree.right()
                .and_then(|l| l.right())
                .and_then(|r| Some(r.freq())),
            Some(30)
        );

        // I don't want to write the more testing
    }
}
