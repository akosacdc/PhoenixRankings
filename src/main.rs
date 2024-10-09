use inquire::Select;
use inquire::InquireError;

fn list_all_players() {
    println!("TO-DO");
}

fn add_new_player() {
    println!("TO-DO");
}

fn add_new_tournament() {
    println!("TO-DO");
}

fn main() {
    loop {
        let options: Vec<&str> = vec!["List all players", "Add new player", "Add new tournament", "Exit"];

        let ans: Result<&str, InquireError> = Select::new("What's your favorite fruit?", options).prompt();

        match ans {
            Err(_) => println!("There was an error, please try again"),
            Ok(choice) => {
                match choice {
                    "List all players" => list_all_players(),
                    "Add new player" => add_new_player(),
                    "Add new tournament" => add_new_tournament(),
                    "Exit" => break,
                    &_ => break,
                }
            },
        }
    }
    
}
