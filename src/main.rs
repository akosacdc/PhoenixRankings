use std::io;
use std::io::Write;
use inquire::Select;
use inquire::InquireError;
use skillratings::{
    glicko::{glicko_rating_period, GlickoConfig, GlickoRating},
    Outcomes,
};
use csv::Writer;
use csv::Reader;

mod db;
use crate::db::PlayersDatabase;
use crate::db::TeamsDatabase;

fn list_all_players(players_database: &PlayersDatabase) {
    let players_list = players_database.list();
    for (name, glicko, results) in players_list {
        println!("{} {} {}", name, glicko.rating, glicko.deviation);
        for (opponent, outcome) in results {
            if outcome == Outcomes::WIN {
                println!("      {} {} win", opponent.rating, opponent.deviation);
            } else {
                println!("      {} {} loss", opponent.rating, opponent.deviation);
            }
        }
    }
}

fn list_all_teams(teams_database: &TeamsDatabase) {
    let teams_list = teams_database.list();
    for (player1_name, player2_name, glicko, results) in teams_list {
        println!("{} {} {} {}", player1_name, player2_name, glicko.rating, glicko.deviation);
        for (opponent, outcome) in results {
            if outcome == Outcomes::WIN {
                println!("      {} {} win", opponent.rating, opponent.deviation);
            } else {
                println!("      {} {} loss", opponent.rating, opponent.deviation);
            }
        }
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

    // players_database.add_existing("a".to_string(), GlickoRating{rating: 1500.0, deviation: 200.0});
    // players_database.add_existing("b".to_string(), GlickoRating{rating: 1400.0, deviation: 30.0});
    // players_database.add_existing("c".to_string(), GlickoRating{rating: 1550.0, deviation: 100.0});
    // players_database.add_existing("d".to_string(), GlickoRating{rating: 1700.0, deviation: 300.0});
}

fn add_new_tournament(players_database: &mut PlayersDatabase) {

    let mut wtr = Writer::from_path("proba.csv").unwrap();
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

        wtr.write_record(&[name_of_the_player.trim(), &new_glicko.rating.round().to_string(), &new_glicko.deviation.round().to_string()]);
    }
}

fn add_set_of_tournament_games_singles(players_database: &mut PlayersDatabase) {
    let mut name_of_the_player = String::new();
    let mut name_of_the_opponent = String::new();
    let mut outcome = String::new();

    loop {
        name_of_the_player = String::new();
        name_of_the_opponent = String::new();
        print!("Name of the player: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name_of_the_player)
            .expect("Failed to read line");

        if name_of_the_player.trim() == "exit" {
            break;
        }

        print!("Name of the opponent: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut name_of_the_opponent)
            .expect("Failed to read line");

        if !players_database.find_player(name_of_the_player.trim().to_string()) {
            players_database.add_new(name_of_the_player.trim().to_string())
        }
        if !players_database.find_player(name_of_the_opponent.trim().to_string()) {
            players_database.add_new(name_of_the_opponent.trim().to_string())
        }

        outcome = String::new();
        while outcome.trim() != "win" && outcome.trim() != "lose" {
            outcome = String::new();

            print!("What was the outcome?(from the player's perspective) ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut outcome)
                .expect("Failed to read line");
        }

        let player_glicko = players_database.get_player_data(name_of_the_player.trim().to_string());
        let opponent_glicko = players_database.get_player_data(name_of_the_opponent.trim().to_string());
        
        if outcome.trim() == "win" {
            players_database.add_new_result(name_of_the_player.trim().to_string(), opponent_glicko.unwrap(), Outcomes::WIN);
            players_database.add_new_result(name_of_the_opponent.trim().to_string(), player_glicko.unwrap(), Outcomes::LOSS);
        } else {
            players_database.add_new_result(name_of_the_player.trim().to_string(), opponent_glicko.unwrap(), Outcomes::LOSS);
            players_database.add_new_result(name_of_the_opponent.trim().to_string(), player_glicko.unwrap(), Outcomes::WIN);
        }
    }
}

fn add_set_of_tournament_games_open_doubles(players_database: &mut PlayersDatabase, teams_database: &mut TeamsDatabase) {
    let mut rdr = Reader::from_path("data/input_doubles_september.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        if !players_database.find_player(record.get(0).unwrap().to_string()) {
            players_database.add_new(record.get(0).unwrap().to_string());
        }
        if !players_database.find_player(record.get(1).unwrap().to_string()) {
            players_database.add_new(record.get(1).unwrap().to_string());
        }
        if !players_database.find_player(record.get(2).unwrap().to_string()) {
            players_database.add_new(record.get(2).unwrap().to_string());
        }
        if !players_database.find_player(record.get(3).unwrap().to_string()) {
            players_database.add_new(record.get(3).unwrap().to_string());
        }

        let team1_player1_glicko = players_database.get_player_data(record.get(0).unwrap().to_string());
        let team1_player2_glicko = players_database.get_player_data(record.get(1).unwrap().to_string());
        let team2_player1_glicko = players_database.get_player_data(record.get(2).unwrap().to_string());
        let team2_player2_glicko = players_database.get_player_data(record.get(3).unwrap().to_string());

        if !teams_database.find_team(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string()) {
            teams_database.add_new(record.get(0).unwrap().to_string(), team1_player1_glicko.unwrap(), record.get(1).unwrap().to_string(), team1_player2_glicko.unwrap());
        }
        if !teams_database.find_team(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string()) {
            teams_database.add_new(record.get(2).unwrap().to_string(), team2_player1_glicko.unwrap(), record.get(3).unwrap().to_string(), team2_player2_glicko.unwrap());
        }

        let team1_glicko = teams_database.get_team_glicko(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string());
        let team2_glicko = teams_database.get_team_glicko(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string());

        if record.get(4).unwrap() == "win" {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::WIN);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::LOSS);
        } else {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::LOSS);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::WIN);
        }
    }
}

fn add_set_of_tournament_games_mixed_doubles(players_database: &mut PlayersDatabase, teams_database: &mut TeamsDatabase) {
    let mut rdr = Reader::from_path("data/input_mixed_doubles_september.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        if !players_database.find_player(record.get(0).unwrap().to_string()) {
            players_database.add_new(record.get(0).unwrap().to_string());
        }
        if !players_database.find_player(record.get(1).unwrap().to_string()) {
            players_database.add_new(record.get(1).unwrap().to_string());
        }
        if !players_database.find_player(record.get(2).unwrap().to_string()) {
            players_database.add_new(record.get(2).unwrap().to_string());
        }
        if !players_database.find_player(record.get(3).unwrap().to_string()) {
            players_database.add_new(record.get(3).unwrap().to_string());
        }

        let team1_player1_glicko = players_database.get_player_data(record.get(0).unwrap().to_string());
        let team1_player2_glicko = players_database.get_player_data(record.get(1).unwrap().to_string());
        let team2_player1_glicko = players_database.get_player_data(record.get(2).unwrap().to_string());
        let team2_player2_glicko = players_database.get_player_data(record.get(3).unwrap().to_string());

        if !teams_database.find_team(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string()) {
            teams_database.add_new(record.get(0).unwrap().to_string(), team1_player1_glicko.unwrap(), record.get(1).unwrap().to_string(), team1_player2_glicko.unwrap());
        }
        if !teams_database.find_team(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string()) {
            teams_database.add_new(record.get(2).unwrap().to_string(), team2_player1_glicko.unwrap(), record.get(3).unwrap().to_string(), team2_player2_glicko.unwrap());
        }

        let team1_glicko = teams_database.get_team_glicko(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string());
        let team2_glicko = teams_database.get_team_glicko(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string());

        if record.get(4).unwrap() == "win" {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::WIN);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::LOSS);
        } else {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::LOSS);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::WIN);
        }
    }
}

fn add_set_of_tournament_games_monster_dyp(players_database: &mut PlayersDatabase, teams_database: &mut TeamsDatabase) {
    let mut rdr = Reader::from_path("data/input_monster_september.csv").unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        if !players_database.find_player(record.get(0).unwrap().to_string()) {
            players_database.add_new(record.get(0).unwrap().to_string());
        }
        if !players_database.find_player(record.get(1).unwrap().to_string()) {
            players_database.add_new(record.get(1).unwrap().to_string());
        }
        if !players_database.find_player(record.get(2).unwrap().to_string()) {
            players_database.add_new(record.get(2).unwrap().to_string());
        }
        if !players_database.find_player(record.get(3).unwrap().to_string()) {
            players_database.add_new(record.get(3).unwrap().to_string());
        }

        let team1_player1_glicko = players_database.get_player_data(record.get(0).unwrap().to_string());
        let team1_player2_glicko = players_database.get_player_data(record.get(1).unwrap().to_string());
        let team2_player1_glicko = players_database.get_player_data(record.get(2).unwrap().to_string());
        let team2_player2_glicko = players_database.get_player_data(record.get(3).unwrap().to_string());

        if !teams_database.find_team(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string()) {
            teams_database.add_new(record.get(0).unwrap().to_string(), team1_player1_glicko.unwrap(), record.get(1).unwrap().to_string(), team1_player2_glicko.unwrap());
        }
        if !teams_database.find_team(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string()) {
            teams_database.add_new(record.get(2).unwrap().to_string(), team2_player1_glicko.unwrap(), record.get(3).unwrap().to_string(), team2_player2_glicko.unwrap());
        }

        let team1_glicko = teams_database.get_team_glicko(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string());
        let team2_glicko = teams_database.get_team_glicko(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string());

        if record.get(4).unwrap() == "win" {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::WIN);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::LOSS);
        } else {
            teams_database.add_new_result(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string(), team2_glicko.unwrap(), Outcomes::LOSS);
            teams_database.add_new_result(record.get(2).unwrap().to_string(), record.get(3).unwrap().to_string(), team1_glicko.unwrap(), Outcomes::WIN);
        }
    }
}

fn calculate_players_glicko(players_database: &PlayersDatabase) {
    let mut wtr = Writer::from_path("data/output.csv").unwrap();
    let players_list = players_database.list();
    for (name, glicko, results) in players_list {
        let config = GlickoConfig::new();
        let new_glicko = glicko_rating_period(&glicko, &results, &config);

        wtr.write_record(&[name.trim(), &new_glicko.rating.round().to_string(), &new_glicko.deviation.round().to_string()]);
    }
}

fn calculate_teams_glicko(teams_database: &TeamsDatabase, players_database: &mut PlayersDatabase) {
    let mut wtr = Writer::from_path("data/output_monster.csv").unwrap();
    let teams_list = teams_database.list();
    for (name1, name2, glicko, results) in teams_list {
        let config = GlickoConfig::new();
        let new_glicko = glicko_rating_period(&glicko, &results, &config);
        let team_glicko_diff_per_player = GlickoRating {
			rating: (new_glicko.rating - glicko.rating) / 2.0,
			deviation: (new_glicko.deviation - glicko.deviation) / 2.0,
		};

        let mut player1_glicko = players_database.get_player_data(name1.clone()).unwrap();
        player1_glicko.rating += team_glicko_diff_per_player.rating;
        player1_glicko.deviation += team_glicko_diff_per_player.deviation;
        players_database.update_player_glicko(name1.clone(), player1_glicko);
        let mut player2_glicko = players_database.get_player_data(name2.clone()).unwrap();
        player2_glicko.rating += team_glicko_diff_per_player.rating;
        player2_glicko.deviation += team_glicko_diff_per_player.deviation;
        players_database.update_player_glicko(name2.clone(), player2_glicko);
    }
    let players_list = players_database.list();
    for (name, glicko, _) in players_list {
        wtr.write_record(&[name.trim(), &glicko.rating.round().to_string(), &glicko.deviation.round().to_string()]);

    }
}

fn main() {
    let mut players_database: PlayersDatabase = PlayersDatabase::new(vec![]);
    let mut teams_database: TeamsDatabase = TeamsDatabase::new(vec![]);

    loop {
        let options: Vec<&str> = vec!["List all players", "List all teams", "Add new player", "Add new tournament", "Add Singles tournament games", "Read Open Doubles games from csv", "Read Mixed Doubles games from csv", "Read Monster DYP games from csv", "Calculate players Glicko!", "Calculate teams Glicko!", "Exit"];

        let ans: Result<&str, InquireError> = Select::new("Input a command: ", options).prompt();

        match ans {
            Err(_) => println!("There was an error, please try again"),
            Ok(choice) => {
                match choice {
                    "List all players" => list_all_players(&players_database),
                    "List all teams" => list_all_teams(&teams_database),
                    "Add new player" => add_new_player(&mut players_database),
                    "Add new tournament" => add_new_tournament(&mut players_database),
                    "Add Singles tournament games" => add_set_of_tournament_games_singles(&mut players_database),
                    "Read Open Doubles games from csv" => add_set_of_tournament_games_open_doubles(&mut players_database, &mut teams_database),
                    "Read Mixed Doubles games from csv" => add_set_of_tournament_games_mixed_doubles(&mut players_database, &mut teams_database),
                    "Read Monster DYP games from csv" => add_set_of_tournament_games_monster_dyp(&mut players_database, &mut teams_database),
                    "Calculate players Glicko!" => calculate_players_glicko(&players_database),
                    "Calculate teams Glicko!" => calculate_teams_glicko(&teams_database, &mut players_database),
                    "Exit" => break,
                    &_ => break,
                }
            },
        }
    }
    
}
