use std::io::Write;
use std::fs::File;
use std::env;
use std::fs;
use rand::Rng;


static HEXFILL_CONTENTS: [&str; 6] = ["Nothing", "Nothing", "Nothing", "Settlement", "Lair", "Weird"];
static SETTLEMENT_POP: [i32; 6] = [100, 100, 100, 100, 600, 180];
static HEX_NOTES: [&str; 6] = ["Relationship to nearby hex", "Relationship to nearby hex", "Relationship to nearby hex", "A boon or ally.", "Hostile.",
 "Roll again & once on Weird"];
static WEIRD_FEATURES: [&str; 10] = ["Geography", "Magical Component", "Strange Merchant",
 "Strange Tutor", "Strange Ally", "Animal Behavior", "Clue to Nearby Hex", "Historical Location", "Treasure", "Roll Twice and Combine"];

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

fn hex_contents_gen() -> str {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
}

fn settlement_sop_gen() -> str {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
}

fn settlement_notes_gen() -> str {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
}

fn werid_gen() -> str {
    //Generates a random number
    //Returns the value in Gearing's 
    //Table
}

fn main() {

    let args: Vec<String> = env::args().collect();

	let parsed_args = parse_config(&args);
	let file = File::create("Hexfill.txt");

    

	let ret = 0;
} 
