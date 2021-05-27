use mini_redis::{Frame, Result};
use tokio::net::TcpStream;

pub struct ConnectionVec {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl ConnectionVec {
    pub fn new(stream: TcpStream) -> Self {
        ConnectionVec {
            stream,
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            if self.buffer.len() == self.cursor {
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

            if n == 0 {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }
}
