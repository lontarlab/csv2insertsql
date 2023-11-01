use std::error::Error;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1] == "help" {
        print_usage();
        return;
    } else if args.len() != 3 {
        println!("Invalid arguments!");
        print_usage();
        return;
    }

    let directory = &args[1];
    let table_name = &args[2];

    match read_csv(directory, table_name) {
        Ok(_) => (),
        Err(err) => panic!("{}", err),
    }
}

fn print_usage() {
    println!("Usage: csv2insertsql file.csv table_name");
    println!("       csv2insertsql help");
}

fn read_csv(directory: &String, table_name: &String) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(directory)?;
    let mut rdr = csv::Reader::from_reader(content.as_bytes());

    let column_string: String = rdr
        .headers()?
        .into_iter()
        .map(|header| format!("\"{}\"", header))
        .collect::<Vec<String>>()
        .join(", ");

    let mut queries: Vec<String> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let values_string = record
            .iter()
            .map(|e| {
                if e.is_empty() {
                    "NULL".to_owned()
                } else {
                    match e.parse::<i64>() {
                        Ok(num) => format!("{}", num),
                        Err(_) => {
                            format!("'{}'", e)
                        }
                    }
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        let query = format!(
            "INSERT INTO \"{}\"({}) VALUES({});",
            table_name, column_string, values_string
        );
        queries.push(query);
    }

    for query in queries {
        println!("{}", query);
    }
    Ok(())
}
