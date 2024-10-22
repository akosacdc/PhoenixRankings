use skillratings::glicko::GlickoRating;
use skillratings::Outcomes;

pub struct PlayersDatabase {
	list: Vec<(String, GlickoRating, Vec<(GlickoRating, Outcomes)>)>,
}

impl PlayersDatabase {
	pub fn new(in_list: Vec<(String, GlickoRating, Vec<(GlickoRating, Outcomes)>)>) -> PlayersDatabase {
		PlayersDatabase{ list: in_list }
	}

	pub fn add_new(&mut self, name: String) {
		self.list.push((name, GlickoRating::new(), vec![]));
	}

	// pub fn add_existing(&mut self, name: String, rating: GlickoRating) {
	// 	self.list.push((name, rating));
	// }

	pub fn list(&self) -> Vec<(String, GlickoRating, Vec<(GlickoRating, Outcomes)>)> {
		self.list.clone()
	}

	pub fn find_player(&self, name: String) -> bool {
		for (db_name, _, _) in &self.list {
			if *db_name == name { return true; }
		}
		false
	}

	pub fn get_player_data(&self, name: String) -> Option<GlickoRating> {
		for (db_name, glicko_rating, _) in &self.list {
			if *db_name == name { return Some(*glicko_rating); }
		}
		None
	}

	pub fn add_new_result(&mut self, name: String, opponent: GlickoRating, outcome: Outcomes) {
		for (db_name, _, results) in &mut self.list {
			if *db_name == name { 
				(*results).push((opponent, outcome));
			}
		}
	}

	pub fn update_player_glicko(&mut self, name: String, glicko: GlickoRating) {
		for (db_name, glicko_rating, _) in &mut self.list {
			if *db_name == name { 
				*glicko_rating = glicko;
			}
		}
	}
}

pub struct TeamsDatabase {
	list: Vec<(String, String, GlickoRating, Vec<(GlickoRating, Outcomes)>)>,
}

impl TeamsDatabase {
	pub fn new(in_list: Vec<(String, String, GlickoRating, Vec<(GlickoRating, Outcomes)>)>) -> TeamsDatabase {
		TeamsDatabase{ list: in_list }
	}

	pub fn add_new(&mut self, player1_name: String, player1_glicko: GlickoRating, player2_name: String, player2_glicko: GlickoRating) {
		let team_rating = (player1_glicko.rating + player2_glicko.rating) / 2.0;
		let team_deviation = (player1_glicko.deviation + player2_glicko.deviation) / 2.0;
		let team_glicko = GlickoRating {
			rating: team_rating,
			deviation: team_deviation,
		};

		self.list.push((player1_name, player2_name, team_glicko, vec![]));
	}

	// pub fn add_existing(&mut self, name: String, rating: GlickoRating) {
	// 	self.list.push((name, rating));
	// }

	pub fn list(&self) -> Vec<(String, String, GlickoRating, Vec<(GlickoRating, Outcomes)>)> {
		self.list.clone()
	}

	pub fn find_team(&self, player1_name: String, player2_name: String) -> bool {
		for (db_name1, db_name2, _, _) in &self.list {
			if *db_name1 == player1_name && *db_name2 == player2_name { return true; }
			if *db_name2 == player1_name && *db_name1 == player2_name { return true; }
		}
		false
	}

	pub fn get_team_glicko(&self, player1_name: String, player2_name: String) -> Option<GlickoRating> {
		for (db_name1, db_name2, glicko, _) in &self.list {
			if *db_name1 == player1_name && *db_name2 == player2_name { return Some(*glicko); }
			if *db_name2 == player1_name && *db_name1 == player2_name { return Some(*glicko); }
		}
		None
	}

	pub fn add_new_result(&mut self, player1_name: String, player2_name: String, opponent: GlickoRating, outcome: Outcomes) {
		for (db_name1, db_name2, _, results) in &mut self.list {
			if *db_name1 == player1_name && *db_name2 == player2_name { 
				(*results).push((opponent, outcome));
			}
			if *db_name2 == player1_name && *db_name1 == player2_name {
				(*results).push((opponent, outcome));
			}
		}
	}
}
