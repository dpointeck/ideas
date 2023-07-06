use std::net::Ipv4Addr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Address to start the server on
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    pub address: String,

    /// Port to start the server on
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

pub fn split_address(address: String) -> Ipv4Addr {
    let mut address_vec: Vec<u8> = Vec::new();
    let mut temp: String = String::new();
    for i in address.chars() {
        if i == '.' {
            address_vec.push(temp.parse::<u8>().unwrap());
            temp = String::new();
        } else {
            temp.push(i);
        }
    }
    address_vec.push(temp.parse::<u8>().unwrap());
    let array: [u8; 4] = address_vec.try_into().unwrap();
    Ipv4Addr::from(array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_address() {
        let address = "192.168.0.1".to_string();
        let expected: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 1);
        assert_eq!(split_address(address), expected);
    }
    
}