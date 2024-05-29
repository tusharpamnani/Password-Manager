mod pass_entry;

use crate::pass_entry::{ServiceInfo, prompt, read_pass_from_file};

fn clr() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let ascii = r#"
    
  _____               __      __         _ _   
  |  __ \              \ \    / /        | | |  
  | |__) |_ _ ___ ___   \ \  / /_ _ _   _| | |_ 
  |  ___/ _` / __/ __|   \ \/ / _` | | | | | __|
  | |  | (_| \__ \__ \    \  / (_| | |_| | | |_ 
  |_|   \__,_|___/___/     \/ \__,_|\__,_|_|\__|


    "#;

    println!("{}", ascii);
}

fn main() {
    clr();
    loop {
        println!("Password Manager Menu:");
        println!("1. Add a new password entry");
        println!("2. View all password entries");
        println!("3. Search for a password entry");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                entry.write_to_file();
                println!("Added new password entry");
            }
            "2" => {
                clr();
                let services = read_pass_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password file: {}", err);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service: {}\n- Username: {}\n- Password: {}\n\n",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_pass_from_file().unwrap_or_else(|err| {
                    eprintln!("Error reading password file: {}", err);
                    Vec::new()
                });
                let search = prompt("Search: ");
                let search_lower = search.to_lowercase();
                let mut found = false;
                for item in &services {
                    if item.service.to_lowercase().contains(&search_lower) {
                        println!(
                            "Service: {}\n- Username: {}\n- Password: {}",
                            item.service, item.username, item.password
                        );
                        found = true;
                    }
                }
                if !found {
                    println!("No entries found for '{}'", search);
                }
            }
            "4" => {
                clr();
                println!("Good Bye! See you soon...");
                break;
            }
            _ => {
                clr();
                println!("Invalid choice, please try again");
            }
        }
        println!("\n\n");
    }
}
