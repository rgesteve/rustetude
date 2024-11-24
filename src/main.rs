use std::env;
use std::process::Command;
use std::time::{Instant, SystemTime};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    println!("The value of HOME is: {:?}.", get_env_var(String::from("HOME"))); // or "None" if key doesn't exist

    let instant = Instant::now();
    let system_time = SystemTime::now();
    println!("Hello, world, instant: {instant:?}");
    println!("Hello, world, system_time: {system_time:?}");
    
    print!("Hello, world, calling a function\n");
    try_references();
    gen_seq_list();
    gen_seq_list_imperative();
    print!("Hello, world, for loop\n");
    let index = 0;
    for index in 0..4 { print!("{} ", index); }
    print!(":{}", index); // index as an index variables shadows "index" on outer scope

    println!("Running `ls`");
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
    println!("`ls` seems to have run correctly");

    print!("\nDone!\n");
    std::process::exit(0);
}

fn gen_seq_list() {
    for i in 0..12 { println!("{}", i); }
}
/* 
use std::iter::range;

fn gen_seq_list() {
    let numbers: Vec<u32> = range(1, 11).collect();
    println!("{:?}", numbers); // Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
}
*/

fn gen_seq_list_imperative() {
    let mut newvec = Vec::new();
    let mut counter = 1;
    loop {
        newvec.push(counter);
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    println!("{newvec:?}");
}

// Map `Result` to `Option`
fn get_env_var(key : String) -> Option<String> {
    match env::var(key) {
        Ok(value) => Some(value),
        Err(_) => None
    }
}

fn try_references() {
    let a = 42;
    let r = &a;
    let b = a + *r;
    println!("a + a = {}.", b);
}

#[test]
fn multiply_test() {
    assert_eq!(6, 2*3);
}