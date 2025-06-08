use tokio::net::UnixListener;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;
use unix_socket_rest::shared::{Person, Request, Response, get_data_len, get_data, send_encoded, send_len_request};
use rmp_serde::to_vec;

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
            let len = match get_data_len(&mut socket).await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to read length: {}", e);
                    return;
                }
            };

            let request = match get_data(&mut socket, len).await {
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

