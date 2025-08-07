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
            let ipv4bit16 = bit8to16(ipv4addr);
            makeipv6(ipv4bit16);
        }
        Err(err) => {
            println!("Invalid IPv4 address :(");
        }
    }
}

fn bit8to16(ipv4addr: Ipv4Addr) -> Vec<u16> {

    let mut octets = ipv4addr.octets();
    println!("Your address is {}", ipv4addr);

    let ipv4chunk: Vec<u16> = octets
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect(); 
    
    ipv4chunk
}

fn makeipv6(ipv4chunk: Vec<u16>) {
    let segments: [u16; 2] = ipv4chunk.try_into().expect("err");
    println!("Using 64:ff9b:: as the prefix");

    let ipv6_addr = Ipv6Addr::new(
        0x0064, 0xff9b, 0x0000, 0x0000, 0x0000, 0x0000, segments[0], segments[1]
    );

    println!("Ipv6 address: {}", ipv6_addr);
}
     
