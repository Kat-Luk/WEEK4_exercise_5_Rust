use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::io;
use std::io::Write;

enum Message {
    Count(i32),
    TimesUp,
}
fn main() {
    println!("Do you want to start or exit?");
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    match choice.trim() {
        "start" => {},
        "exit" => {
            println!("$ Goodbye then.");
            std::process::exit(0);
        }
        _ => println!("Unexpected command"),
    }
    let args : Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Unexpected number of arguments! Use as follows:");
        println!("cargo run <letter> <integer>");
        return;
    }
    let string_arg = args[1].clone();
    let string_arg_thread = string_arg.clone();
    let int_arg:  u64 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid integer argument: {}", args[2]);
            return;
        }
    };

    let counter: i32 = 0;
    let counter = Arc::new(Mutex::new(counter));
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    let tx_timer = tx.clone();
    drop(tx);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(int_arg));
            tx_timer.send(Message::TimesUp).unwrap();
        }
    });

    let counter1 = Arc::clone(&counter);
    thread::spawn(move || {
        loop {
            io::stdout().flush().unwrap();
            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read input");
            if user_input.trim() == string_arg_thread {
                let mut c = counter1.lock().unwrap();
                *c += 1;
                tx1.send(Message::Count(*c)).unwrap();
            }
        }
    });

    let mut last_amount = 0;
    for msg in rx {
        match msg {
            Message::Count(n) => {
                println!("Presses: {}", n);
                last_amount = n;
            }
            Message::TimesUp => break,

        }
    }
    println!("You have managed to press '{}' {} times.", string_arg, last_amount);
}
