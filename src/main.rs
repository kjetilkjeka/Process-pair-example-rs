use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::time::{SystemTime, Duration};
use std::thread;
use std::str::*;
use std::process::Command;
use std::env::args;

const FILEPATH: &'static str = "checkpoint";
const TIMEOUT_MS: u64 = 3000;
const PERIOD_MS: u64 = 1000;

fn main() {
    println!("I'm backup");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILEPATH)
        .unwrap();
    
    while
    {SystemTime::now().duration_since(file.metadata().unwrap().modified().unwrap()).unwrap() <= Duration::from_millis(TIMEOUT_MS)}
    {}
    
    println!("Can't find primary");

    let mut file_source = String::new();
    println!("The source is: \"{}\"", file_source);
    let mut number = match file.read_to_string(&mut file_source).unwrap() {
        0 => 0,
        _ => i32::from_str(file_source.as_str()).unwrap(),
    };

    println!("Spawning the backup");
    let backup_spawning_command = Command::new("gnome-terminal")
        .arg("-x")
        .arg(args().nth(0).unwrap())
        .spawn();
    
    println!("I'm the primary now");
    
     // Become master
     loop{
         number = number + 1;
         file.set_len(0);
         file.seek(io::SeekFrom::Start(0));
         file.write_fmt(format_args!("{}", number));
         println!("The number is: {}", number);
         thread::sleep(Duration::from_millis(PERIOD_MS));
     }
     
}
