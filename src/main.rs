use std::env;
use std::fmt;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:\n {}", err);
        println!("this program requires one argument\n format: YEAR-MONTH-DAY");
        process::exit(1);
    });

    println!("{}", config.date)
}

struct Config {
    date: Date,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        //what if a user writes no input? or more than one input?
        if args.len() != 2 {
            return Err("the number of input parameter is wrong");
        }
        //check the validity of the input.
        let input = &args[1];
        let date = get_date(input);
        match date {
            Ok(v) => Ok(Config { date: v }),
            Err(e) => return Err(e),
        }
    }
}

struct Date {
    year: String,
    month: String,
    day: String,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INPUT DATE : {} {} {}", self.year, self.month, self.day)
    }
}

fn get_date(arg: &String) -> Result<Date, &'static str> {
    let mut date: Vec<String> = vec!(String::from(""),String::from(""),String::from(""));
    let mut index = 0;
    for c in arg.chars() {
        if c == '-' {
            index += 1;
            continue;
        } 
        date[index].push(c);
    }
    let year = date[0].clone();
    if is_numeric(year.parse::<i32>()) == false {
        return Err("the year input seems to be wrong....");
    }
    let month = date[1].clone();
    if is_numeric(month.parse::<i32>()) == false {
        return Err("the month input seems to be wrong....");
    }
    let day = date[2].clone();
    if is_numeric(day.parse::<i32>()) == false {
        return Err("the day input seems to be wrong....");
    }
    Ok(Date { year, month, day })
}

fn is_numeric(num: Result<i32, std::num::ParseIntError>) -> bool {
    match num {
        Ok(_val) => true,
        Err(_why) => false,
    }
}
