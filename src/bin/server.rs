use tokio::net::UnixListener;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use unix_socket_rest::shared::Person;
use rmp_serde::{from_slice, to_vec};

#[derive(Debug, Default)]
struct ListPerson {
    persons: Vec<Person>
}

impl ListPerson {
    fn add(&mut self, person: Person) {
        self.persons.push(person)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_uds.sock";
    let _ = fs::remove_file(path); 

    let listener = UnixListener::bind(path)?;
    println!("Servidor rodando em {}", path);

    let list_person = Arc::new(Mutex::new(ListPerson::default()));

    loop {
        let (mut socket, _addr) = listener.accept().await?;
        let list_person = Arc::clone(&list_person);

        tokio::spawn(async move {
            let mut len_buf = [0u8; 4];
            if socket.read_exact(&mut len_buf).await.is_err() { return; }

            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            if socket.read_exact(&mut buf).await.is_err() { return; }

            let person: Person = from_slice(&buf).unwrap();
            println!("Recebido: {:?}", person);

            let mut list = list_person.lock().unwrap();
            handle_request(&mut list, person);
            println!("{:?}", *list);
        });
    }
}

fn handle_request(list_person: &mut ListPerson, person: Person){
   list_person.add(person); 
}

