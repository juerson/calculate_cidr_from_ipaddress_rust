use std::net::{IpAddr, Ipv4Addr};
use cidr::IpCidr;
use std::str::FromStr;


fn common_prefix_len(a: u32, b: u32) -> u8 {
    let mut x = a ^ b;
    let mut len = 0;
    while x != 0 {
        x >>= 1;
        len += 1;
    }
    32 - len
}

fn main() {
    let start_ip_str = "213.188.207.0";
    let end_ip_str = "213.188.223.255";
    
    let start_ip = Ipv4Addr::from_str(start_ip_str).unwrap();
    let end_ip = Ipv4Addr::from_str(end_ip_str).unwrap();
    
    let prefix_len = common_prefix_len(u32::from(start_ip), u32::from(end_ip));
    
    // Create a mask with the same number of leading ones as the prefix length
    let mask = (!0u32) << (32 - prefix_len);

    // Ensure the host part of the start IP is zero
    let start_ip_net = Ipv4Addr::from(u32::from(start_ip) & mask);

    let cidr = IpCidr::new(IpAddr::V4(start_ip_net), prefix_len).unwrap();
    println!("The CIDR representation of the IP range {} - {} is {}", start_ip, end_ip, cidr);
}
