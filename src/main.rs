use std::env;
use std::fs;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut min = 1;

    if args.len() < 3 {
        panic!("require and input folder and an output file")
    } else {
        min = args[3].parse().expect("nb error");
        println!("filtering line smaller than {} characters", min)
    }

    let output_file = File::create(&args[2]).expect("invadid output file");
    let mut output_file = LineWriter::new(output_file);

    for input_path in fs::read_dir(&args[1]).expect("a") {
        if let Ok(int) = input_path {
            let file = fs::File::open(int.path()).expect("error while opening file");
            let lines = io::BufReader::new(file).lines();

            for line in lines {
                if let Ok(line_) = line {
                    println!("{}", line_);
                    if min <= line_.len() {
                        output_file
                            .write_all((line_ + "\n").as_bytes())
                            .expect("error while writing file");
                    }
                }
            }
        }
    }

}