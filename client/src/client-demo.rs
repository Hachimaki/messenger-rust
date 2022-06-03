use std::{
    str,
    net::TcpStream,
    io::{
        self,
        prelude::*,
        BufReader,
        Result,
        Write
    }
};

fn main() -> Result<()> {
    // connect
    // struct used to start requests to the server
    // check TCP stream connection to the server
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    for _ in 0..1000 {
        // allow sender to enter message input
        let mut input = String::new();
        // first access the input message and read it
        io::stdin().read_line(&mut input).expect("Failed to read");
        // write the message so that the receiver can access it
        stream.write(input.as_bytes()).expect("failed to write");
        // add buffering so that the receiver can read messages from the stream
        let mut reader = BufReader::new(&stream);
        // check fi this input message values are u8
        let mut buffer: Vec<u8> = Vec::new();
        // read input information
        reader.read_until(b'\n', &mut buffer)?;

        println!("read from server:{}", str::from_utf8(&buffer).unwrap());
        println!();
    }

    Ok(())
}