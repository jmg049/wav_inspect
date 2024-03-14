//! ``wav_inspect`` is a simple command line tool for inspecting the header of a wav file. It uses
//! [WaveRs](https://crates.io/crates/wavers) as the backend for reading the audio data.
//!
//! By passing the file path of a wav file, the program will display the content of the file header. 
//! Currently, what you can expect in the output is:
//!     1. Format Chunk
//!     2. List Chunk (if present)
//!     3. Fact Chunk (if present)
//!     4. All chunks present in the header and their offset within the file. 
//!
//! ## Installation
//! You can download and install ``wav_inspect`` using ``cargo install wav_inspect``
//!
//! ## Usage 
//! From the command line just run:
//! ``wav_inspect <file_path>``
//!
//!

use clap::{arg, command};
use colored::Colorize;
use std::path::PathBuf;
use wavers::Wav;

fn main() {
    let args = command!().arg(arg!([name])).get_matches();

    let fp: PathBuf = match args.get_one::<String>("name") {
        Some(fp) => PathBuf::from(fp),
        None => {
            eprintln!("Error: No file path provided");
            return;
        }
    };

    match std::fs::metadata(&fp) {
        Ok(metadata) => {
            if !metadata.is_file() {
                eprintln!("Error: {} is not a file", fp.display());
                return;
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }

    match fp.extension() {
        Some(ext) => {
            if ext.to_string_lossy().to_lowercase() != "wav" {
                eprintln!("Error: {} is not a wav file", fp.display());
                return;
            }
        }
        None => {
            eprintln!("Error: {} is not a wav file", fp.display());
            return;
        }
    }

    let mut wav: Wav<_> = match wavers::Wav::<f32>::from_path(&fp) {
        Ok(wav) => wav,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let header = wav.header().clone();
    let list_chunk = match wav.get_list_chunk() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error reading list chunk: {}", e);
            None
        }
    };

    let fact_chunk = match wav.get_fact_chunk() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error reading fact chunk: {}", e);
            None
        }
    };

    println!(
        "{} {}",
        "Wav file:".yellow().bold().underline(),
        fp.display()
    );
    println!("{}", header);
    if let Some(list_chunk) = list_chunk {
        println!("{}", list_chunk);
    }

    if let Some(fact_chunk) = fact_chunk {
        println!("{}", fact_chunk);
    }
}
