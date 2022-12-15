use std::io;
use std::net::TcpStream;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
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
            let mut null_based_str = String::from(query);
            null_based_str.push_str("\0");
            let send_result = self.connection.write(null_based_str.as_bytes());
            match send_result {
                Ok(_written) => {
                    break;
                }
                Err(e) => {
                    if e.kind() == ErrorKind::BrokenPipe {
                        self.connect()?;
                    } else {
                        return Err(e)
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

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        let mut i = 0;
        loop {
            let result = self.connection.read(buf);
            match result {
                Ok(len) => {
                    return Ok(len);
                }
                Err(e) => {
                    if e.kind() == ErrorKind::BrokenPipe {
                        self.connect()?;
                    } else {
                        return Err(e)
                    }
                }
            }
            i += 1;
            if i > 3 {
                break;
            }
        }
        Ok(0)
    }

    pub fn new(addr: String) -> Result<RpcClient, io::Error> {
        Ok(RpcClient{
            addr: addr.clone(),
            connection: TcpStream::connect(&addr.clone())?
        })
    }
}
