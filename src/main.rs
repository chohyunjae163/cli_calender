use std::env;
use std::fmt;
use std::process;

fn main() {
    println!("GREGORIAN CALENDER");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Config new error: {}", err);
        println!("this program requires one argument - format: YEAR-MONTH-DAY");
        process::exit(1);
    });

    println!("config date: {}", config.date);
    let date = config.date;
    date.year;
    date.month;
    date.day;

    let leap_count = get_leap_year_count(date.year);
    println!("leap year count: {}", leap_count);
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
            Err(e) => Err(e),
        }
    }
}

struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "INPUT DATE : {} {} {}", self.year, self.month, self.day)
    }
}

fn get_date(arg: &String) -> Result<Date, &'static str> {
    let mut strs: Vec<String> = vec![String::from(""), String::from(""), String::from("")];
    let mut index = 0;
    //expected string format year-month-day.
    for c in arg.chars() {
        if c == '-' {
            index += 1;
            continue;
        }
        strs[index].push(c);
    }
    let result = parse_date(&strs[0], &strs[1], &strs[2]);
    match result {
        Ok(date) => Ok(date),
        Err(e) => Err(e),
    }
}

fn parse_date(yy: &String, mm: &String, dd: &String) -> Result<Date, &'static str> {
    let year: u32;
    let month: u32;
    let day: u32;
    let result = yy.parse::<u32>();
    match result {
        Ok(v) => {
            if v < 1 {
                return Err("the year starts from 1 A.D.");
            }
            year = v
        }
        Err(_e) => return Err("failed parsing a year"),
    }
    let result = mm.parse::<u32>();
    match result {
        Ok(v) => {
            if v < 1 || v > 12 {
                return Err("a month should be within 1-12");
            }
            month = v
        }
        Err(_e) => return Err("failed parsing a month"),
    }
    let result = dd.parse::<u32>();
    match result {
        Ok(v) => {
            if v < 1 || v > 31 {
                return Err("day should be within 1-31");
            }
            if month == 2 {
                if v == 30 || v == 31 {
                    return Err("30 or 31 does not exist in Feb");
                }
                if v == 29 && is_leap_year(year) == false {
                    return Err("not a leap year");
                }
            }
            day = v
        }
        Err(_e) => return Err("failed parsing a day"),
    }

    Ok(Date { year, month, day })
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_leap_year_count(year: u32) -> u32 {
    //let months: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    //let months_leap_year: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    //let total = ((year - 1) * 365) + ((month - 1) * 30) + day;

    let count = (year / 4) - (year / 100);
    count
}
