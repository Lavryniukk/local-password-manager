use crate::db::database::{Database, PasswordDatabase};
pub fn handle_user_input(input: &mut String, password_db: &PasswordDatabase) {
    loop {
        input.clear();
        std::io::stdin().read_line(input).unwrap();
        let input = input.trim();
        match input {
            "add" => {
                println!("What is the name of the password?");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                println!("What is the password?");
                let mut password = String::new();
                std::io::stdin().read_line(&mut password).unwrap();
                let password = password.trim();
                password_db.add_password(name, password).unwrap();
                println!("Password added");
                ()
            }
            "get" => {
                println!("What is the name of the password?");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();
                let password = match password_db.get_password(name) {
                    Ok(password) => password,
                    Err(_) => {
                        println!("Password not found");
                        continue;
                    }
                };
                match password {
                    Some(password) => println!("Password: {}", password),
                    None => println!("Password not found"),
                }
                ()
            }
            "get_all" => {
                let names = password_db.get_all_password_names().unwrap();
                for name in names {
                    println!("Name: {}", name);
                }
            }
            "exit" => break,
            _ => println!("Invalid input"),
        }
    }
}
