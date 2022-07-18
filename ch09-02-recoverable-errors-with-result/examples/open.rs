use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("hello.txt")?;
    Ok(())
}