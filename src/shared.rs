use serde::{Serialize, Deserialize};
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rmp_serde::from_slice;
use serde::de::DeserializeOwned;
use bincode::{Decode, Encode};

#[derive(Serialize, Decode, Encode, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Get(String),
    Post(Person),
    Delete(String),
    List
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Ok(Person),
    NotFound(String),
    Created,
    Deleted,
    List(Vec<Person>),
    Error(String),
}

pub async fn get_data_len(socket: &mut UnixStream) -> Result<usize, std::io::Error> {
    let mut len_buf = [0u8; 4];
    socket.read_exact(&mut len_buf).await?;
    Ok(u32::from_be_bytes(len_buf) as usize)
}

pub async fn get_data<T: DeserializeOwned>(
    socket: &mut UnixStream,
    len: usize,
) -> Result<T, Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; len];
    socket.read_exact(&mut buf).await?;
    let data: T = from_slice(&buf)?;
    Ok(data)
}

pub async fn send_len_request(socket: &mut UnixStream, encoded: &Vec<u8>) -> Result<(), std::io::Error> {
    let len_bytes = (encoded.len() as u32).to_be_bytes();
    socket.write_all(&len_bytes).await?;
    Ok(())
}

pub async fn send_encoded(socket: &mut UnixStream, encoded: &Vec<u8>) -> Result<(),std::io::Error> {
    socket.write_all(&encoded).await?;
    Ok(())
}
