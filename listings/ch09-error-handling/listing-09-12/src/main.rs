use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file_benvenuto = File::open("ciao.txt")?;

    Ok(())
}
