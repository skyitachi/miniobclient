use std::io;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::io::ErrorKind::ConnectionAborted;

pub struct RpcClient {
    connection: TcpStream,
    addr: String,
}

impl RpcClient {

    fn connect(&mut self) -> Result<(), io::Error>{
        self.connection = TcpStream::connect(&self.addr)?;
        Ok(())
    }

    pub fn send(&mut self, query: &str) -> Result<(), io::Error>{
        let mut i = 0;
        loop {
            let send_result = self.connection.write(query.as_bytes());
            match send_result {
                Ok(written) => {
                    println!("send {} bytes to server", written);
                    break;
                }
                Err(e) => {
                    if e.kind() == ErrorKind::BrokenPipe {
                        self.connect()?;
                    }
                }
            }
            i += 1;
            if i > 3 {
                break;
            }
        }
        Ok(())
    }

    pub fn new(addr: String) -> Result<RpcClient, io::Error> {
        Ok(RpcClient{
            addr: addr.clone(),
            connection: TcpStream::connect(&addr.clone())?
        })
    }
}
