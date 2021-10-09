use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:\n {}", err);
        process::exit(1);
    });

    run(config);
}

struct Config {
    date: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        //what if a user writes no input? or more than one input?
        if args.len() != 2 {
            return Err("this program requires only one argument, date\n format: YEAR-MONTH-DAY");
        }
        //check the validity of the input.
        let input = &args[1];
        for c in input.chars() {
            let year = c;
        }
        let date = args[1].clone();
        println!("parsed date: {}", date);
        Ok(Config { date })
    }
}

fn run(config: Config) {
    println!("running...on date: {}", config.date);
}