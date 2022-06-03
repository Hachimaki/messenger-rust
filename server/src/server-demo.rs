extern crate core;

use core::time::Duration;
use std::{io::{Read, Write, Result}, net::{TcpListener, TcpStream}, thread::{JoinHandle, sleep}, thread};

fn main() -> Result<()> {
    // enable binding to port 7878
    let receiver_listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind with the sender");
    // getting a handle of the underlying thread
    let mut thread_vec: Vec<JoinHandle<()>> = Vec::new();
    // listen to incoming connections message and bind them to a socket server address
    for stream in receiver_listener.incoming() {
        let stream = stream.expect("failed");
        // let the receiver connect with the sender
        let handle = thread::spawn(move || {
            // receiver fauled to read from the stream
            handle_sender(stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        // push messages in the order they are sent
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // return each single value output contained in the heap
        handle.join().unwrap();
    }

    Ok(())
}

// handle access stream
// create a struct to hold the stream's state
// perform I/O operations
fn handle_sender(mut stream: TcpStream) -> Result<()> {
    // handle multiple access streams
    let mut buffer = [0;512];
    for _ in 0..1000 {
        // let the receiver get a message from a sender
        let bytes_read = stream.read(&mut buffer)?;

        // sender stream in a mutable variable
        if bytes_read == 0 {
            return Ok(());
        }

        stream.write(&buffer[..bytes_read])?;

        // print acceptance message
        // read, print the message sent
        println!("from the sender: {}", String::from_utf8_lossy(&buffer));
        // sleep the connection with the connected sender
        sleep(Duration::from_secs(1));
    }

    // success value
    Ok(())
}