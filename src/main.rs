#[macro_use]
extern crate nom;

use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // ----------
	// Get filename from stdin
	println!("Enter filename: ");

	let mut filename = String::new();

	io::stdin().read_line(&mut filename)
		.expect("Something went wrong getting file name.");

	let filename = filename.trim();
	// end Get filename from stdin
	// ----------

	// ----------
	// Read file
	println!("Open file: \'{}\'", filename);

    let mut f = File::open(filename)
    	.expect("No such file.");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
    	.expect("Something went wrong reading file.");

    println!("-----\n{}\n-----\n", contents);

    let split = contents.split("\n");


    let mut counter = 0;
    let mut vec = Vec::new();

    for line in split {
    	counter = 1 + counter;
    	let length = line.chars().count();

    	if length > 0 { 
    		// println!("{} {} {}", counter, length, line);

    		vec.push(line);
    	}
    }
    // end Read file
    // ----------

    // ----------
    // Parse lines

    let ops = vec!["+", "-", "="];

    for sent in vec {

    	// let mut val = String::new();
    	let mut key = String::new();

    	let mut sentence = sent.to_string();

    	let mut ch = sentence.remove(0);

    	let mut glut = String::new(); // catch all chars in entity

    	loop {

    		let mut op_check = false;

    		for op in &ops {
    			if &ch == &op.to_string().remove(0) {
    				op_check = true;
    				key = glut;
    				glut = String::new();
    			}
    		}

    		if op_check == false {
    			glut.push(ch);
    		}


    		if sentence.chars().count() == 0 {
    			println!("{} : {}", key, glut);
    			break;
    		} else {
    			ch = sentence.remove(0);
    		}
    	}
    }

    // end Parse lines
    // ----------
}
