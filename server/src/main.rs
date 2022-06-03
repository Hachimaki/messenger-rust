extern crate core;
extern crate core;

use core::panicking::panic;
use core::time::Duration;
use std::{env, fs, io::{Read, Write, Result}, net::{TcpListener, TcpStream}, thread::{JoinHandle, sleep}, thread};
use std::collections::HashMap;
use std::fs::File;
use std::process::exit;

use user::User;

pub struct Server {
    user_info_file:String,
    ip_address:String,
    server_port:String,
    master_socket_descriptor:i16,
    registered_users:HashMap<String, User>,
    open_connections:HashMap<thread, User>,
    connected_users:i16
}

impl Server{
    fn setup_config(&mut self, file:String) {
        println!("Loading configration file at {}", file);

        let mut config_file = match File::open(file) {
            Err(error) => panic!("Error opening file {}: {}", file, error),
            Ok(file) => file,
        };

        let mut config = String::new();
        config_file.read_to_string(&mut config)?;
        let mut config = config.split(": \n");
        if config[0] == "port" {
            self.set_port(config[1]);
        }

        println!("Server configuration loaded.");
    }
    fn setup_users(&mut self, file:String) {
        println!("Loading user info file at {}", file);

        if let Ok(lines) = read_lines(file) {
            for line in lines {
                if let Ok(user_info) = line {
                    user_info.split("|;\n");
                    // user | password | friend 1; ... ; friend n \n
                    let mut user = User{
                        username: user_info[0].to_string(),
                        password: user_info[1].to_string(),
                        user_friends: Vec::new(),
                        ip_address: "",
                        client_port: "",
                        client_socket_descriptor: -1,
                        info_string: "",
                    };
                    // TODO: this needs testing
                    for friend in user_info(2..) {
                        user.add_new_friend(friend);
                    }

                    self.add_registered_user((user_info[0], user));
                }
            }
        }

    }
    fn init_server(&self) {
    }
    fn shutdown(&self) {}

    // TODO: determine if these are necessary
    fn get_ip(&self) -> &String { &self.ip_address }
    fn set_ip(&mut self, address:String) { self.ip_address = address }
    fn get_port(&self) -> &String { &self.server_port }
    fn set_port(&mut self, port:String) { self.server_port = port }
    fn get_socket(&self) -> &i16{ &self.master_socket_descriptor }
    fn set_socket(&mut self, socket:i16) { self.master_socket_descriptor = socket}
    fn get_num_connections(&self) -> &i16 { &self.connected_users }
    fn set_num_connections(&mut self, users:i16) { self.connected_users = users }

    fn add_registered_user(&mut self, user_info:(String, User)) { self.registered_users.insert(user_info.0, user_info.1); }
    fn get_user_details(&self, username:String) -> Option<&User> {
        if self.registered_users.contains_key(&username) {
            return self.registered_users[username];
        }

        return None
    }
    fn add_new_connection(&mut self, connection:(thread, User)) {
        self.open_connections.insert(connection.0, connection.1);
        self.connected_users += 1;
    }
    fn close_connection(&self, connection_thread:thread, socket_descriptor:i16) {

    }
    fn process_message(&self) {}
    fn send_all_users(&self) {}
    fn send_friends(&self) {}
    fn sent_message(&self) {}
    fn register_new_user(&self) {}
    fn login_user(&self) {}
    fn send_invitation(&self) {}
    fn send_invite_acceptance(&self) {}

    fn SIGINT_handler(&self, s:i16) {
        if (s) {
            println!("SIGINT received, shutting down.");
            self.shutdown()
        }
    }
}

fn process_connection() {}

fn main() -> Result<()> {
    let mut server = Server{
        user_info_file: "".to_string(),
        ip_address: "".to_string(),
        server_port: "".to_string(),
        master_socket_descriptor: -1,
        registered_users: HashMap::new(),
        open_connections: HashMap::new(),
        connected_users: -1
    };

    // handle command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("messenger_server\n");
        println!("Usage:\tmessenger_server <path to user_info_file> <path to config_file>");
        println!("\tuser_info_file: path to file containing config parameters");
        println!("\tconfig_file: path to file containing config parameters");
        return Ok(())
    }

    println!("Messenger server starting.");

    // TODO: if we panic or error, we need to exit on these functions
    server.setup_config(args[2].to_string());
    setup.setup_users(args[1].to_string());
    setup.init_server();

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
    let mut buffer = [0; 512];
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