use std::io::{stdin, stdout, Read, Write};
use std::io;
use std::io::*;
use std::fmt;
use std::mem;
use std::vec;



fn pause() 
{
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn menu()
{

	println!("Choose an option below:");
	println!("-----------------------");
	println!("1: Basics/Display");
	println!("2: Formatting");
	println!("3: Primitive Types");
	println!("4: Arrays/Slices");
	println!("5: Structures");
	
	let mut inputNum = String::new();
	io::stdin().read_line(&mut inputNum);
	
	let mut menuChoiceNum: i32 = inputNum.trim().parse().expect("string num");
	
	
	if menuChoiceNum == 1												//will change these to integers later//
	{
		display();
	}
	
	else if menuChoiceNum == 2
	{
		format();
	}
	
	else if menuChoiceNum == 3
	{
		staticPrimTypes();
	}
	
	else if menuChoiceNum == 4
	{
		arraysAndSlices();
	}
	
	else if menuChoiceNum == 5
	{
		structures();
	}
	else
	{
	
		println!(" ");
		println!("that is not a correct choice.");
		pause();
	}
	
	
	
}

fn display()
{
	println!("Basics/Display");
	println!("--------------");
	
	println!("{} is a default argument", "THIS");
	
	println!("{0} is the first argument. {1} is the second", "That argument", "This argument");
	
	println!("{food}, {drink}, and {side} were made with named arguments",
		food = "burger",drink = "diet coke",side = "coleslaw");
		
	println!("{} of {:b} people know binary, the other half doesn't with this special formatting", 1, 2);
	
	println!("I also learned to space {out:>width$} a little bit", out = "out", width = 6);
	
	println!("If I want, I can make these spaces with zeroes, like {numbers:>0width$}", numbers = 1, width = 6);
	
	println!(" ");
	
	pause();
	
	
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
	
	let mut string = format!("{h}:{m}:{s}, on {mo} {d}, {y}", s=second,m=minute,h=hour,d=day,mo=month,y=year);
	println!("The current formatted date is {s}", s=string);
	
	pause();
		
}

fn staticPrimTypes()
{
	
	println!(" ");
	println!("Static Primitive Types");
	println!("----------------------");
	
	let theBool: bool = true;
	let aFloat: f64 = 1.11111111114;
	let anInt: i8 = 5;
	let mut inferrdInt = 15;
	inferrdInt = 333392;
	let mut mutableNum: i8 = 10;
	
	
	println!("the integer and mutable number's sum is = {}", anInt + mutableNum);
	println!("the integer and mutable number's difference is = {}", anInt - mutableNum);					
	println!("the integer and mutable number in binary is {0:b} and {1:b}", anInt, mutableNum);
	
	println!(" ");
	
	let mut basicTuple = (4, "tuples!", 5.5553, false);
	let specificTuple = (33.3, true, 39u32);
	let groupTuple = (("first tuple", 3, 44), ("second tuple", 5, 33), 99);
	let (num,string,float,bool) = basicTuple;
	
	println!("an example of a tuple is {:?}", basicTuple);
	println!("this is a tuple with mulitle members: {:?}", groupTuple);
	println!("the second value from the smaller tuple is {:?}", specificTuple.1);
	println!("we can create bindings to display '{:?}{:?}{:?}{:?}' from the tuple", num, string, float, bool); 
	
	pause();
	
}

fn borrowSlice(slice: &[i8])
{
	println!("the first element of the slice is {}", slice[0]);
	

}

fn arraysAndSlices()
{
	let mut simpleArray: [i8; 6] = [5,6,7,8,9,10]; 		//[type; size]//
	let mut	filledArray: [i8; 30] = [0; 30];			//fils array with 0s//
	
	println!(" ");
	println!("Arrays and Slices");
	println!("----------------");
	
	println!("the first and second element in the first array is {0} and {1}", simpleArray[0], simpleArray[1]);
	println!("the filled array is filled with {}'s", filledArray[0]);
	
	println!(" ");
	
	println!("slices are like arrays, except size is unknown (at compile time). slices can borrow from arrays.");
	borrowSlice(&filledArray);
	borrowSlice(&simpleArray[0..2]);
	
	pause();
	
}


struct Car<'m>											//apostraphe and m is used to get the size needed for the string//
{
	model: &'m str,
	year: i32,
}

struct Coord<>
{
	x: f32,
	y: f32,
}


fn structures()
{


	println!(" ");
	println!("Structures");
	println!("----------");
	
	let carModel = "Pontiac Firebird Trans Am";
	let carYear = 1972;
	let carInstance = Car { model: carModel, year: carYear };
	
	println!("this is an example of a basic structure.");
	println!("the car is a {1} {0}", carInstance.model, carInstance.year);
	
	println!("sometimes there are longer ways of instantiating a structure.");
	let coordInstance: Coord = Coord {x: 0.4, y: 0.6};
	println!("the coordinates are {} {}", coordInstance.x, coordInstance.y);
	let newCoord = Coord {x: 0.8, ..coordInstance};									//this keeps the y coord the same as the last one//
	println!("the new coordinates are {} {}", newCoord.x, newCoord.y);
	
	pause();
	
	
	
}

fn main() 
{
    menu();
}

// ///////// //
// MISC CODE //
// ///////// //

//let mut numOfMenus = 5;//
//let mut menuVector = Vec::new();//

//for i in 1..=numOfMenus//
	//{//
	//	menuVector.push(i);//
	//}//
	
	//for i in 0..numOfMenus//
	//{//
		//if inputNum == menuVector[i]//
		//{//
			
		//}//
	//}//
		
