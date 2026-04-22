use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::io::Lines;

pub fn get_string(name: String) -> Lines<BufReader<File>>  {
    match File::open(name) {
        Ok(file) => BufReader::new(file).lines(),
        _ => panic!("Fin du game")
    }
}
