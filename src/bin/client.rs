use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use unix_socket_rest::shared::Person;
use rmp_serde::{from_slice, to_vec};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/rust_uds.sock").await?;

    let msg = Person {
        name: "Ana".into(),
        age: 30,
    };

    let encoded = to_vec(&msg).unwrap();
    let len = (encoded.len() as u32).to_be_bytes();

    stream.write_all(&len).await?;
    stream.write_all(&encoded).await?;

    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_be_bytes(len_buf) as usize;

    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    let response: Person = from_slice(&buf).unwrap();

    println!("Resposta: {:?}", response);
    Ok(())
}

