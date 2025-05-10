use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use unix_socket_rest::shared::{Person, Request};
use rmp_serde::{from_slice, to_vec};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/rust_uds.sock").await?;

    let request = menu();
    let encoded = to_vec(&request).unwrap();

    send_len_request(&mut stream, &encoded).await?;
    send_encoded(&mut stream, &encoded).await?;
    Ok(())
}

async fn send_len_request(stream: &mut UnixStream, encoded: &Vec<u8>) -> Result<(), std::io::Error> {
    let len_bytes = (encoded.len() as u32).to_be_bytes();

    stream.write_all(&len_bytes).await?;
    Ok(())
}

async fn send_encoded(stream: &mut UnixStream, encoded: &Vec<u8>) -> Result<(),std::io::Error> {
    stream.write_all(&encoded).await?;
    Ok(())
}

fn menu() -> Request {
    loop {
        println!("\n=== MENU ===");
        println!("1. Add person");
        println!("2. Get person by name");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => {
                return Request::Post(prompt_for_person())
            }
            "2" => {
                return Request::Get(prompt_name())
            }
            _ => println!("Invalid option, try again."),
        }
    }
}

fn prompt_name() -> String {
    let mut name = String::new();
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    name.trim().to_string()
}

fn prompt_for_person() -> Person {
    let mut name = String::new();
    print!("Enter name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    let mut age_str = String::new();
    print!("Enter age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age_str).unwrap();
    let age: u8 = age_str.trim().parse().expect("Invalid age");

    Person { name, age }
}
