use colorama::Colored;
use std::io::{BufReader, BufRead};
use std::fs::File;

mod python;
mod rust;

pub fn header(file_name: &str) {
    let n_cols: usize = termsize::get().unwrap().cols.into();
    println!("─────┬{}", "─".repeat(n_cols - 6));
    println!("     │  File {}", file_name.to_string().style("bold"));
    println!("─────┼{}", "─".repeat(n_cols - 6));
}

pub fn contents_pretty(file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        print!(" {: ^width$} │  ", i+1, width=3);
        let file_extension = file_name.split('.').last().unwrap();
        let keywords = match file_extension {
            "py" => Some(python::keywords()),
            "rs" => Some(rust::keywords()), 
            _ => None,
        };
        
        let line = line.unwrap();
        let splits = line.split(' ');
        for word in splits {
            match keywords.clone() {
                Some(val) => {
                    if val.contains(&word) {
                        print!("{} ", word.to_string().color("green"));
                    } else {
                        print!("{} ", word);
                    }
                },
                None => print!("{} ", word),
            };
        }
        println!("");
    }
}

pub fn contents(file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

pub fn footer() {
    let n_cols: usize = termsize::get().unwrap().cols.into();
    println!("─────┴{}", "─".repeat(n_cols - 6));
}
