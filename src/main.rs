use std::fs;
use substring::Substring;
use std::num::ParseIntError;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


const CSV_DIRECTORY: &str = "CSV_files";

fn get_int_from_user() -> Result<i32, ParseIntError>{
    let mut line = String::new();
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    match line[0..line.len() - 2].parse::<i32>(){ 
        Ok(my_int) => Ok(my_int), 
        Err(e) => Err(e),
    }
}

fn read_files() -> Vec<String> {
    let mut vec = Vec::new();
    let paths = fs::read_dir(CSV_DIRECTORY).unwrap();
    let mut file_name: String;
    for path in paths {
        file_name = String::from(path.unwrap().path().display().to_string());
        file_name = String::from(file_name.substring(CSV_DIRECTORY.len() + 1, file_name.len() + 1));
        vec.push(file_name);
    }
    return vec;
}

fn print_files(files: &Vec<String>){
    let mut i = 1;
    for file_name in files {
        println!("{1}: {0}", file_name, i);
        i += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {

    println!("    Hello! This program is to help teach me more about rust\n\
            and to prepare me for my cyber security tests. For this program,\n\
            add CSV files to the CSV_files directory in the following format.\n\n\
            \t# Each new line is a new question for the flash cards\n\
            \t# Each line starts with a w or a m\n\
            \t# w stands for written question\n\
            \t# m stands for multiple choice question\n\
            \t# See Example.csv for an example.\n");
    println!("Listed below are your files\n");
    let mut user_input;    
    let vec = read_files();
    loop {
        print_files(&vec);
        println!("\nPlease select a file: ");
        user_input = get_int_from_user().unwrap();
        if !(user_input < 1 || user_input > vec.len() as i32) { break;}        
        println!("\nError! Number not in range.\n");
    }
    let file_name = String::from(&vec[(user_input - 1) as usize]);
    let file_name_path = String::from(CSV_DIRECTORY) + "\\" + &file_name;
    println!("\nOpening {}...\n", file_name);
    let mut i = 0;
    let mut answer: String = String::from("");
    if let Ok(lines) = read_lines(file_name_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if &ip[0..1] == "m"{
                    for value in String::from(&ip).split(","){
                        if i > 0{
                            if i == 2{
                                println!("Q: {}", value);
                                println!("Answers:")
                            }
                            else if i > 2{
                                println!("{}: {}", i - 2, value)
                            }
                            else if i == 1{
                                answer = String::from(value);
                            }
                        }
                        i += 1;
                    }
                }
                else if &ip[0..1] == "w"{
                    for value in String::from(&ip).split(","){
                        if i == 2 {
                            println!("Q: {}", value);
                        }
                        else if i == 1{                            
                            answer = String::from(value);
                        }
                        i += 1;
                    }
                }
                let mut t = String::new();
                std::io::stdin().read_line(&mut t).unwrap();
                if String::from(String::from(t).trim()) == answer.trim(){
                    println!("Correct!\n");
                }
                else{
                    println!("Incorrect it is {}\n", answer)
                }
                i = 0;
            }
        }
    }
}
