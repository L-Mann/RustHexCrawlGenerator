use std::io::Write;
use std::fs::File;
use std::env;



static HEXFILL_CONTENTS: [&str; 6] = ["Nothing", "Nothing", "Nothing", "Settlement", "Lair", "Weird"];
static SETTLEMENT_POP: [i32; 6] = [100, 100, 100, 100, 600, 180];
static HEX_NOTES: [&str; 6] = ["Relationship to nearby hex ",
 	"Relationship to nearby hex ",
	"Relationship to nearby hex ",
	"A boon or ally. ",
	"Hostile. ",
	"Roll again & once on Weird "];
static WEIRD_FEATURES: [&str; 10] = ["Geography ",
"Magical Component ", "Strange Merchant ",
"Strange Tutor ", "Strange Ally ",
"Animal Behavior ", "Clue to Nearby Hex ",
"Historical Location ", "Treasure ",
"Roll Twice and Combine "];

struct Arguments {
	file_name: String,
    
    lower_bound_row: i32,
    lower_bound_column: i32,
    
    upper_bound_row: i32,
    upper_bound_column: i32,
}

fn random_number(lower_bound: i32, upper_bound: i32) -> i32 {
	let rng = rand::random_range(lower_bound..=upper_bound);
	return rng
}

fn parse_config(args: &Vec<String>) -> Arguments { 
	let file_output_name = &args[1];

	let left_bound_column = &args[2].parse::<i32>().expect("First bounds argument incorrect!");
	let left_bound_row = &args[3].parse::<i32>().expect("Second bounds argument incorrect!");
	
	let right_bound_column = &args[4].parse::<i32>().expect("Third bounds argument incorrect!");
	let right_bound_row = &args[5].parse::<i32>().expect("Fourth bounds argument incorrect!");

	return Arguments {file_name: String::from(file_output_name), lower_bound_row: *left_bound_row,
         lower_bound_column: *left_bound_column,
         upper_bound_row: *right_bound_row,
         upper_bound_column: *right_bound_column}

}

fn hex_contents_gen() -> String {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
    let rand_num: usize = random_number(0, 5).try_into().unwrap();
    return HEXFILL_CONTENTS[rand_num].to_string();
}

fn settlement_sop_gen() -> i32 {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
    let rand_num: usize = random_number(0, 5).try_into().unwrap();
    return random_number(0, SETTLEMENT_POP[rand_num]);
}

fn hex_notes_gen() -> String {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
	let rand_num: usize = random_number(0, 5).try_into().unwrap();

	if rand_num == 5 {
		let weird = weird_gen().to_string();
		let settlement_notes_str = hex_notes_gen().to_string();
		let return_str = String::new() + &weird + &settlement_notes_str;
		return return_str;
	}
	else {
		let return_str = HEX_NOTES[rand_num].to_string();
		return return_str; 
	}

}

fn weird_gen() -> String {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
	let rand_num: usize = random_number(0, 9).try_into().unwrap();

	if rand_num == 9 {
		let new_num: usize = random_number(0, 9).try_into().unwrap();
		let weird_one: String = WEIRD_FEATURES[new_num].to_string();
		let weird_two = weird_gen();
		let return_str = String::new() + &weird_one + &weird_two;
		return return_str
	}
	else {
		return WEIRD_FEATURES[rand_num].to_string();
	}
}

fn main() ->std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

	let parsed_args = parse_config(&args);
	let mut file = File::create(parsed_args.file_name)?;


	let loop_start_row = parsed_args.lower_bound_row;
	let loop_start_column = parsed_args.lower_bound_column;

	let loop_end_row = parsed_args.upper_bound_row;
	let loop_end_column = parsed_args.upper_bound_column;

	for row in loop_start_row..loop_end_row+1 {
		
		for column in loop_start_column..loop_end_column+1 {
			let contents = hex_contents_gen();

			if contents.eq(&"Nothing".to_string()) {
				
				let header = format!("## {}.{}\nEmpty\n\n", column, row);

				write!(file, "{}", header)?;
				
				continue
			}
			else if contents.eq(&"Settlement".to_string()) {

				let population: i32 = settlement_sop_gen();
				let notes: String = hex_notes_gen();

				let header = format!("## {}.{}\n", column, row);
				let body = format!("{}\nPopulation: {}\n\n", notes, population);

				write!(file, "{}{}", header, body)?;
				continue
			}
			else if contents.eq(&"Lair".to_string()) {
		
				let notes: String = hex_notes_gen();

				let header = format!("## {}.{}\n", column, row);
				let body = format!("Lair\nNotes: {}\n\n", notes);

				write!(file, "{}{}", header, body)?;
				continue
			}
			else if contents.eq(&"Weird".to_string()) {
				
				let weird = weird_gen();

				let header = format!("## {}.{}\n", column, row);
				let body = format!("Weird: {}\n\n", weird);

				write!(file, "{}{}", header, body)?;
				continue
			}
		}
	}

	Ok(())

    

	
} 
