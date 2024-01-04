use std::io;
use std::io::Write;
use whoami;
use clearscreen;

fn get_input(inp: String) -> String {
    print!("{}", inp.to_string());
    io::stdout().flush().expect("Failed to flush stdout");

    let mut user_cmd = String::new();
    io::stdin().read_line(&mut user_cmd).expect("Failed to read line");

    // Trim whitespace and newlines from the cmd
    user_cmd.trim().to_string()
}

fn commands(cmd: String) {

    if cmd == "help" {
            
        println!("");
        println!("STANDALONE COMMANDS: ");
        println!("help      lists all commands");
        println!("echo      echos user input");
        println!("exit      exits the shell");
        println!("clear     clears the shell");
        println!("");

    } else if cmd == "echo" {

        let text = get_input("cmd: ".to_string());
        println!("{}", text);

    } else if cmd == "exit" {

        std::process::exit(0);

    } else if cmd == "clear" {

        let _ = clearscreen::clear();
    }

}

fn main() {
    
    let _ = clearscreen::clear();

    loop {
        let _hostname: String = whoami::hostname();
        let _username: String = whoami::username();
        let _command: String = _username + "@" + &_hostname + ": ";

        let input = get_input(_command);
        commands(input);
        
    }
}