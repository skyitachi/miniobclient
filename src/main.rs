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

    loop {
        let send_result = client.send("hello world\n");
        match send_result {
            Ok(()) => {
                println!("send ok");
                sleep(Duration::from_secs(1));
            }
            Err(err) => {
                println!("send error {}", err);
                sleep(Duration::from_secs(2));
            }
        }

    }

    // let exit_str = String::from("exit");
    // loop {
    //     let input = Text::new("obclient >").prompt();
    //     match input {
    //         Ok(input) => {
    //             if input == exit_str {
    //                 break;
    //             }
    //             println!("hello {}", input)
    //         },
    //         Err(_) => println!("error happened")
    //     }
    // }
    // println!("bye!!!")
}
