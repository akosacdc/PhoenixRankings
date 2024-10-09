use std::io;
use std::io::Write;
use inquire::Select;
use inquire::InquireError;

mod db;
use crate::db::PlayersDatabase;

fn list_all_players(players_database: &PlayersDatabase) {
    let players_list = players_database.list();
    for (name, rating) in players_list {
        println!("{} {}", name, rating);
    }
}

fn add_new_player(players_database: &mut PlayersDatabase) {
    let mut name = String::new();

    print!("Name of the new player: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    players_database.add_new(name.trim().to_string());
}

fn add_new_tournament() {
    println!("TO-DO");
}

fn main() {
    let mut players_database: PlayersDatabase = PlayersDatabase::new(vec![]);

    loop {
        let options: Vec<&str> = vec!["List all players", "Add new player", "Add new tournament", "Exit"];

        let ans: Result<&str, InquireError> = Select::new("What's your favorite fruit?", options).prompt();

        match ans {
            Err(_) => println!("There was an error, please try again"),
            Ok(choice) => {
                match choice {
                    "List all players" => list_all_players(&players_database),
                    "Add new player" => add_new_player(&mut players_database),
                    "Add new tournament" => add_new_tournament(),
                    "Exit" => break,
                    &_ => break,
                }
            },
        }
    }
    
}
