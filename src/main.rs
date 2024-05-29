use clap::{Arg, Command};
use md5::{Md5, Digest};
use sha2::{Sha256, Sha512};

use std::fs::File;
use std::io::{Read};
use std::path::Path;

fn main() {
    let matches = Command::new("CLI Hashing App")
        .version("1.0")        
        .about("Hashes a text file using MD5, SHA-256, or SHA-512")
        .arg(Arg::new("file")
            .short('f')
            .long("file")
            .num_args(1)
            .help("The text file to hash"))
        .arg(Arg::new("algorithm")
            .short('a')
            .long("algorithm")
            .num_args(1)
            .value_parser(["md5", "sha256", "sha512"])
            .help("The hashing algorithm to use"))
        .get_matches();

    let file_path = matches.get_one::<String>("file").expect("File path is required");
    let algorithm = matches.get_one::<String>("algorithm").expect("Algorithm is required");

    if !Path::new(file_path).exists() {
        eprintln!("File does not exist.");
        return;
    }

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let hash = match algorithm.as_str() {
        "md5" => format!("{:x}", Md5::digest(contents.as_bytes())),
        "sha256" => format!("{:x}", Sha256::digest(contents.as_bytes())),
        "sha512" => format!("{:x}", Sha512::digest(contents.as_bytes())),
        _ => unreachable!(),
    };

    println!("Hash using {}: {}", algorithm, hash);
}
