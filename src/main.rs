use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    
    dbg!(&args);
    
    let mut min : u32  = 0;
    
    let mut i = 3;

    if args.len() < 2 {
        return ();
    } 

    while i < args.len() {
        
        if args[i] == "--min" {
            min =  args[i + 1].parse().expect("min must be a integer");
            i += 1;
        
        }
        i += 1;
    }

}