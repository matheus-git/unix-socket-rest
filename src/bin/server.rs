use tokio::net::{UnixListener, UnixStream};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use unix_socket_rest::shared::{Person, Request};
use rmp_serde::{from_slice, to_vec};

#[derive(Debug, Default)]
struct ListPerson {
    persons: Vec<Person>
}

impl ListPerson {
    fn find_by_name(&self, name: String) -> Option<&Person> {
        self.persons.iter().find(|person| person.name == name)
    }

    fn add(&mut self, person: Person) {
        self.persons.push(person)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_uds.sock";
    let _ = fs::remove_file(path); 

    let listener = UnixListener::bind(path)?;

    let list_person = Arc::new(Mutex::new(ListPerson::default()));

    loop {
        let (mut socket, _addr) = listener.accept().await?;
        let list_person = Arc::clone(&list_person);

        tokio::spawn(async move {
            let len = match get_len_request(&mut socket).await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to read length: {}", e);
                    return;
                }
            };

            let request = match get_request(&mut socket, len).await {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to read request: {}", e);
                    return;
                }
            };

            let mut list = list_person.lock().unwrap();
            handle_request(&mut list, request);
            println!("{:?}", *list);
        });
    }
}

pub async fn get_len_request(socket: &mut UnixStream) -> Result<usize, std::io::Error> {
    let mut len_buf = [0u8; 4];
    socket.read_exact(&mut len_buf).await?;
    Ok(u32::from_be_bytes(len_buf) as usize)
}

pub async fn get_request(socket: &mut UnixStream, len: usize) -> Result<Request, Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;
    let request: Request = from_slice(&buf)?;
    Ok(request)
}

fn handle_request(list_person: &mut ListPerson, request: Request ){
    match request {
        Request::Get(name) => {
            list_person.find_by_name(name);
        },
        Request::Post(person) =>  {
            list_person.add(person);
        },
    }
}

