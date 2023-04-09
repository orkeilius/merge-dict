use std::env;

use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::io::{self, BufRead};
use std::io::Error;

const VERSION :&str = "0.1";




fn merge() -> Result<bool, Error> {
    let args: Vec<String> = env::args().collect();

    let mut min = 1;

    if args.len() < 3 {
        println!("error: missing argument");
        return Ok(false);
        
        
    }
    else if args.len() > 3 {
        let result = args[3].parse();
        if result.is_err(){
            println!("error : filter is not an number");
            return Ok(false);
        }
        min = result.unwrap();
        println!("filtering line smaller than {} characters", min)
    }
    println!("creating output file");
    let output_file = File::create(&args[2])?;
    let mut output_file = LineWriter::new(output_file);

    println!("scanning input folder");
    for input_path in fs::read_dir(&args[1])? {

        if let Ok(int) = input_path {
            println!("opening {:?}",int.file_name());
            let file = fs::File::open(int.path())?;
            let lines = io::BufReader::new(file).lines();
            let mut line_error = 0;

            for line in lines {
                if let Ok(line_) = line {
                    if min <= line_.len() {
                        output_file
                            .write_all((line_ + "\n").as_bytes())?;
                    }
                }
                else {
                    line_error += 1;
                }
            }
            if line_error != 0{
                println!("{} error while reading a line",line_error)
            }

        }
    }
    print!("done");
    return Ok(true);
}

fn main() {
    let result = merge();

    match result {
        Ok(r) => { 
            if r {
                return;
            }
        },
        Err(e) => {
            println!("{}",e)
        },
    }

    println!("");
    println!("merge-dict - a tools to merge dictionary version {}", VERSION );
    println!("use : merge-dict <input folder> <output file> [filter]  ");
    println!("");
    println!("input folder : path to folder containing file to merge");
    println!("ouput file   : output file");
    println!("filter       : filter word smaller that inputed number (default 1)")



}
