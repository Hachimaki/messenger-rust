use std::{str, net::TcpStream, io::{
    self,
    prelude::*,
    BufReader,
    Result,
    Write
}, process};
use std::collections::HashMap;
use std::fs::File;
use user::User;

pub struct Client {
    server_host: String,
    server_port: String,
    ip_address: String,
    client_port: String,
    username: String,
    master_socket_descriptor: i16,
    listen_socket_descriptor: i16,
    // master_fd_set: fd_set,  // TODO: need to define and evaluate what an fd_set was
    max_fd: i16,
    connected_fds: Vec<i16>,
    // connected_users: HashMap<String, User>
}

impl Client {
    fn print_help(&self) {
        println!("To send message, use these commands:\n");
        println!("\th - View the command reference\n");
        println!("\tr - Register a new user\n");
        println!("\tl - Log in as an existing user\n");
        println!("\tm <username> <message> - Send a message to a given user\n");
        println!("\ti <username> <message> - Send an invitation to chat to a given user, with an optional message\n");
        println!("\tia <username> <message> - Accept an invitation to chat from a given user, with an optional message\n");
        println!("\texit - Exit the messenger client\n\n");
    }

    /**
     * Pull the config values from the user provided configuration file.
     * Config variables are in form "keyword: value", each on individual lines
     * NOTE: Only server_host and server_port are currently supported.
     *
     * @param: char *file - user provided file that contains config parameters
     */
    fn setup_client(&self, file_path: String) {
        let token: char;
        let args = [' '; 2];
        let line = [' '; 100];
        let host: String;
        let port: String;

        println!("Loading configuration file from file at {}.\n", file_path);

        // Try to open the given file
        let config_file = match File::open(file_path) {
            Ok(config_file) => config_file,
            Err(err) => {
                println!("Error reading from file at {}: {}", file_path, err);
                process::exit(1);
            }
        };

        // TODO: this might not be necessary
        // if config_file == null() {}

        // Parse the config parameters from the file
        // while () {};

        // Once we get here, we've finished reading the config and the file will close
        // once it goes out of scope
        println!("Client configuration loaded.\n");
    }

    fn init_client(&self) -> Result<()> {
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
    fn init_listen_socket(&self) {}
    fn process_server_message(&self) {}
    fn process_client_message(&self) {}
    fn register_new_user(&self) {}
    fn login_user(&self) {}
    fn send_invitation(&self) {}
    fn send_invite_acceptance(&self) {}
    fn send_message(&self) {}
    fn shutdown(&self) {
        println!("Shutting down...\n");

        // TODO: handle closing all open sockets
        println!("Client shutdown complete.\n");
        process::exit(0);
    }

    fn get_server_host(&self) -> &String { &self.server_host }
    fn set_server_host(&mut self, host: String) { self.server_host = host}
    fn get_server_port(&self) -> &String { &self.server_port }
    fn set_server_port(&mut self, port: String) { self.server_port = port}
    fn get_ip(&self) -> &String { &self.ip_address }
    fn set_ip(&mut self, address: String) { self.ip_address = address}
    fn get_port(&self) -> &String { &self.client_port }
    fn set_port(&mut self, port: String) { self.client_port = port}
    fn get_username(&self) -> &String { &self.username }
    fn set_username(&mut self, name: String) { self.username = name}
    fn get_socket(&self) -> &i16 { &self.master_socket_descriptor}
    fn set_socket(&mut self, socket_descriptor: i16) { self.master_socket_descriptor = socket_descriptor}
    fn get_listen_socket(&self) -> &i16 { &self.listen_socket_descriptor }
    fn set_listen_socket(&mut self, socket_descriptor: i16) { self.listen_socket_descriptor = socket_descriptor }
    fn get_master_fd_set(&self) -> &String { &self.master_fd_set }
    //fn set_master_fd_set(&mut self, fdset: fd_set) { self.master_fd_set = fdset }
    fn get_max_fd(&self) -> &i16 { &self.max_fd }
    fn set_max_fd(&mut self, fd: i16) { self.max_fd = fd }
    fn add_connected_user(&self, user_info: (String, User)) {}
    fn get_user_details(&self) {}
}

// fn is_empty(input: *char) -> bool {
//     while input != '\0' {
//         if !input.is_whitespace() {
//             return false;
//         }
//         // input++;
//     }
//     return true;
// }

// TODO: find a better way of doing this
fn sigint_handler(client: Client, signal: i16) {
    if signal {
        // Initially clearing a warning, but this might be improved
    };

    println!("SIGINT received.\n");
    client.shutdown();
}

fn main(argc: i16, argv: *mut char) {
    let client = Client{
        server_host: "".to_string(),
        server_port: "".to_string(),
        ip_address: "".to_string(),
        client_port: "".to_string(),
        username: "".to_string(),
        master_socket_descriptor: -1,
        listen_socket_descriptor: -1,
        // master_fd_set: (),  // TODO: need to evaluate and define what an fd_set was
        max_fd: -1,
        connected_fds: vec![],
        // connected_users: HashMap::from(),
    };

    // let buffer = char[100];
    // let rset = fd_set;

    if argc < 2 {
        println!("messenger_client\n\n");
        println!("Usage:\tmessenger_client <path_to_config_file>\n");
        println!("\tconfig_file: path to file containing config parameters\n");
        // exit(EXIT_SUCCESS);
    }

    // Intercept and handle SIGINT (terminal exit eents)
    // sigint_action.sa_handler = SIGINTHandler;
    // sigemptyset(&sigint_action.sa_mask);
    // sigint_action.sa_flags = SA_RESTART;

    // Intercept and handle exit statuses (SIGINT, etc.)
    // if (sigaction(SIGINT, &sigint_action, NULL) == -1) {
    //     eprintln!("Error handling interrupt: ");
    //     process::exit(1);
    // }

    println!("Messenger client starting.\n");

    client.setup_client(argv[1]);
    client.init_client().ok();

    println!("\nWelcome to the messenger client.\n\n");
    client.print_help();

    // while 1 {};

    // If we ever get here, we're outside the main loop and should shutdown the client
    client.shutdown();
}