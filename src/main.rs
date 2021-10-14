use std::process;
mod date;

fn main() {
    let bytes = include_bytes!("../asset/title.txt");
    println!("{}", String::from_utf8_lossy(bytes));
    let mut line = String::new();
    let date: date::Date;
    loop {
        line.clear();
        println!("Enter a date (YEAR-MONTH-DATE) or press Q to exit:");
        std::io::stdin().read_line(&mut line).unwrap();
        let trimmed = line.trim().to_lowercase();
        if trimmed == "q" {
            println!("goodbye!");
            process::exit(0);
        }
        match date::get_date(&trimmed) {
            Ok(v) => {
                date = v;
                break;
            }
            Err(e) => eprintln!("{}", e),
        };
    }
    //dbg!(&date);
    match date::calc_day(&date) {
        Ok(day) => println!(
            "{}. The day of the week is {}",
            date,
            date::DAYS[day as usize]
        ),
        Err(e) => eprintln!("{}", e),
    }
}
