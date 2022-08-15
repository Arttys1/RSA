mod primality;
mod prime_generator;
mod tools;
mod rsa;
mod network;

use network::network::{NetworkListener, NetworkWriter};
use crate::primality::primality::{ is_prime_ };
use crate::prime_generator::gen::{ generator };
use crate::rsa::rsa::{generate, encrypt_tab, decrypt_tab};

use chrono::prelude::{DateTime, Local};

fn main() {
    //get users args and check if their are enough
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <mode> [number]", args[0]);
        println!("--help to have more informations");
        return;
    }    

    //get the local time at the beginning of the program
    let start: DateTime<Local> = Local::now();

    //Process
    match args[1].as_str() {
        "--help" => { help(); },
        "--primality" => { 
            if args.len() < 3 {
                println!("Usage: {} {} <number>", args[0], args[1]);
                return;
            }
            check_primality(&args[2]); 
        },
        "--generator" => { 
            if args.len() < 3 {
                println!("Usage: {} {} <number>", args[0], args[1]);
                return;
            }
            generate_prime(&args[2]);
        },
        "--encrypt" => {
        if args.len() < 3 {
            println!("Usage: {} {} <message>", args[0], args[1]);
            return;
        }
        encrypt(&args[2]);
        }
        "--receiver" => {
            active_receiver();
        }
        "--sender" => {
            active_sender();
        }

        _ => { println!("Unknown mode. Use --help to have more informations"); },
    };  
    
    //print the delta time at the end of the program
    let delta_time: chrono::Duration = Local::now() - start;
    let mut  millis: i64 = delta_time.num_milliseconds();
    let s: i64 = millis / 1000;
    millis %= 1000;
    println!("Operation performed in {}s {}ms", s, millis);
}

fn help() {
    println!("Welcome to a prime number tools !");
    println!("");
    println!("--primality : check if a number is prime");
    println!("--generator : generate a prime number");
    println!("--encrypt : encrypt and uncrypt a message");
    println!("--receiver : launch a crypted TCP server");
    println!("--sender : connect to the receiver and allow to send crypted message");
    println!("--help : display this help");
}

fn active_receiver() {
    let listener = NetworkListener::new("127.0.0.1:1234");
    listener.listen();
}

fn active_sender() {
    let mut writer = NetworkWriter::new("127.0.0.1:1234");    
    writer.listen();
}

fn encrypt(s: &String) {
    let (public, private) = generate();

    println!("public key : {}|{}", public.e(), public.n());
    println!("private public : {}", private.d());

    let tab = s.as_bytes();
    let encrypted_message = encrypt_tab(tab, &public);
    println!("encrypted message : {:?}", encrypted_message);
    let decrypted_message = decrypt_tab(&encrypted_message, &public, &private);
    let mut decrypt_string : String = String::new();
    for i in 0..decrypted_message.len() {
        decrypt_string.push(decrypted_message[i] as u8 as char);
    }
    
    println!("decrypted message : {:?}", decrypt_string);
}

fn generate_prime(s : &String) {
    let n: u8 = match s.parse::<u8>() {
        Ok(n) => n,
        Err(_) => {
            println!("Error: {} is not a number", s);
            return;
        }
    };
    println!("Generating...");
    let prime: i128 = generator(n);
    println!("And... Here is your prime number : {}", prime);
}

fn check_primality(s: &String) {
    let n : u128 =  match s.parse::<u128>() {
        Ok(n) => n,
        Err(_) => {
            println!("{} is not a valid number", s);
            return;
        }
    };

    println!("checking primality...");
    println!("{} is {}", n, if is_prime_(n) { "prime" } else { "not prime" });
}