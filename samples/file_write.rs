use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut path = env::home_dir().unwrap();
    path.push("Desktop");
    path.push("file_write_sammple.txt");

    println!("{:?}", path);

    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    writer.write("hello rust\n".as_bytes()).unwrap();
    writer.write("こんにちは\n".as_bytes()).unwrap();

    //file.flush();

    println!("write file.");

    Ok(())
}
