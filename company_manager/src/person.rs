pub enum Gender{
	Male,
	Female,
	GenderFluid,
	Nonbinary,
	Other(String)
}

pub struct Person{
	pub name: String,
	pub age: u8,
	pub gender: Gender,
	pub id: usize
}

impl Gender{
	pub fn to_string(&self) -> String{
		match self{
			Gender::Male => "male",
			Gender::Female => "female",
			Gender::GenderFluid => "gender fluid",
			Gender::Nonbinary => "nonbinary",
			Gender::Other(name) => name
		}.to_string()
	}
}

impl Person{
	pub fn to_string(&self) -> String{
		self.name.clone() + ", " + &self.age.to_string() + ", " + &self.gender.to_string()
	}
}
