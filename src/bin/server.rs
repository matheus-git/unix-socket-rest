use tokio::net::{UnixListener, UnixStream};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use unix_socket_rest::shared::{Person, Request, Response};
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

    fn remove_by_name(&mut self, name: &str) -> Option<Person> {
        if let Some(pos) = self.persons.iter().position(|p| p.name == name) {
            Some(self.persons.remove(pos))
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_uds.sock";
    fs::remove_file(path)?; 

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

            let mut list = list_person.lock().await;
            let response = handle_request(&mut list, request);
            let encoded = to_vec(&response).unwrap();

            if let Err(e) = send_len_request(&mut socket, &encoded).await {
                eprintln!("Failed to send length: {}", e);
            }
            if let Err(e) = send_encoded(&mut socket, &encoded).await {
                eprintln!("Failed to send response: {}", e);
            }
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

async fn send_len_request(socket: &mut UnixStream, encoded: &Vec<u8>) -> Result<(), std::io::Error> {
    let len_bytes = (encoded.len() as u32).to_be_bytes();

    socket.write_all(&len_bytes).await?;
    Ok(())
}

async fn send_encoded(socket: &mut UnixStream, encoded: &Vec<u8>) -> Result<(),std::io::Error> {
    socket.write_all(&encoded).await?;
    Ok(())
}

fn handle_request(list_person: &mut ListPerson, request: Request ) -> Response {
    match request {
        Request::Get(name) => {
            return match list_person.find_by_name(name) {
                Some(person) => Response::Ok(person.clone()),
                None => Response::NotFound("Not found".to_string())
            }
        },
        Request::Post(person) =>  {
            list_person.add(person);
            return Response::Created;
        },
        Request::Delete(name) => {
            return match list_person.remove_by_name(&name) {
                Some(_) => Response::Deleted,
                None => Response::NotFound("Person not found".into()),
            }
        },
        Request::List => {
            Response::List(list_person.persons.clone())
        }
    }
}

