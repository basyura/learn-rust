use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    // Rustの?演算子
    // https://qiita.com/kanna/items/a0c10a0563573d5b2ed0
    let mut path = env::current_dir()?;
    path.push("file_read_line.rs");

    println!("{}\n", path.display());

    for result in BufReader::new(File::open(path)?).lines() {
        let l = result?;
        println!("{}", l);
    }

    Ok(())
}
