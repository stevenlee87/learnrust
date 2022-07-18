use std::fs::File;
use std::io::{BufRead, BufReader};

fn main(){
    read_file_line_by_line("hello.txt").expect("Failed to open hello.txt");
}

// 逐行读取文本文件
fn read_file_line_by_line(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}