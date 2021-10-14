use std::fmt;

//the Gregorian calendar is adopted in 1582
const GREGORIAN_CALENDAR: u32 = 1582;
//Jan. 1st of the Gregorian calendar is Thursday.
const DAYS: [&'static str; 7] = [
    "THURSDAY",
    "FRIDAY",
    "SATURDAY",
    "SUNDAY",
    "MONDAY",
    "TUESDAY",
    "WEDNESDAY",
];

#[derive(Debug)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Date : {}-{}-{}", self.year, self.month, self.day)
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
    //dbg!(&strs[0], &strs[1], &strs[2]);
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
            if v < GREGORIAN_CALENDAR {
                return Err("The Gregorian calendar was introduced in 1582.");
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
    let leap_year = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    //dbg!(leap_year);
    leap_year
}

fn get_leap_year_count(year: u32) -> u32 {
    let pre_gregorian_leaps =
        (GREGORIAN_CALENDAR / 4) - (GREGORIAN_CALENDAR / 100) + (GREGORIAN_CALENDAR / 400);

    let leap_year_count = (year / 4) - (year / 100) + (year / 400) - pre_gregorian_leaps;
    //dbg!(leap_year_count);
    leap_year_count
}

fn calc_day(date: &Date) -> u32 {
    const YEAR: u32 = 365;
    let mut days = 0;
    if date.month > 1 {
        const MONTHS: [u32; 11] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        const MONTHS_LEAP_YEAR: [u32; 11] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30];
        let month_minus_one = (date.month - 1) as usize;
        if is_leap_year(date.year) {
            for d in &MONTHS_LEAP_YEAR[0..month_minus_one] {
                days += d;
            }
        } else {
            for d in &MONTHS[0..month_minus_one] {
                days += d;
            }
        }
    }
    let leap_year_count = get_leap_year_count(date.year);
    days += (date.year - GREGORIAN_CALENDAR) * YEAR + date.day + leap_year_count;
    days % 7
}

fn print_title() {
    let bytes = include_bytes!("../asset/title.txt");
    println!("{}", String::from_utf8_lossy(bytes));
}

pub fn run() {
    print_title();
    let mut line = String::new();
    let date: Date;
    loop {
        line.clear();
        println!("Enter a date (YEAR-MONTH-DATE) or press Q to exit:");
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim().to_lowercase();
        if trimmed == "q" {
            println!("goodbye!");
            return;
        }
        match get_date(&trimmed) {
            Ok(v) => {
                date = v;
                break;
            }
            Err(e) => eprintln!("{}", e),
        };
    }
    //dbg!(&date);
    let day = calc_day(&date);
    println!("{}. The day of the week is {}", date, DAYS[day as usize]);
}
