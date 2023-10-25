use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "DE.txt";
    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                if let Ok(line) = line_result {
                    println!("{}", line.trim());

                    let words: Vec<_> = line.split_whitespace().collect();
                    if let Some(ip1) = words.get(0) {
                        println!("IP1: {}", ip1);
                    }
                    if let Some(ip2) = words.get(1) {
                        println!("IP2: {}", ip2);
                    }
                } else if let Err(e) = line_result {
                    println!("无法读取文件内容: {}", e);
                }
            }
        }
        Err(e) => {
            println!("{} 文件不存在或无法打开: {}", file_path, e);
        }
    }
}
