use std::fs::File;

use clap::Parser;
use rs_conllu::{cli, parse_file};

fn main() {
    let cli = cli::LintCli::parse();
    let walker = walkdir::WalkDir::new(cli.path).into_iter();

    for entry in walker {
        let path = entry.unwrap().into_path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "conllu" {
                    println!("Parsing {path:?}");
                    let file = File::open(path).unwrap();
                    for s in parse_file(file) {
                        if let Err(e) = s {
                            println!("❌");
                            println!("{e}");
                        }
                    }
                    println!()
                }
            }
        }
    }
}
