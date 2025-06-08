use tokio::net::UnixStream;
use unix_socket_rest::shared::{Person, Request, Response, get_data_len, get_data, send_len_request, send_encoded};
use rmp_serde::to_vec;
use std::io::{self, Write};
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut stream = UnixStream::connect("/tmp/rust_uds.sock").await?;
        
        let request = menu();
        let encoded = to_vec(&request).unwrap();

        send_len_request(&mut stream, &encoded).await?;
        send_encoded(&mut stream, &encoded).await?;

        let len = get_data_len(&mut stream).await?;
        let response: Response = get_data(&mut stream, len).await?;

        match response {
            Response::Ok(person) => println!("Found person: {:?}", person),
            Response::Created => println!("Person created successfully."),
            Response::List(people) => {
                println!("=== List of People ===");
                for person in people {
                    println!("- {} ({} years old)", person.name, person.age);
                }
            }
            Response::NotFound(msg) => println!("Error: {}", msg),
            Response::Deleted => println!("Person deleted successfully."),
            _ => println!("Other response."),
        };
    }
}

fn menu() -> Request {
    loop {
        println!("\n=== MENU ===");
        println!("1. Add person");
        println!("2. Get person by name");
        println!("3. List people");
        println!("4. Delete person");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => return Request::Post(prompt_for_person()),
            "2" => return Request::Get(prompt_name()),
            "3" => return Request::List,
            "4" => return Request::Delete(prompt_name()),
            "5" => {
                println!("Exiting...");
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

