use cidr::IpCidr;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;

/**
 * 按行读取内容，按空格分割符分割出向量
 */
fn read_file_ip1_and_ip2(file_path: &str) -> Result<Vec<(String, String)>, std::io::Error> {
    let mut result: Vec<(String, String)> = Vec::new();

    match File::open(file_path) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line_result in reader.lines() {
                if let Ok(line) = line_result {
                    let words: Vec<_> = line.split_whitespace().collect();
                    if let Some(ip1) = words.get(0) {
                        if let Some(ip2) = words.get(1) {
                            result.push((ip1.to_string(), ip2.to_string()));
                        }
                    }
                } else if let Err(e) = line_result {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(result)
}

/**
 * 按行读取文件内容，获取IP段的开始IP、结束IP，计算对应IPv4 CIDR的值
 */
fn main() {
    let file_path = "input.txt";
    let write_file_path = "output.txt";
    // 创建文件（用于输出文件）
    let mut write_file = match File::create(write_file_path) {
        Ok(write_file) => write_file,
        Err(e) => {
            println!("无法创建文件: {}", e);
            return;
        }
    };
    match read_file_ip1_and_ip2(file_path) {
        Ok(result) => {
            for (start_ip_str, end_ip_str) in result {
                let start_ip = Ipv4Addr::from_str(&start_ip_str).unwrap();
                let end_ip = Ipv4Addr::from_str(&end_ip_str).unwrap();

                let mut current_ip = u32::from(start_ip);
                let end_ip_u32 = u32::from(end_ip);

                while current_ip <= end_ip_u32 {
                    // 确保当前IP的主机部分为0
                    let current_ip_net = Ipv4Addr::from(current_ip & (0xFF_FF_FF_00));

                    match IpCidr::new(IpAddr::V4(current_ip_net), 24) {
                        Ok(cidr) => {
                            // 将计算后的cidr写入文件中
                            match write_file.write_all(format!("{}\n", cidr).as_bytes()) {
                                Ok(()) => {
                                    println!("{} - {} => {}", start_ip_str, end_ip_str, cidr);
                                    let _ = write_file.flush(); // 立刻写入文件（可以看成写入一个cidr就保存）
                                }
                                Err(e) => {
                                    println!("无法写入文件: {}", e);
                                }
                            }
                        }
                        Err(e) => println!("Error: {}", e),
                    }
                    current_ip += 256; // 增加256个，因为每个/24网络包含256个IP地址
                }
            }
        }
        Err(e) => {
            println!("无法处理文件: {}", e);
        }
    }
}
