use anyhow::Result;
use derive_more::Display;
use std::io::Cursor;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn read_var_int<T>(stream: &mut T) -> Result<i32>
where
    T: AsyncRead + std::marker::Unpin,
{
    let mut num_read: i32 = 0;
    let mut result: i32 = 0;
    loop {
        let read = stream.read_u8().await? as i32;
        let value = read & 0b0111_1111;
        result |= value << (7 * num_read);
        num_read += 1;
        if num_read > 5 {
            return Err(anyhow!("VarInt too big!"));
        }
        if (read & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(result)
}

pub async fn write_var_int<T>(stream: &mut T, mut value: i32) -> Result<()>
where
    T: AsyncWrite + std::marker::Unpin,
{
    loop {
        let mut temp: i16 = (value & 0b0111_1111) as i16;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        stream.write_i8(temp as i8).await?;
        if value == 0 {
            break Ok(());
        }
    }
}
pub async fn read_string<T>(stream: &mut T) -> Result<String>
where
    T: AsyncRead + std::marker::Unpin,
{
    let length = read_var_int(stream).await?;
    let mut buf = vec![0u8; length as usize];
    stream.read_exact(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[derive(Debug, Clone)]
pub struct HandshakeRequest {
    size: i32,
    raw_body: Vec<u8>,
    id: i32,
    version: i32,
    host: String,
    port: u16,
    next_state: NextState,
}

impl HandshakeRequest {
    pub async fn read(stream: &mut TcpStream) -> Result<Self> {
        let size = read_var_int(stream).await?;
        let mut raw_body = vec![0u8; size as usize];
        stream.read_exact(&mut raw_body).await?;
        let mut raw_body = Cursor::new(raw_body);
        let id = read_var_int(&mut raw_body).await?;
        if id != 0 {
            return Err(anyhow!("{} is not a id of handshake packet", id));
        }
        let version = read_var_int(&mut raw_body).await?;
        let host = read_string(&mut raw_body).await?;
        let port = raw_body.read_u16().await?;
        let next_state = NextState::from_i32(read_var_int(&mut raw_body).await?)?;
        Ok(Self {
            size,
            id,
            version,
            host,
            port,
            next_state,
            raw_body: raw_body.into_inner(),
        })
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_next_state(&self) -> &NextState {
        &self.next_state
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_raw_body(&self) -> &[u8] {
        &self.raw_body
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Display)]
pub enum NextState {
    Status,
    Login,
}

impl NextState {
    pub fn from_i32(num: i32) -> Result<NextState> {
        Ok(match num {
            1 => Self::Status,
            2 => Self::Login,
            _ => return Err(anyhow!("Cannot convert {} to NextState", num)),
        })
    }
}
