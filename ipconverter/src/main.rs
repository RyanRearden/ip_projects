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

fn bit8to16(ipv4addr: Ipv4Addr) -> [u16; 2] {

    let mut octets = ipv4addr.octets();
    println!("Your address is {}", ipv4addr);

    let ipv4chunkvec: Vec<u16> = octets
        .chunks_exact(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect(); 
    
    let ipv4chunkarr: [u16; 2] = ipv4chunkvec.try_into().expect("Error making u16 array from vector");
    ipv4chunkarr
}

fn makeipv6(ipv4chunk: [u16; 2]) {
    println!("Using 64:ff9b:: as the prefix");

    let ipv6_addr = Ipv6Addr::new(
        0x0064, 0xff9b, 0x0000, 0x0000, 0x0000, 0x0000, ipv4chunk[0], ipv4chunk[1]
    );

    println!("Ipv6 address: {}", ipv6_addr);
}
     
