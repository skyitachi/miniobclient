use inquire::Text;

mod rpc;

fn main() {

    let exit_str = String::from("exit");
    loop {
        let input = Text::new("obclient >").prompt();
        match input {
            Ok(input) => {
                if input == exit_str {
                    break;
                }
                println!("hello {}", input)
            },
            Err(_) => println!("error happened")
        }
    }
    println!("bye!!!")
}
