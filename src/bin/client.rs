use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use unix_socket_rest::shared::{Person, Request, Response};
use rmp_serde::{from_slice, to_vec};
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    let mut stream = UnixStream::connect("/tmp/rust_uds.sock").await?;

    let request = menu();
    let encoded = to_vec(&request).unwrap();

    send_len_request(&mut stream, &encoded).await?;
    send_encoded(&mut stream, &encoded).await?;

    let len = get_len_response(&mut stream).await?;
    let response: Response = get_response(&mut stream, len).await?;

    match response {
        Response::Ok(person) => println!("Found person: {:?}", person),
        Response::Created => println!("Person created successfully."),
        Response::NotFound(msg) => println!("Error: {}", msg),
        Response::Deleted => println!("Person deleted successfully"),
        _ => println!("other")
    };

    Ok(())
}

async fn get_len_response(socket: &mut UnixStream) -> Result<usize, std::io::Error> {
    let mut len_buf = [0u8; 4];
    socket.read_exact(&mut len_buf).await?;
    Ok(u32::from_be_bytes(len_buf) as usize)
}

async fn get_response(socket: &mut UnixStream, len: usize) -> Result<Response, Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;
    let response: Response = from_slice(&buf)?;
    Ok(response)
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
        println!("3. Delete person");
        println!("4. Exit");
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
            "3" => {
                return Request::Delete(prompt_name())
            }
            "4" => {
                process::exit(0);
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
