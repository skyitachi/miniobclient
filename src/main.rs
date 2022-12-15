use std::fs::read;
use std::io::{ErrorKind, Write};
use std::net::TcpStream;
use std::process;
use std::thread::sleep;
use std::time::Duration;
use inquire::Text;
use crate::rpc::RpcClient;

mod rpc;

#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use std::{io, process};
    use crate::rpc::RpcClient;

    #[test]
    fn rpc_client_init() -> Result<(), io::Error>{
        let mut client = RpcClient::new(String::from("localhost:6789"))?;
        client.send("hello world\n")?;
        Ok(())
    }
}

fn main() {
    let mut client = RpcClient::new(String::from("localhost:6789")).unwrap();

    let exit_str = String::from("exit");
    let mut buffer : [u8; 8096] = [0;8096];
    loop {
        let input = Text::new("obclient >").prompt();
        match input {
            Ok(input) => {
                if input == exit_str {
                    break;
                }
                let send_result = client.send(&input);
                match send_result {
                    Ok(()) => {
                        let read_result = client.read(&mut buffer);
                        match read_result {
                            Ok(len) => {
                                let parsed = std::str::from_utf8(&buffer[0..len]).unwrap();
                                print!("{}", parsed);
                            }
                            Err(e) => {
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        continue;
                    }
                }
            },
            Err(_) => println!("error happened")
        }
    }
    println!("bye!!!")
}
