/*
 Clam Companion
 Automates running Clam AV scans and creating conf files for Clam AV.
 Created by calicojack1720
 Created: 5/30/24
 Updated: 6/8/24
*/

use std::io;
use std::cmp;

fn main() {
    let mut choice_input = String::new();

	println!("Welcome to Clam Companion!");
	
	while !(choice_input.eq("0")) {

		println!("\nClam Companion Menu - Please select from the following options:");
		println!("\n0 - Quit");
	
		io::stdin()
			.read_line(&mut choice_input)
			.expect("Failed to read line");

		choice_input.pop();
	}
}
