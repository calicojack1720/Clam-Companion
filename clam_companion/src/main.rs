/*
 Clam Companion
 Automates running Clam AV scans and creating conf files for Clam AV.
 Created by calicojack1720
 Created: 5/30/24
 Updated: 5/31/24
*/

use std::io;

fn main() {
    let mut choice: i16 = -1;
    let mut choice_input = String::new();

	while choice != 0 {
		io::stdin()
			.read_line(&mut choice_input)
			.expect("Failed to read line");
	}
}
