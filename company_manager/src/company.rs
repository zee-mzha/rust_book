use std::collections::HashMap;
use crate::person;
use crate::utils;

pub struct Company{
	structure: HashMap<String, HashMap<String, person::Person>>,
	current_id: usize
}

impl Company{
	pub fn new() -> Company{
		Company{
			structure: HashMap::new(),
			current_id: 0
		}
	}

	pub fn create_department(&mut self){
		print!("department name: ");
		utils::flush_stdout();
		
		let name = utils::get_trimmed_input();
		self.structure.entry(name).or_insert(HashMap::new());
	}
	
	pub fn delete_department(&mut self){
		print!("department name: ");
		utils::flush_stdout();
	
		let name = utils::get_trimmed_input();
		if let None = self.structure.remove(&name){
			println!("{} is not a valid department name", name);
		}
		else{
			println!("removed {} from department list", name);
		}
	}
	
	pub fn list_departments(&self){
		for department in self.structure.keys(){
			println!("{}", department);
		}
	}
	
	pub fn add_employee(&mut self){
		let department = self.get_department();

		print!("enter employee's name: ");
		utils::flush_stdout();
		let name = utils::get_trimmed_input();
	
		let age: u8 = loop{
			print!("enter employee's age: ");
			utils::flush_stdout();
	
			match utils::get_trimmed_input().parse(){
				Ok(num) => break num,
				Err(_) => {
					println!("enter a number using only 0-9");
					utils::wait_for_enter();
					print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
				}
			}
		};
	
		let gender = loop{
			print!("enter employee's gender: ");
			utils::flush_stdout();
	
			match utils::get_trimmed_input().chars().next().unwrap(){
				'm' | 'M' => break person::Gender::Male,
				'f' | 'F' => break person::Gender::Female,
				'g' | 'G' => break person::Gender::GenderFluid,
				'n' | 'N' => break person::Gender::Nonbinary,
				'o' | 'O' => {
					print!("enter gender name: ");
					utils::flush_stdout();
					let name = utils::get_trimmed_input();
					break person::Gender::Other(name)
				},
				_ => {
					println!("invaid input, enter (m)ale, (f)emale, (g)ender fluid, (n)onbinary, or (o)ther");
					utils::wait_for_enter();
					print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1,));
				}
			}
		};
	
		let person = person::Person{
			name: name.clone(),
			age,
			gender,
			id: self.current_id
		};

		match self.structure.get_mut(&department){
			Some(department) => department.insert(name+&self.current_id.to_string(), person),
			None => unreachable!()
		};
		self.current_id += 1;
	}
	
	pub fn remove_employee(&mut self){
		let department = self.get_department();

		print!("employee's name and id (format name123): ");
		utils::flush_stdout();

		let name_id = utils::get_trimmed_input();
		match self.structure.get_mut(&department).unwrap().remove(&name_id){
			Some(_) => {
				println!("removed {} from {}", name_id, department);
			},
			None => {
				println!("invalid employee name or id find employee!");
			}
		};
	}
	
	pub fn list_employees(&self){
		let department = self.get_department();

		println!("Employee's of {}: ", department);
		for (unique_id, person) in self.structure[&department].iter(){
			println!("\tunique id: {}", unique_id);
			println!("\t\tinfo: {}", person.to_string());
		}
	}
	
	pub fn list_all(&self){
		for(name, department) in self.structure.iter(){
			println!("Employee's of {}: ", name);
			for (unique_id, person) in department.iter(){
				println!("\tunique id: {}", unique_id);
				println!("\t\tinfo: {}", person.to_string());
			}
		}
	}

	fn get_department(&self) -> String{
		loop{
			print!("employee's department: ");
			utils::flush_stdout();
	
			let name = utils::get_trimmed_input();
			if self.structure.contains_key(&name){
				break name
			}
		}
	}
}
