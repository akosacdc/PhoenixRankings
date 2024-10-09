use skillratings::glicko::GlickoRating;

pub struct PlayersDatabase {
	list: Vec<(String, GlickoRating)>,
}

impl PlayersDatabase {
	pub fn new(in_list: Vec<(String, GlickoRating)>) -> PlayersDatabase {
		PlayersDatabase{ list: in_list }
	}

	pub fn add_new(&mut self, name: String) {
		self.list.push((name, GlickoRating::new()));
	}

	pub fn add_existing(&mut self, name: String, rating: GlickoRating) {
		self.list.push((name, rating));
	}

	pub fn list(&self) -> Vec<(String, GlickoRating)> {
		self.list.clone()
	}

	pub fn find_player(&self, name: String) -> bool {
		for (db_name, _) in &self.list {
			if *db_name == name { return true; }
		}
		false
	}

	pub fn get_player_data(&self, name: String) -> Option<GlickoRating> {
		for (db_name, glicko_rating) in &self.list {
			if *db_name == name { return Some(*glicko_rating); }
		}
		None
	}
}
