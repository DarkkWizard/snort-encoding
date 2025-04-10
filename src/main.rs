use clap::Parser;
use encoding::{decode_huffman_solo, encode_huffman_solo};
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

#[derive(clap::ValueEnum, Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    HuffmanSolo,
    HuffmanCounts,
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
                    &text,
                    |x| x.chars(),
                    |y| {
                        y.chars().into_iter().fold(
                            HashMap::new(),
                            |mut acc: HashMap<_, _>, ch: char| {
                                *acc.entry(ch).or_insert(0) += 1;
                                acc
                            },
                        )
                    },
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
                let raw_data = std::fs::read(args.input)?;
                let uncompressed_data = decode_huffman_solo::<char>(&raw_data)?;
                dbg!(uncompressed_data);
            }
            Mode::HuffmanCounts => {
                todo!();
            }
        },
    }

    Ok(())
}
