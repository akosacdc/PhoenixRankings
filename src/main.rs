use std::io;
use std::io::Write;
use inquire::Select;
use inquire::InquireError;
use skillratings::glicko::GlickoRating;

mod db;
use crate::db::PlayersDatabase;

fn list_all_players(players_database: &PlayersDatabase) {
    let players_list = players_database.list();
    for (name, glicko) in players_list {
        println!("{} {} {}", name, glicko.rating, glicko.deviation);
    }
}

fn add_new_player(players_database: &mut PlayersDatabase) {
    let mut name = String::new();

    // print!("Name of the new player: ");
    // io::stdout().flush().unwrap();
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");

    // players_database.add_new(name.trim().to_string());

    players_database.add_existing("a".to_string(), GlickoRating{rating: 1500.0, deviation: 200.0});
    players_database.add_existing("b".to_string(), GlickoRating{rating: 1400.0, deviation: 30.0});
    players_database.add_existing("c".to_string(), GlickoRating{rating: 1550.0, deviation: 100.0});
    players_database.add_existing("d".to_string(), GlickoRating{rating: 1700.0, deviation: 300.0});
}

fn add_new_tournament(players_database: &mut PlayersDatabase) {
    use skillratings::{
        glicko::{glicko, glicko_rating_period, GlickoConfig, GlickoRating},
        Outcomes,
    };

    let mut no_of_players_string = String::new();

    print!("How many players attended? ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut no_of_players_string)
        .expect("Failed to read line");

    for _ in 0..no_of_players_string.trim().parse().unwrap() {
        let mut name_of_the_player = String::new();

        print!("Name of the player: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name_of_the_player)
            .expect("Failed to read line");

        if !players_database.find_player(name_of_the_player.trim().to_string()) {
            players_database.add_new(name_of_the_player.trim().to_string())
        }

        let player_glicko = players_database.get_player_data(name_of_the_player.trim().to_string());

        let mut no_of_games = String::new();

        print!("How many games did they play? ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut no_of_games)
            .expect("Failed to read line");

        let mut results: Vec<(GlickoRating, Outcomes)> = vec![];

        for _ in 0..no_of_games.trim().parse().unwrap() {
            let mut name_of_the_opponent = String::new();

            print!("Name of the opponent: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut name_of_the_opponent)
                .expect("Failed to read line");

            if !players_database.find_player(name_of_the_opponent.trim().to_string()) {
                players_database.add_new(name_of_the_opponent.trim().to_string())
            }

            let opponent_glicko = players_database.get_player_data(name_of_the_opponent.trim().to_string());

            let mut outcome = String::new();

            while outcome.trim() != "win" && outcome.trim() != "lose" {
                outcome = String::new();

                print!("What was the outcome?(from the player's perspective) ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut outcome)
                    .expect("Failed to read line");
            }
             
            if outcome.trim() == "win" {
                results.push((opponent_glicko.unwrap(), Outcomes::WIN));
            } else {
                results.push((opponent_glicko.unwrap(), Outcomes::LOSS));
            }
        }

        let config = GlickoConfig::new();
        let new_glicko = glicko_rating_period(&(player_glicko.unwrap()), &results, &config);

        println!("{} {}", new_glicko.rating, new_glicko.deviation);
    }
}

fn main() {
    let mut players_database: PlayersDatabase = PlayersDatabase::new(vec![]);

    loop {
        let options: Vec<&str> = vec!["List all players", "Add new player", "Add new tournament", "Exit"];

        let ans: Result<&str, InquireError> = Select::new("Input a command: ", options).prompt();

        match ans {
            Err(_) => println!("There was an error, please try again"),
            Ok(choice) => {
                match choice {
                    "List all players" => list_all_players(&players_database),
                    "Add new player" => add_new_player(&mut players_database),
                    "Add new tournament" => add_new_tournament(&mut players_database),
                    "Exit" => break,
                    &_ => break,
                }
            },
        }
    }
    
}
