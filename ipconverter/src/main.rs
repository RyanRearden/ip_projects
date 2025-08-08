use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

fn main() {
    println!("Enter an IPv4 addr:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("That is not a number");

    let input = input.trim();

    match Ipv4Addr::from_str(input) {
        Ok(ipv4addr) => {
            makeipv6(ipv4addr);
        }
        Err(error) => {
            panic!("Invalid IPv4 address: {error:?}");
        }
    }
}


fn makeipv6(ipv4addr: Ipv4Addr) {

    let ipv4octs = ipv4addr.octets();

    println!("Using 64:ff9b:: as the prefix");
    let ipv6_prefix = Ipv6Addr::new(0x0064, 0xff9b, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000);
    let mut ipv6octs = ipv6_prefix.octets();

    let mut octaddr = [0; 16];
    octaddr[..12].copy_from_slice(&ipv6octs[0..12]);
    octaddr[12..].copy_from_slice(&ipv4octs[0..4]);
    let ipv6_addr = Ipv6Addr::from(octaddr); 

    println!("Ipv6 address: {}", ipv6_addr);
}
     
