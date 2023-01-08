#[macro_use]
extern crate magic_crypt;

use dirs::home_dir;
use magic_crypt::MagicCryptTrait;
use pancurses::{endwin, initscr};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::string::String;

enum Options {
    Add,
    Search,
    All,
}

impl Options {
    fn as_str(&self) -> &'static str {
        match self {
            Options::Add => "new",
            Options::Search => "search",
            Options::All => "all",
        }
    }
}

fn set_path() -> PathBuf {
    let home_path = home_dir().unwrap();
    let relative_path = PathBuf::from(".config/pwd");
    let path = home_path.join(relative_path);
    path
}

fn main() {
    run();
}

fn run() {
    let path = set_path();
    let dic: HashMap<String, String>;
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        display_options();
        return;
    }
    let query = args[1].as_str();

    match query {
        "new" => store_data(path.clone()),
        "search" => {
            let key = new_key();
            dic = file_to_hash(path);
            let passwd = dic.get(&key);
            match passwd {
                Some(pwd) => println!("{pwd}"),
                None => println!("Identifiant inconnu !"),
            }
        }
        "all" => {
            for (key, value) in file_to_hash(path) {
                println!("{} : {}", key, value);
            }
        }
        _ => display_options(),
    }
}

fn store_data(path: PathBuf) {
    let key: String = new_key();
    let repository: Vec<String> = read_file(path.clone());
    for data in repository {
        let stored_key = data.split("$").next().unwrap();
        if key == stored_key {
            println!("This identifier has a stored password");
            return;
        }
    }
    let passwd = new_passwd();
    let mut new_key_passwd = key.to_string() + &"$".to_string();
    new_key_passwd = new_key_passwd + &passwd.to_string();
    save_to_file(&path, new_key_passwd);
}

fn new_key() -> String {
    let mut key = String::new();
    println!(" enter site/computer name ");
    io::stdin()
        .read_line(&mut key)
        .expect("unable to read user input ");
    key.pop();
    key
}

fn new_passwd() -> String {
    let mut passwd = String::new();
    println!("enter password ");
    io::stdin()
        .read_line(&mut passwd)
        .expect("unable to read user input ");
    passwd
}

fn save_to_file(path: &PathBuf, new_data: String) {
    if path.exists() {
        let mut f = File::options()
            .append(true)
            .write(true)
            .read(true)
            .open(&path)
            .unwrap();
        f.write_all(new_data.as_bytes()).unwrap();
    } else {
        let f = File::create(&path);
        match f {
            Err(e) => println!("erreur  in else: {}", e),
            Ok(mut f) => f.write_all(new_data.as_bytes()).unwrap(),
        };
    }
}

fn read_file(path: PathBuf) -> Vec<String> {
    let mut output_string = String::new();
    let output_vector_of_string: Vec<String>;
    if path.exists() {
        let f = File::options().read(true).open(&path).unwrap();

        let mut buffer_reader = BufReader::new(f);
        buffer_reader
            .read_to_string(&mut output_string)
            .expect("unable to read file");
    } else {
        let _ = File::create(&path);
    }
    output_vector_of_string = output_string
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    output_vector_of_string
}

fn file_to_hash(path: PathBuf) -> HashMap<String, String> {
    let mut passwd_dic: HashMap<String, String> = HashMap::new();
    let f = File::open(&path).expect("error");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut iter = line.split("$");
                let key = &iter.next().unwrap();
                let value = &iter.next().unwrap();
                // println!("key = {} value = {}", key, value);
                passwd_dic.insert(key.to_string(), value.to_string());
            }

            Err(e) => println!("{}", e),
        }
    }
    passwd_dic
}

fn display_options() {
    let window = initscr();
    window.printw(Options::as_str(&Options::Add));
    window.printw("\n");
    window.printw(Options::as_str(&Options::Search));
    window.printw("\n");
    window.printw(Options::as_str(&Options::All));
    window.refresh();
    window.getch();
    endwin();
}

fn encrypt(line: &str, pwd: &str) -> String {
    let mc = new_magic_crypt!(pwd, 256);
    let base64 = mc.encrypt_str_to_base64(line);
    base64
}

fn decrypt(input: &str, pwd: &str) -> String {
    let mc = new_magic_crypt!(pwd, 256);
    let output = mc.decrypt_base64_to_string(input).unwrap();
    output
}
