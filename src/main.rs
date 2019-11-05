use std::net::SocketAddr;
use std::num::ParseIntError;
use std::str::FromStr;
use tokio::net::tcp::{ConnectFuture, TcpStream};

#[derive(Debug)]
struct Square {
    width: i32,
    height: i32,
}

impl FromStr for Square {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();

        let width_fromstr: i32 = coords[0].parse::<i32>()?;
        let height_fromstr: i32 = coords[1].parse::<i32>()?;

        Ok(Square {
            width: width_fromstr,
            height: height_fromstr,
        })
    }
}

fn main() {
    // &str to u32 w/ turbofish
    let four: u32 = "4".parse::<u32>().unwrap();
    println!("{:#?}", four);

    // &str to f32 w/ turbofish
    let pie: f32 = "3.14".parse::<f32>().unwrap();
    println!("{:#?}", pie);

    // &str to String
    let foo: String = "j".parse::<String>().unwrap();
    println!("{:#?}", foo);

    // &str to SocketAddr
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let connect_future: ConnectFuture = TcpStream::connect(&addr);
    println!("{:#?}", connect_future);

    // &str to Square's FromString implementation
    let a1: Square = Square::from_str("(1,2)").unwrap();
    println!("{:#?}", a1);
}
