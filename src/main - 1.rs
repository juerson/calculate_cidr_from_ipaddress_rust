use std::net::{IpAddr, Ipv4Addr};
use cidr::IpCidr;
use std::str::FromStr;


fn _common_prefix_len(a: u32, b: u32) -> u8 {
    let mut x = a ^ b;
    let mut len = 0;
    while x != 0 {
        x >>= 1;
        len += 1;
    }
    32 - len
}

fn main() {
    
    let start_ip_str = "213.191.34.136";
    let end_ip_str = "213.191.35.231";

    let start_ip = Ipv4Addr::from_str(start_ip_str).unwrap();
    let end_ip = Ipv4Addr::from_str(end_ip_str).unwrap();

    let mut current_ip = u32::from(start_ip);
    let end_ip_u32 = u32::from(end_ip);

    while current_ip <= end_ip_u32 {
        // Ensure the host part of the current IP is zero
        let current_ip_net = Ipv4Addr::from(current_ip & (0xFF_FF_FF_00));
        
        match IpCidr::new(IpAddr::V4(current_ip_net), 24) {
            Ok(cidr) => println!("The CIDR representation of the IP range is {}", cidr),
            Err(e) => println!("Error: {}", e),
        }
        
        current_ip += 256; // Increase by 256 because each /24 network contains 256 IP addresses
    }
}
