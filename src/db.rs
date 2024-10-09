pub struct PlayersDatabase {
	list: Vec<(String, u16)>,
}

impl PlayersDatabase {
	pub fn new(in_list: Vec<(String, u16)>) -> PlayersDatabase {
		PlayersDatabase{ list: in_list }
	}

	pub fn add_new(&mut self, name: String) {
		self.list.push((name, 1200));
	}

	pub fn add(&mut self, name: String, rating: u16) {
		self.list.push((name, rating));
	}

	pub fn list(& self) -> Vec<(String, u16)> {
		self.list.clone()
	}
}
