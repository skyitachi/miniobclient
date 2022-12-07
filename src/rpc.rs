use std::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use std::io::ErrorKind::ConnectionAborted;

pub struct RpcClient {
    connection: TcpStream,
    addr: str,
}

impl RpcClient {

    fn connect(&mut self) {
        self.connection = TcpStream::connect(&self.addr)?;
        let mut reader = BufReader::new(&self.connection);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer);
    }

    fn send(&mut self, query: &str) {
        let mut i = 0;
        loop {
            if i > 3 {
                break;
            }
            let result = self.connection.write(query.as_bytes())?;

            match result {
                Ok(bytes_written) => {
                    println!("send ok");
                    break
                }
                Err(err) => {
                    if err == ConnectionAborted {
                        self.connect()
                    }
                }
            }
            i += 1
        }
    }
    // pub fn new(addr: &str) -> RpcClient {
    //     RpcClient{
    //         addr: *addr,
    //     }
    // }
}
