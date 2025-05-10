use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use unix_socket_rest::shared::Person;
use rmp_serde::{from_slice, to_vec};

#[derive(Debug, Default)]
struct ListPerson {
    peoples: Vec<Person>
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "/tmp/rust_uds.sock";
    let _ = fs::remove_file(path); 

    let listener = UnixListener::bind(path)?;
    println!("Servidor rodando em {}", path);

    let list_person = ListPerson::default();

    loop {
        let (mut socket, _addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut len_buf = [0u8; 4];
            if socket.read_exact(&mut len_buf).await.is_err() { return; }

            let len = u32::from_be_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            if socket.read_exact(&mut buf).await.is_err() { return; }

            let msg: Person = from_slice(&buf).unwrap();
            println!("Recebido: {:?}", msg);

            let response = to_vec(&msg).unwrap();
            let len = (response.len() as u32).to_be_bytes();
            let _ = socket.write_all(&len).await;
            let _ = socket.write_all(&response).await;
        });
    }
}

