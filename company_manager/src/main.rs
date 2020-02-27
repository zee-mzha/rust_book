mod utils;
mod person;
mod company;

use termion;
use company::Company;

fn main(){
	let mut company = Company::new();

	loop{
		print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));
		println!("1. Create new department\n\
				2. Delete department\n\
				3. List departments\n\
				4. Add employee to department\n\
				5. Remove employee from department\n\
				6. List employees in department\n\
				7. List all departments and employees\n\
				8. Exit program");

		let choice = utils::get_trimmed_input();

		print!("{}{}", termion::clear::All, termion::cursor::Goto(1,1));

		let choice: i32 = match choice.parse(){
			Ok(num) => num,
			Err(_) => {
				println!("enter a number using only 0-9");
				utils::wait_for_enter();
				continue;
			}
		};

		match choice{
			1 => company.create_department(),
			2 => company.delete_department(),
			3 => company.list_departments(),
			4 => company.add_employee(),
			5 => company.remove_employee(),
			6 => company.list_employees(),
			7 => company.list_all(),
			8 => break,
			_ => {
				println!("Invalid option.");
			}
		}

		utils::wait_for_enter();
	}
}
