#[allow(dead_code)]

pub mod network {
    use std::{net::{TcpListener, TcpStream}, io::{Write, Read}};
    use crate::rsa::rsa::{PrivateKey, PublicKey, generate, encrypt_tab, decrypt_tab};

    pub struct NetworkListener {
        listener: TcpListener,
    }

    pub struct NetworkWriter {
        stream: TcpStream,
    }

    impl NetworkListener {
        pub fn new(addr: &str) -> NetworkListener {
            let listener = match TcpListener::bind(addr) {
                Ok(listener) => listener,
                Err(e) => panic!("Error : {}", e),
            };
            NetworkListener { listener }
        }

        pub fn listen(&self) {
            for stream in self.listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("New connection: {}", stream.peer_addr().expect("unable to read addr"));

                        self.handle_stream(stream);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }

        fn handle_stream(&self, stream: TcpStream) {
            std::thread::spawn(move || {
                let mut network = NetworkWriter::construct(stream);
                let mut keys :(PublicKey<u128>, PrivateKey<u128>) = (PublicKey::construct(0, 0), PrivateKey::construct(0));
                let mut err = 0;
                loop {
                    match network.read().to_lowercase().as_str() {
                        "start" => {
                            println!("generate keys");
                            keys = generate(9);
                            let public_key_str = keys.0.to_string();
                            println!("{}", public_key_str);
                            network.write(public_key_str.as_str());
                        }
                        received => {
                            if !received.is_empty() {
                                //decrypt with private key
                                println!("received : {}", received);
                                let split = received.split("|");
                                let vec = NetworkListener::split_to_vec(split);
                                let decrypted = decrypt_tab(&vec, &keys.0, &keys.1);
                                let mut decrypt_string = String::new();
                                for i in 0..decrypted.len() {
                                    decrypt_string.push(decrypted[i] as u8 as char);
                                }
                                println!("decrypted : {}", decrypt_string);
                            }
                            else {
                                err += 1;
                                if err > 10 {
                                    println!("stop receive data");
                                    break;
                                }
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(200));
                }

                network.close();
            });
        }

        fn split_to_vec(split: std::str::Split<&str>) -> Vec<u128> {
            let mut res = Vec::<u128>::new();
            let parser = | s: &str | -> u128 {
                match s.parse::<u128>() {
                    Ok(x) => x,
                    Err(e) => panic!("{}", e),
                }
            };

            for s in split {
                if !s.is_empty() {
                    let u = parser(s);
                    res.push(u);
                }
            }

            res
        }

    }


    impl NetworkWriter {
        pub fn new(addr: &str) -> NetworkWriter {
            let stream = match TcpStream::connect(addr) {
                Ok(stream) => stream,
                Err(e) => panic!("Error : {}", e),
            };
            NetworkWriter { stream }
        }

        pub fn construct(stream : TcpStream) -> NetworkWriter {
            NetworkWriter { stream }
        }

        pub fn write(&mut self, message: &str) {
            match self.stream.write(message.as_bytes()) {
                Ok(size) => { println!("{} bytes sent", size); },
                Err(e) => println!("Error : {}", e),
            }
            match self.stream.flush() {
                Ok(_) => {},
                Err(e) => println!("Error : {}", e),
            }
        }

        pub fn read(&mut self) -> String {
            let mut buf : [u8; 2048] = [0; 2048];
            match self.stream.read(&mut buf) {
                Ok(size) => {
                    let received = match String::from_utf8(buf[0..size].to_vec()) {
                        Ok(received) => received,
                        Err(e) => panic!("Error : {}", e),
                    };
                    received
                }
                Err(_) => String::new(),
            }
        }

        pub fn close(&mut self) {
            match self.stream.shutdown(std::net::Shutdown::Both) {
                Ok(_) => println!("disconnect"),
                Err(e) => println!("Error : {e}"),
            }
        }

        fn parse_public_key(&self, received: &String) -> PublicKey<u128> {
            println!("{}", received);

            let parser = |i: usize| -> u128 {
                let mut split = received.split("||");
                match split.nth(i) {
                    Some(res) => match res.parse::<u128>() {
                            Ok(a) => a,
                            Err(e) => panic!("{}", e)
                        },
                    None => panic!("out of bound split"),
                }
            };
            let e = parser(2);
            let n = parser(1);
            PublicKey::construct(e, n)
        }

        pub fn listen(&mut self) {
            self.write("start");
            let mut received = self.read();
            while received.is_empty() { received = self.read() }

            let public = self.parse_public_key(&received);

            let get_input_user = || -> String {
                let mut input = String::new();
                let _in = std::io::stdin();
                _in.read_line(&mut input).expect("unable to get input");
                input
            };

            let mut input = get_input_user();
            while &input[0..&input.len() - 2] != "stop" {
                let encrypted = encrypt_tab(input.as_bytes(), &public);
                let mut message = String::new();
                for i in encrypted {
                    message.push_str(format!("{}|", i).as_str());
                }
                self.write(&message);

                input = get_input_user();
            }

            self.close();

        }
    }

}
