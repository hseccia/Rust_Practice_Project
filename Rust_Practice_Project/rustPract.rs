use std::io::{stdin, stdout, Read, Write};
use std::fmt;

fn pause() 
{
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn menu()
{
	display();
	format();
	pause();
}

fn display()
{
	println!("{} is a default argument", "THIS");
	
	println!("{0} is the first argument. {1} is the second", "That argument", "This argument");
	
	println!("{food}, {drink}, and {side} were made with named arguments",
		food = "burger",drink = "diet coke",side = "coleslaw");
		
	println!("{} of {:b} people know binary, the other half doesn't with this special formatting", 1, 2);
	
	println!("I also learned to space {out:>width$} a little bit", out = "out", width = 6);
	
	println!("If I want, I can make these spaces with zeroes, like {numbers:>0width$}", numbers = 1, width = 6);
	
	println!(" ");
	
	
}

fn format()
{
	
	let second = 40;
	let minute = 38;
	let hour = 10;
	
	let day = 24;
	let month = "April";
	let year = 2019;
	
	println!(" ");
	
	let mut string = format!("{h}:{m}:{s}, on {mo}, {d} {y}", s=second,m=minute,h=hour,d=day,mo=month,y=year);
	
	println!("The current formatted date is {s}", s=string);
		
}

fn main() 
{
    menu();
}

