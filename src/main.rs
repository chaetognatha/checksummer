use std::fs::read_to_string;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use checksums::hash_file;
use clap::Parser;


fn main() {
    let args = Args::parse();
    let checksum_file = Path::new(&args.input);
    let output_file = Path::new("./result.txt");
    let mut output = File::create(output_file).unwrap();
    for line in read_to_string(checksum_file).unwrap().lines(){
        let my_split = line.split_whitespace();
        let my_collection = my_split.collect::<Vec<&str>>();
        if my_collection.len() == 2 {
            let my_hash = my_collection[0];
            let my_path = Path::new(my_collection[1]);
            if let Some(text_path) = my_path.to_str(){
            let digest = hash_file(my_path, checksums::Algorithm::MD5).to_lowercase();
            if my_hash == digest {
                writeln!(output, "{}    OK", text_path).unwrap();
            } else {
                writeln!(output, "{}    ERROR: MISMATCH", text_path).unwrap();
            }
        }
        }
    }
    println!("DONE!")
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

