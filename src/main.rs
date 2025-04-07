use encoding::{encode_huffman_solo, freqs_chars, huffman_tree};
mod encoding;
use clap::Parser;
use std::path::PathBuf;

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
            Mode::HuffmanCounts => {
                todo!();
            }
            Mode::HuffmanSolo => {
                let text = std::fs::read_to_string(args.input)?;
                let dest = args.output.clone();
                eprintln!("found and read file");

                let compressed_datums = encode_huffman_solo(&text, |text| text.chars());
                eprintln!("compressed the given data into {:?}", &args.output);
            }
        },
        Action::Decode => {
            todo!();
        }
    }

    Ok(())
}
