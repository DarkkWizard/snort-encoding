use clap::Parser;
use encoding::{decode_huffman_solo, encode_huffman_solo};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{BufWriter, Write},
    path::PathBuf,
};
mod encoding;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_enum)]
    action: Action,
    #[arg(value_enum)]
    mode: Mode,
    input: PathBuf,
    output: PathBuf,
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
enum Action {
    Encode,
    Decode,
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
enum Mode {
    HuffmanSolo,
    HuffmanCounts,
    HuffmanChunks,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.action {
        Action::Encode => match args.mode {
            Mode::HuffmanSolo => {
                let text = std::fs::read_to_string(args.input)?;
                let dest = args.output.clone();
                eprintln!("found and read file");

                let compressed_datums = encode_huffman_solo(
                    // this is so dumb but I want it for the extra functionality in the other
                    // length inputs.
                    &text,
                    |x| x.chars().map(|g| g.to_string()),
                    |y| {
                        y[0].chars().into_iter().fold(
                            HashMap::new(),
                            |mut acc: HashMap<_, _>, ch: char| {
                                *acc.entry(ch.to_string()).or_insert(0) += 1;
                                acc
                            },
                        )
                    },
                    Mode::HuffmanSolo,
                )?;
                eprintln!("compressed the given data");

                let file = std::fs::File::create(&dest)?;
                let mut bw = BufWriter::new(&file);
                let _ = bw.write(&compressed_datums);
                eprintln!("writen compressed data to {:?}", &args.output);
            }
            Mode::HuffmanChunks => {
                let text = std::fs::read_to_string(args.input)?;
                let dest = args.output.clone();
                eprintln!("found and read file");

                let compressed_datums = encode_huffman_solo(
                    &text,
                    |x| {
                        let mut chars = x.chars();
                        std::iter::from_fn(move || {
                            let n = chars.next()?;
                            let nn = chars.next()?;
                            Some(format!("{}{}", n, nn))
                        })
                    },
                    |y| {
                        y.iter().into_iter().fold(
                            HashMap::new(),
                            |mut acc: HashMap<_, _>, st: &String| {
                                *acc.entry(st.to_string()).or_insert(0) += 1;
                                acc
                            },
                        )
                    },
                    Mode::HuffmanChunks,
                )?;
                eprintln!("compressed the given data");

                let file = std::fs::File::create(&dest)?;
                let mut bw = BufWriter::new(&file);
                let _ = bw.write(&compressed_datums);
                eprintln!("writen compressed data to {:?}", &args.output);
            }
            Mode::HuffmanCounts => {
                todo!();
            }
        },
        Action::Decode => match args.mode {
            Mode::HuffmanSolo => {
                let dest = args.output.clone();
                let raw_data = std::fs::read(args.input)?;
                eprintln!("Read the input data");

                let uncompressed_data = decode_huffman_solo::<char>(&raw_data)?;
                eprintln!("Decopmressed data in __ time");

                let file = std::fs::File::create(&dest)?;
                let mut bw = BufWriter::new(&file);
                let _ = bw.write(&uncompressed_data.into_bytes());
                eprintln!("Written decompressed data to {:?}", &args.output);
            }
            Mode::HuffmanCounts => {
                todo!();
            }
            Mode::HuffmanChunks => {
                let dest = args.output.clone();
                let raw_data = std::fs::read(args.input)?;
                eprintln!("Read the input data");

                let uncompressed_data = decode_huffman_solo::<String>(&raw_data)?;
                eprintln!("Decopmressed data in __ time");

                let file = std::fs::File::create(&dest)?;
                let mut bw = BufWriter::new(&file);
                let _ = bw.write(&uncompressed_data.into_bytes());
                eprintln!("Written decompressed data to {:?}", &args.output);
            }
        },
    }

    Ok(())
}
